//! HTTP-handlers аутентификации.

use crate::application::auth_service::AuthService;
use crate::domain::errors::DomainError;
use crate::infrastructure::persistence::repositories::sea_orm_user_repository::SeaOrmUserRepository;
use crate::infrastructure::security::argon2_password_hasher::Argon2PasswordHasher;
use crate::infrastructure::security::jwt_token_service::JwtTokenService;
use crate::presentation::dto::auth::{AuthResponse, LoginRequest, RegisterRequest};
use actix_web::{HttpResponse, post, web};
use validator::Validate;

type BlogAuthService = AuthService<SeaOrmUserRepository, Argon2PasswordHasher, JwtTokenService>;

/// Настраивает маршруты аутентификации.
pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(register_handler)
            .service(login_handler),
    );
}

/// Регистрирует нового пользователя.
#[post("/register")]
async fn register_handler(
    service: web::Data<BlogAuthService>,
    payload: web::Json<RegisterRequest>,
) -> Result<HttpResponse, DomainError> {
    let payload = payload.into_inner();
    payload
        .validate()
        .map_err(|err| DomainError::Validation(err.to_string()))?;
    let registration = payload.into();
    let session = service.register(registration).await?;

    Ok(HttpResponse::Created().json(AuthResponse::from(session)))
}

/// Выполняет вход пользователя.
#[post("/login")]
async fn login_handler(
    service: web::Data<BlogAuthService>,
    payload: web::Json<LoginRequest>,
) -> Result<HttpResponse, DomainError> {
    let payload = payload.into_inner();
    payload
        .validate()
        .map_err(|err| DomainError::Validation(err.to_string()))?;
    let credentials = payload.into();
    let session = service.login(credentials).await?;

    Ok(HttpResponse::Ok().json(AuthResponse::from(session)))
}
