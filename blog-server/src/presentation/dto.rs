//! DTO HTTP-слоя серверного приложения.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::auth_service::AuthSession;
use crate::domain::user::User;

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

/// Ответ с данными пользователя.
#[derive(Debug, Serialize)]
pub struct UserResponse {
    /// Идентификатор пользователя.
    pub id: Uuid,
    /// Имя пользователя.
    pub username: String,
    /// Электронная почта пользователя.
    pub email: String,
    /// Время создания пользователя.
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
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
