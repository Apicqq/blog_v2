//! Protected/debug handlers.

use actix_web::{HttpResponse, get, web};

use crate::application::auth_service::AuthService;
use crate::domain::errors::DomainError;
use crate::infrastructure::persistence::repositories::sea_orm_user_repository::SeaOrmUserRepository;
use crate::infrastructure::security::argon2_password_hasher::Argon2PasswordHasher;
use crate::infrastructure::security::jwt_token_service::JwtTokenService;
use crate::presentation::auth::AuthenticatedUser;
use crate::presentation::dto::auth::UserResponse;

type BlogAuthService = AuthService<SeaOrmUserRepository, Argon2PasswordHasher, JwtTokenService>;

/// Настраивает protected routes для сервиса текущего пользователя.
pub fn configure_protected_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(current_user);
}

/// Возвращает пользователя, связанного с текущим JWT-токеном.
#[get("/me")]
async fn current_user(
    user: AuthenticatedUser,
    service: web::Data<BlogAuthService>,
) -> Result<HttpResponse, DomainError> {
    let user = service.current_user(user.user_id).await?;

    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::http::StatusCode;
    use actix_web::{App, test, web};
    use chrono::Duration;

    use crate::infrastructure::security::jwt_token_service::JwtTokenService;
    use crate::presentation::middlewares::jwt_auth::jwt_validator;
    use actix_web_httpauth::middleware::HttpAuthentication;

    const JWT_SECRET: &str = "test-secret";

    #[actix_web::test]
    async fn current_user_requires_authorization_header() {
        let token_service = JwtTokenService::new(JWT_SECRET.to_string(), Duration::minutes(15));
        let app = test::init_service(
            App::new().app_data(web::Data::new(token_service)).service(
                web::scope("")
                    .wrap(HttpAuthentication::bearer(jwt_validator))
                    .configure(configure_protected_routes),
            ),
        )
        .await;

        let request = test::TestRequest::get().uri("/me").to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
