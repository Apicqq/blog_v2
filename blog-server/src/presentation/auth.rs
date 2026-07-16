//! Аутентифицированный пользователь HTTP-запроса.

use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest};
use futures_util::future::{Ready, ready};
use uuid::Uuid;

/// Пользователь, извлеченный из JWT-токена.
#[derive(Debug, Clone, Copy)]
pub struct AuthenticatedUser {
    /// Идентификатор пользователя.
    pub user_id: Uuid,
}

impl AuthenticatedUser {
    /// Создает данные аутентифицированного пользователя.
    #[must_use]
    pub const fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(user) = req.extensions().get::<AuthenticatedUser>() {
            return ready(Ok(*user));
        }
        ready(Err(ErrorUnauthorized("missing authenticated user")))
    }
}
