//! Контракт хеширования паролей.

use crate::domain::errors::DomainError;

/// Хешер и проверяющий паролей.
pub trait PasswordHasher {
    /// Создает хеш пароля.
    ///
    /// # Errors
    ///
    /// Возвращает доменную ошибку, если пароль не удалось захешировать.
    fn hash_password(&self, password: &str) -> Result<String, DomainError>;

    /// Проверяет пароль по сохраненному хешу.
    ///
    /// # Errors
    ///
    /// Возвращает доменную ошибку, если сохраненный хеш имеет некорректный формат.
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, DomainError>;
}
