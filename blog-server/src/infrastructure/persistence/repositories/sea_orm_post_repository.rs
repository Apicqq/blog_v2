//! Репозиторий постов на `SeaORM`.

use async_trait::async_trait;
use sea_orm::DatabaseConnection;

use crate::application::ports::post_repository::PostRepository;
use crate::domain::errors::DomainError;
use crate::domain::post::{Post, PostAttributes};

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
        let _ = (&self.db, attributes);
        todo!("реализовать создание поста через SeaORM")
    }

    async fn find_by_id(&self, id: u64) -> Result<Option<Post>, DomainError> {
        let _ = (&self.db, id);
        todo!("реализовать поиск поста по ID через SeaORM")
    }

    async fn update(&self, post: Post) -> Result<Post, DomainError> {
        let _ = (&self.db, post);
        todo!("реализовать обновление поста через SeaORM")
    }

    async fn delete(&self, id: u64) -> Result<(), DomainError> {
        let _ = (&self.db, id);
        todo!("реализовать удаление поста через SeaORM")
    }

    async fn list(&self, limit: u64, offset: u64) -> Result<Vec<Post>, DomainError> {
        let _ = (&self.db, limit, offset);
        todo!("реализовать получение списка постов через SeaORM")
    }
}
