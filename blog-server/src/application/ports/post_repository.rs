//! Контракт хранилища постов.

use async_trait::async_trait;

use crate::domain::errors::DomainError;
use crate::domain::post::{Post, PostAttributes};

/// Хранилище постов для прикладных сценариев.
#[async_trait]
pub trait PostRepository {
    /// Создает пост из существенных данных.
    ///
    /// # Errors
    ///
    /// Возвращает доменную ошибку, если пост не может быть сохранен.
    async fn create(&self, attributes: PostAttributes) -> Result<Post, DomainError>;

    /// Ищет пост по идентификатору.
    ///
    /// # Errors
    ///
    /// Возвращает доменную ошибку, если хранилище недоступно.
    async fn find_by_id(&self, id: i64) -> Result<Option<Post>, DomainError>;

    /// Обновляет пост.
    ///
    /// # Errors
    ///
    /// Возвращает доменную ошибку, если пост не может быть обновлен.
    async fn update(&self, post: Post) -> Result<Post, DomainError>;

    /// Удаляет пост по идентификатору.
    ///
    /// # Errors
    ///
    /// Возвращает доменную ошибку, если пост не может быть удален.
    async fn delete(&self, id: i64) -> Result<(), DomainError>;

    /// Возвращает страницу постов.
    ///
    /// # Errors
    ///
    /// Возвращает доменную ошибку, если хранилище недоступно.
    async fn list(&self, limit: u64, offset: u64) -> Result<Vec<Post>, DomainError>;
}
