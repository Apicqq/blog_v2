//! Сценарии работы с постами блога.

use std::sync::Arc;

use tracing::{debug, info, instrument, warn};
use uuid::Uuid;

use crate::application::ports::post_repository::PostRepository;
use crate::domain::errors::DomainError;
use crate::domain::post::{Post, PostAttributes, UpdatePost};

/// Страница постов.
#[derive(Debug)]
pub struct PostPage {
    /// Посты текущей страницы.
    pub posts: Vec<Post>,
    /// Общее количество постов.
    pub total: u64,
}

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
    #[instrument(skip(self, title, content), fields(author_id = %author_id))]
    pub async fn create_post(
        &self,
        author_id: Uuid,
        title: String,
        content: String,
    ) -> Result<Post, DomainError> {
        let attributes = PostAttributes::new(&title, content, author_id)?;
        let post = self.repo.create(attributes).await?;

        info!(post_id = post.id, author_id = %post.author_id, "post created");

        Ok(post)
    }

    /// Возвращает пост по идентификатору.
    ///
    /// # Errors
    ///
    /// Возвращает `PostNotFound`, если пост не найден.
    #[instrument(skip(self), fields(post_id = post_id))]
    pub async fn get_post(&self, post_id: i64) -> Result<Post, DomainError> {
        let post = self
            .repo
            .find_by_id(post_id)
            .await?
            .ok_or(DomainError::PostNotFound(post_id))?;

        debug!(post_id = post.id, author_id = %post.author_id, "post loaded");

        Ok(post)
    }

    /// Обновляет пост, если пользователь является автором.
    ///
    /// # Errors
    ///
    /// Возвращает `PostNotFound`, если пост не найден, и `Forbidden`, если пользователь не автор.
    #[instrument(skip(self, update), fields(user_id = %user_id, post_id = post_id))]
    pub async fn update_post(
        &self,
        user_id: Uuid,
        post_id: i64,
        update: UpdatePost,
    ) -> Result<Post, DomainError> {
        let mut post = self.get_post(post_id).await?;

        ensure_author(&post, user_id)?;

        post.update(update);

        let post = self.repo.update(post).await?;
        info!(post_id = post.id, user_id = %user_id, "post updated");

        Ok(post)
    }

    /// Удаляет пост, если пользователь является автором.
    ///
    /// # Errors
    ///
    /// Возвращает `PostNotFound`, если пост не найден, и `Forbidden`, если пользователь не автор.
    #[instrument(skip(self), fields(user_id = %user_id, post_id = post_id))]
    pub async fn delete_post(&self, user_id: Uuid, post_id: i64) -> Result<(), DomainError> {
        let post = self.get_post(post_id).await?;

        ensure_author(&post, user_id)?;

        self.repo.delete(post_id).await?;
        info!(post_id = post_id, user_id = %user_id, "post deleted");

        Ok(())
    }

    /// Возвращает страницу постов.
    ///
    /// # Errors
    ///
    /// Возвращает доменную ошибку, если хранилище недоступно.
    #[instrument(skip(self), fields(limit = limit, offset = offset))]
    pub async fn list_posts(&self, limit: u64, offset: u64) -> Result<PostPage, DomainError> {
        let posts = self.repo.list(limit, offset).await?;
        let total = self.repo.count().await?;

        debug!(
            returned = posts.len(),
            total = total,
            limit = limit,
            offset = offset,
            "posts listed"
        );

        Ok(PostPage { posts, total })
    }
}

