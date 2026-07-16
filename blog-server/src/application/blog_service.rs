//! Сценарии работы с постами блога.

use std::sync::Arc;

use uuid::Uuid;

use crate::application::ports::post_repository::PostRepository;
use crate::domain::errors::DomainError;
use crate::domain::post::{Post, PostAttributes, UpdatePost};

/// Сервис сценариев блога.
#[derive(Debug, Clone)]
pub struct BlogService<R> {
    repo: Arc<R>,
}

impl<R> BlogService<R>
where
    R: PostRepository,
{
    /// Создает сервис сценариев блога.
    #[must_use]
    pub const fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    /// Создает новый пост.
    ///
    /// # Errors
    ///
    /// Возвращает доменную ошибку, если пост не может быть сохранен.
    pub async fn create_post(
        &self,
        author_id: Uuid,
        title: String,
        content: String,
    ) -> Result<Post, DomainError> {
        self.repo
            .create(PostAttributes::new(title, content, author_id))
            .await
    }

    /// Возвращает пост по идентификатору.
    ///
    /// # Errors
    ///
    /// Возвращает `PostNotFound`, если пост не найден.
    pub async fn get_post(&self, post_id: i64) -> Result<Post, DomainError> {
        self.repo
            .find_by_id(post_id)
            .await?
            .ok_or(DomainError::PostNotFound(post_id))
    }

    /// Обновляет пост, если пользователь является автором.
    ///
    /// # Errors
    ///
    /// Возвращает `PostNotFound`, если пост не найден, и `Forbidden`, если пользователь не автор.
    pub async fn update_post(
        &self,
        user_id: Uuid,
        post_id: i64,
        update: UpdatePost,
    ) -> Result<Post, DomainError> {
        let mut post = self.get_post(post_id).await?;

        ensure_author(&post, user_id)?;

        post.update(update);

        self.repo.update(post).await
    }

    /// Удаляет пост, если пользователь является автором.
    ///
    /// # Errors
    ///
    /// Возвращает `PostNotFound`, если пост не найден, и `Forbidden`, если пользователь не автор.
    pub async fn delete_post(&self, user_id: Uuid, post_id: i64) -> Result<(), DomainError> {
        let post = self.get_post(post_id).await?;

        ensure_author(&post, user_id)?;

        self.repo.delete(post_id).await
    }

    /// Возвращает страницу постов.
    ///
    /// # Errors
    ///
    /// Возвращает доменную ошибку, если хранилище недоступно.
    pub async fn list_posts(&self, limit: u64, offset: u64) -> Result<Vec<Post>, DomainError> {
        self.repo.list(limit, offset).await
    }
}

fn ensure_author(post: &Post, user_id: Uuid) -> Result<(), DomainError> {
    if post.is_author(user_id) {
        return Ok(());
    }

    Err(DomainError::Forbidden)
}
