//! HTTP-handlers аутентификации.

use actix_web::{HttpResponse, post, web};

use crate::application::auth_service::AuthService;
use crate::domain::errors::DomainError;
use crate::domain::user::{LoginCredentials, RegistrationData};
use crate::infrastructure::persistence::repositories::sea_orm_user_repository::SeaOrmUserRepository;
use crate::infrastructure::security::argon2_password_hasher::Argon2PasswordHasher;
use crate::infrastructure::security::jwt_token_service::JwtTokenService;
use crate::presentation::dto::{AuthResponse, LoginRequest, RegisterRequest};

type BlogAuthService = AuthService<SeaOrmUserRepository, Argon2PasswordHasher, JwtTokenService>;

/// Настраивает маршруты аутентификации.
pub fn configure_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(register).service(login));
}

/// Регистрирует нового пользователя.
#[post("/register")]
async fn register(
    service: web::Data<BlogAuthService>,
    payload: web::Json<RegisterRequest>,
) -> Result<HttpResponse, DomainError> {
    let payload = payload.into_inner();
    let registration = RegistrationData::new(payload.username, payload.email, payload.password);
    let session = service.register(registration).await?;

    Ok(HttpResponse::Created().json(AuthResponse::from(session)))
}

/// Выполняет вход пользователя.
#[post("/login")]
async fn login(
    service: web::Data<BlogAuthService>,
    payload: web::Json<LoginRequest>,
) -> Result<HttpResponse, DomainError> {
    let payload = payload.into_inner();
    let credentials = LoginCredentials::new(payload.username, payload.password);
    let session = service.login(credentials).await?;

    Ok(HttpResponse::Ok().json(AuthResponse::from(session)))
}
