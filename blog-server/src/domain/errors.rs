//! Ошибки доменного слоя.

use thiserror::Error;

/// Ошибка доменного слоя блога.
#[derive(Debug, Error)]
pub enum DomainError {
    /// Доменная валидация не пройдена.
    #[error("validation error: {0}")]
    Validation(String),

    /// Внутренняя ошибка доменного слоя.
    #[error("internal server error")]
    Internal(String),

    /// Действие запрещено для текущего пользователя.
    #[error("you do not have permission to perform this action")]
    Forbidden,

    /// Пользователь не аутентифицирован.
    #[error("authentication is required to access this resource")]
    Unauthorized,

    /// Пользователь не найден.
    #[error("user not found: {0}")]
    UserNotFound(String),

    /// Пользователь уже существует.
    #[error("user already exists: {0}")]
    UserAlreadyExists(String),

    /// Переданы неверные учетные данные.
    #[error("username or password is incorrect")]
    InvalidCredentials,

    /// Пост не найден.
    #[error("post not found: {0}")]
    PostNotFound(u64),
}
