//! DTO аутентификации.

use crate::application::auth_service::AuthSession;
use crate::domain::errors::DomainError;
use crate::domain::user::{LoginCredentials, RegistrationData, User};
use serde::{Deserialize, Serialize};

/// Запрос регистрации пользователя.
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    /// Имя пользователя.
    pub username: String,
    /// Электронная почта пользователя.
    pub email: String,
    /// Пароль пользователя.
    pub password: String,
}

/// Запрос входа пользователя.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// Имя пользователя.
    pub username: String,
    /// Пароль пользователя.
    pub password: String,
}

impl TryFrom<RegisterRequest> for RegistrationData {
    type Error = DomainError;

    fn try_from(request: RegisterRequest) -> Result<Self, Self::Error> {
        Self::new(&request.username, &request.email, request.password)
    }
}

impl TryFrom<LoginRequest> for LoginCredentials {
    type Error = DomainError;

    fn try_from(request: LoginRequest) -> Result<Self, Self::Error> {
        Self::new(&request.username, request.password)
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
