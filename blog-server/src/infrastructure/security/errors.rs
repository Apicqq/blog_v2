//! Преобразование ошибок адаптеров безопасности.

use argon2::password_hash::Error as PasswordHashError;

use crate::domain::errors::DomainError;

impl From<PasswordHashError> for DomainError {
    fn from(err: PasswordHashError) -> Self {
        Self::Internal(err.to_string())
    }
}