fn ensure_author(post: &Post, user_id: Uuid) -> Result<(), DomainError> {
    if post.is_author(user_id) {
        return Ok(());
    }

    warn!(post_id = post.id, author_id = %post.author_id, user_id = %user_id, "post access denied");

    Err(DomainError::Forbidden)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Mutex;
    use std::sync::atomic::{AtomicI64, Ordering};

    use async_trait::async_trait;
    use chrono::Utc;

    #[derive(Debug)]
    struct TestPostRepository {
        posts: Mutex<Vec<Post>>,
        next_id: AtomicI64,
    }

    impl Default for TestPostRepository {
        fn default() -> Self {
            Self {
                posts: Mutex::new(Vec::new()),
                next_id: AtomicI64::new(1),
            }
        }
    }

    impl TestPostRepository {
        fn with_posts(posts: Vec<Post>) -> Self {
            let next_id = posts.iter().map(|post| post.id).max().unwrap_or(0) + 1;

            Self {
                posts: Mutex::new(posts),
                next_id: AtomicI64::new(next_id),
            }
        }
    }

    #[async_trait]
    impl PostRepository for TestPostRepository {
        async fn create(&self, attributes: PostAttributes) -> Result<Post, DomainError> {
            let id = self.next_id.fetch_add(1, Ordering::SeqCst);
            let post = Post::from_attributes(id, attributes);
            self.posts
                .lock()
                .expect("posts mutex should not be poisoned")
                .push(clone_post(&post));

            Ok(post)
        }

        async fn find_by_id(&self, id: i64) -> Result<Option<Post>, DomainError> {
            let post = self
                .posts
                .lock()
                .expect("posts mutex should not be poisoned")
                .iter()
                .find(|post| post.id == id)
                .map(clone_post);

            Ok(post)
        }

        async fn update(&self, post: Post) -> Result<Post, DomainError> {
            let mut posts = self
                .posts
                .lock()
                .expect("posts mutex should not be poisoned");
            let Some(existing_post) = posts.iter_mut().find(|existing| existing.id == post.id)
            else {
                return Err(DomainError::PostNotFound(post.id));
            };

            *existing_post = clone_post(&post);

            Ok(post)
        }

        async fn delete(&self, id: i64) -> Result<(), DomainError> {
            let mut posts = self
                .posts
                .lock()
                .expect("posts mutex should not be poisoned");
            let previous_len = posts.len();
            posts.retain(|post| post.id != id);

            if posts.len() == previous_len {
                return Err(DomainError::PostNotFound(id));
            }

            Ok(())
        }

        async fn list(&self, limit: u64, offset: u64) -> Result<Vec<Post>, DomainError> {
            let offset = usize::try_from(offset).unwrap_or(usize::MAX);
            let limit = usize::try_from(limit).unwrap_or(usize::MAX);
            let posts = self
                .posts
                .lock()
                .expect("posts mutex should not be poisoned")
                .iter()
                .skip(offset)
                .take(limit)
                .map(clone_post)
                .collect();

            Ok(posts)
        }

        async fn count(&self) -> Result<u64, DomainError> {
            let count = self
                .posts
                .lock()
                .expect("posts mutex should not be poisoned")
                .len();

            Ok(u64::try_from(count).expect("posts count should fit into u64"))
        }
    }

    #[actix_web::test]
    async fn create_post_persists_author_and_content() {
        let author_id = Uuid::new_v4();
        let repository = Arc::new(TestPostRepository::default());
        let service = BlogService::new(Arc::clone(&repository));

        let post = service
            .create_post(author_id, "Заголовок".to_string(), "Содержимое".to_string())
            .await
            .expect("post should be created");

        assert_eq!(post.id, 1);
        assert_eq!(post.title, "Заголовок");
        assert_eq!(post.content, "Содержимое");
        assert_eq!(post.author_id, author_id);
        assert_eq!(repository.count().await.expect("count should work"), 1);
    }

    #[actix_web::test]
    async fn create_post_rejects_invalid_title() {
        let service = BlogService::new(Arc::new(TestPostRepository::default()));

        let error = service
            .create_post(
                Uuid::new_v4(),
                "  a  ".to_string(),
                "Содержимое".to_string(),
            )
            .await
            .expect_err("invalid title should return an error");

        assert!(matches!(error, DomainError::Validation(_)));
    }

    #[actix_web::test]
    async fn get_post_returns_not_found_for_missing_post() {
        let service = BlogService::new(Arc::new(TestPostRepository::default()));

        let error = service
            .get_post(42)
            .await
            .expect_err("missing post should return an error");

        assert!(matches!(error, DomainError::PostNotFound(42)));
    }

    #[actix_web::test]
    async fn update_post_rejects_non_author() {
        let author_id = Uuid::new_v4();
        let other_user_id = Uuid::new_v4();
        let post = test_post(1, author_id, "Заголовок", "Содержимое");
        let service = BlogService::new(Arc::new(TestPostRepository::with_posts(vec![post])));

        let error = service
            .update_post(
                other_user_id,
                1,
                UpdatePost::new("Новый заголовок", "Новый текст".to_string())
                    .expect("update should be valid"),
            )
            .await
            .expect_err("non-author should not update post");

        assert!(matches!(error, DomainError::Forbidden));
    }

    #[actix_web::test]
    async fn update_post_changes_author_post() {
        let author_id = Uuid::new_v4();
        let post = test_post(1, author_id, "Старый заголовок", "Старый текст");
        let service = BlogService::new(Arc::new(TestPostRepository::with_posts(vec![post])));

        let updated_post = service
            .update_post(
                author_id,
                1,
                UpdatePost::new("Новый заголовок", "Новый текст".to_string())
                    .expect("update should be valid"),
            )
            .await
            .expect("author should update post");

        assert_eq!(updated_post.title, "Новый заголовок");
        assert_eq!(updated_post.content, "Новый текст");
        assert!(updated_post.updated_at.is_some());
    }

    #[actix_web::test]
    async fn delete_post_rejects_non_author() {
        let author_id = Uuid::new_v4();
        let other_user_id = Uuid::new_v4();
        let post = test_post(1, author_id, "Заголовок", "Содержимое");
        let service = BlogService::new(Arc::new(TestPostRepository::with_posts(vec![post])));

        let error = service
            .delete_post(other_user_id, 1)
            .await
            .expect_err("non-author should not delete post");

        assert!(matches!(error, DomainError::Forbidden));
    }

    #[actix_web::test]
    async fn list_posts_returns_page_and_total() {
        let author_id = Uuid::new_v4();
        let posts = vec![
            test_post(1, author_id, "Первый", "Текст"),
            test_post(2, author_id, "Второй", "Текст"),
        ];
        let service = BlogService::new(Arc::new(TestPostRepository::with_posts(posts)));

        let page = service
            .list_posts(1, 1)
            .await
            .expect("posts page should be returned");

        assert_eq!(page.total, 2);
        assert_eq!(page.posts.len(), 1);
        assert_eq!(page.posts[0].id, 2);
    }

    fn test_post(id: i64, author_id: Uuid, title: &str, content: &str) -> Post {
        Post {
            id,
            title: title.to_string(),
            content: content.to_string(),
            author_id,
            created_at: Utc::now(),
            updated_at: None,
        }
    }

    fn clone_post(post: &Post) -> Post {
        Post {
            id: post.id,
            title: post.title.clone(),
            content: post.content.clone(),
            author_id: post.author_id,
            created_at: post.created_at,
            updated_at: post.updated_at,
        }
    }
}
