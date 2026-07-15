//! Репозиторий пользователей на `SeaORM`.

use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use uuid::Uuid;

use crate::application::ports::user_repository::UserRepository;
use crate::domain::errors::DomainError;
use crate::domain::user::User;
use crate::infrastructure::persistence::entities::user;

/// `SeaORM`-реализация хранилища пользователей.
#[derive(Debug, Clone)]
pub struct SeaOrmUserRepository {
    db: DatabaseConnection,
}

impl SeaOrmUserRepository {
    /// Создает репозиторий пользователей.
    #[must_use]
    pub const fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for SeaOrmUserRepository {
    async fn create(&self, user: User) -> Result<User, DomainError> {
        let active_model = user::ActiveModel {
            id: Set(user.id),
            username: Set(user.username),
            email: Set(user.email),
            password_hash: Set(user.password_hash),
            created_at: Set(user.created_at),
        };

        let model = active_model
            .insert(&self.db)
            .await
            .map_err(|err| DomainError::Internal(err.to_string()))?;

        Ok(User::from(model))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        let _ = (&self.db, id);
        todo!("реализовать поиск пользователя по ID через SeaORM")
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError> {
        let _ = (&self.db, username);
        todo!("реализовать поиск пользователя по имени через SeaORM")
    }

    async fn exists_by_username(&self, username: &str) -> Result<bool, DomainError> {
        let _ = (&self.db, username);
        todo!("реализовать проверку существования пользователя через SeaORM")
    }
}
