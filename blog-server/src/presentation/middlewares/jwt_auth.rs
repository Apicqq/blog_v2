//! Middleware JWT-аутентификации.

use futures_util::future::{LocalBoxFuture, Ready, ready};

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header::AUTHORIZATION;
use actix_web::{Error, HttpMessage, error};

use crate::application::ports::token_service::TokenService;
use crate::presentation::auth::AuthenticatedUser;

/// Middleware проверки JWT-токена.
#[derive(Debug, Clone)]
pub struct JwtAuthMiddleware<T> {
    token_service: T,
}

impl<T> JwtAuthMiddleware<T> {
    /// Создает middleware JWT-аутентификации.
    #[must_use]
    pub const fn new(token_service: T) -> Self {
        Self { token_service }
    }
}

impl<S, B, T> Transform<S, ServiceRequest> for JwtAuthMiddleware<T>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    T: TokenService + Clone + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtAuthService<S, T>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthService {
            service,
            token_service: self.token_service.clone(),
        }))
    }
}

/// Сервис JWT-аутентификации для конкретного Actix service.
#[derive(Debug)]
pub struct JwtAuthService<S, T> {
    service: S,
    token_service: T,
}

impl<S, B, T> Service<ServiceRequest> for JwtAuthService<S, T>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    T: TokenService + Clone + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let user_id = extract_token(
            req.headers()
                .get(AUTHORIZATION)
                .and_then(|value| value.to_str().ok()),
        )
        .and_then(|token| {
            self.token_service
                .verify(token)
                .map_err(|_| error::ErrorUnauthorized("invalid authorization token"))
        });

        match user_id {
            Ok(user_id) => {
                req.extensions_mut().insert(AuthenticatedUser::new(user_id));
                let future = self.service.call(req);

                Box::pin(future)
            }
            Err(err) => Box::pin(async { Err(err) }),
        }
    }
}

fn extract_token(header: Option<&str>) -> Result<&str, Error> {
    let header = header.ok_or_else(|| error::ErrorUnauthorized("missing authorization header"))?;

    header
        .strip_prefix("Bearer ")
        .filter(|token| !token.is_empty())
        .ok_or_else(|| error::ErrorUnauthorized("invalid authorization header"))
}
