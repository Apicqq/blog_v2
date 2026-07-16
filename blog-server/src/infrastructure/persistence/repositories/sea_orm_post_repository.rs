//! Репозиторий постов на `SeaORM`.

use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, NotSet, QueryOrder, QuerySelect, Set,
    Unchanged,
};

use crate::application::ports::post_repository::PostRepository;
use crate::domain::errors::DomainError;
use crate::domain::post::{Post, PostAttributes};
use crate::infrastructure::persistence::entities::post::{ActiveModel, Column, Entity as DBPost};

/// `SeaORM`-реализация хранилища постов.
#[derive(Debug, Clone)]
pub struct SeaOrmPostRepository {
    db: DatabaseConnection,
}

impl SeaOrmPostRepository {
    /// Создает репозиторий постов.
    #[must_use]
    pub const fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl PostRepository for SeaOrmPostRepository {
    async fn create(&self, attributes: PostAttributes) -> Result<Post, DomainError> {
        let new_post = ActiveModel {
            id: NotSet,
            title: Set(attributes.title),
            content: Set(attributes.content),
            author_id: Set(attributes.author_id),
            created_at: Set(Utc::now()),
            updated_at: Set(None),
        };

        let post = new_post.insert(&self.db).await?;

        Ok(Post::from(post))
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Post>, DomainError> {
        let post = DBPost::find_by_id(id).one(&self.db).await?;
        Ok(post.map(Post::from))
    }

    async fn update(&self, post: Post) -> Result<Post, DomainError> {
        let active_model = ActiveModel {
            id: Unchanged(post.id),
            title: Set(post.title),
            content: Set(post.content),
            author_id: Unchanged(post.author_id),
            created_at: Unchanged(post.created_at),
            updated_at: Set(post.updated_at),
        };

        let model = active_model.update(&self.db).await?;

        Ok(Post::from(model))
    }

    async fn delete(&self, id: i64) -> Result<(), DomainError> {
        let result = DBPost::delete_by_id(id).exec(&self.db).await?;

        if result.rows_affected == 0 {
            return Err(DomainError::PostNotFound(id));
        }

        Ok(())
    }

    async fn list(&self, limit: u64, offset: u64) -> Result<Vec<Post>, DomainError> {
        let posts = DBPost::find()
            .order_by_desc(Column::CreatedAt)
            .limit(limit)
            .offset(offset)
            .all(&self.db)
            .await?;

        Ok(posts.into_iter().map(Post::from).collect())
    }
}
