//! Контракт создания и проверки токенов.

use uuid::Uuid;

use crate::domain::errors::DomainError;

/// Сервис выпуска и проверки пользовательских токенов.
pub trait TokenService {
    /// Выпускает новый токен для пользователя.
    ///
    /// # Errors
    ///
    /// Возвращает доменную ошибку, если токен не удалось создать.
    fn issue_new(&self, user_id: Uuid) -> Result<String, DomainError>;

    /// Проверяет токен и возвращает идентификатор пользователя.
    ///
    /// # Errors
    ///
    /// Возвращает доменную ошибку, если токен некорректен или истек.
    fn verify(&self, token: &str) -> Result<Uuid, DomainError>;
}
