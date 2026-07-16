//! Контракт хранилища пользователей.

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::DomainError;
use crate::domain::user::User;

/// Хранилище пользователей для прикладных сценариев.
#[async_trait]
pub trait UserRepository {
    /// Создает пользователя.
    ///
    /// # Errors
    ///
    /// Возвращает доменную ошибку, если пользователь не может быть сохранен.
    async fn create(&self, user: User) -> Result<User, DomainError>;

    /// Ищет пользователя по идентификатору.
    ///
    /// # Errors
    ///
    /// Возвращает доменную ошибку, если хранилище недоступно.
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;

    /// Ищет пользователя по имени.
    ///
    /// # Errors
    ///
    /// Возвращает доменную ошибку, если хранилище недоступно.
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError>;

    /// Проверяет существование пользователя с указанным именем.
    ///
    /// # Errors
    ///
    /// Возвращает доменную ошибку, если хранилище недоступно.
    async fn exists_by_username(&self, username: &str) -> Result<bool, DomainError>;

    /// Проверяет существование пользователя с указанной электронной почтой.
    ///
    /// # Errors
    ///
    /// Возвращает доменную ошибку, если хранилище недоступно.
    async fn exists_by_email(&self, email: &str) -> Result<bool, DomainError>;
}
