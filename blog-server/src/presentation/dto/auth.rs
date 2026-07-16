//! DTO аутентификации.

use crate::application::auth_service::AuthSession;
use crate::domain::user::{LoginCredentials, RegistrationData, User};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Запрос регистрации пользователя.
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    /// Имя пользователя.
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    /// Электронная почта пользователя.
    #[validate(email, length(max = 128))]
    pub email: String,
    /// Пароль пользователя.
    #[validate(length(min = 8, max = 128))]
    pub password: String,
}

/// Запрос входа пользователя.
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    /// Имя пользователя.
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    /// Пароль пользователя.
    #[validate(length(min = 8, max = 128))]
    pub password: String,
}

impl From<RegisterRequest> for RegistrationData {
    fn from(request: RegisterRequest) -> Self {
        Self::new(
            request.username.trim().to_string(),
            request.email.trim().to_lowercase(),
            request.password,
        )
    }
}

impl From<LoginRequest> for LoginCredentials {
    fn from(request: LoginRequest) -> Self {
        Self::new(request.username.trim().to_string(), request.password)
    }
}

/// Ответ с данными пользователя.
#[derive(Debug, Serialize)]
pub struct UserResponse {
    /// Имя пользователя.
    pub username: String,
    /// Электронная почта пользователя.
    pub email: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            username: user.username,
            email: user.email,
        }
    }
}

/// Ответ успешной аутентификации.
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    /// JWT-токен пользователя.
    pub token: String,
    /// Данные пользователя.
    pub user: UserResponse,
}

impl From<AuthSession> for AuthResponse {
    fn from(session: AuthSession) -> Self {
        Self {
            token: session.token,
            user: UserResponse::from(session.user),
        }
    }
}
