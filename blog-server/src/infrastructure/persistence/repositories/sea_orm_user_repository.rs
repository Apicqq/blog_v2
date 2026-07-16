//! Репозиторий пользователей на `SeaORM`.

use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, RuntimeErr, Set, SqlxError,
};
use uuid::Uuid;

use crate::application::ports::user_repository::UserRepository;
use crate::domain::errors::DomainError;
use crate::domain::user::User;
use crate::infrastructure::persistence::entities::user::{self, ActiveModel, Entity as DBUser};

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
        let active_model = ActiveModel {
            id: Set(user.id),
            username: Set(user.username),
            email: Set(user.email),
            password_hash: Set(user.password_hash),
            created_at: Set(user.created_at),
        };

        let model = active_model
            .insert(&self.db)
            .await
            .map_err(map_user_create_error)?;

        Ok(User::from(model))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        let user = DBUser::find_by_id(id).one(&self.db).await?;

        Ok(user.map(User::from))
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError> {
        let user = DBUser::find()
            .filter(user::Column::Username.eq(username))
            .one(&self.db)
            .await?;

        Ok(user.map(User::from))
    }

    async fn exists_by_username(&self, username: &str) -> Result<bool, DomainError> {
        let user_exists = DBUser::find()
            .filter(user::Column::Username.eq(username))
            .exists(&self.db)
            .await?;

        Ok(user_exists)
    }

    async fn exists_by_email(&self, email: &str) -> Result<bool, DomainError> {
        let user_exists = DBUser::find()
            .filter(user::Column::Email.eq(email))
            .exists(&self.db)
            .await?;

        Ok(user_exists)
    }
}

fn map_user_create_error(err: DbErr) -> DomainError {
    unique_constraint_error(&err).unwrap_or_else(|| err.into())
}

fn unique_constraint_error(err: &DbErr) -> Option<DomainError> {
    let (DbErr::Exec(RuntimeErr::SqlxError(SqlxError::Database(database_error)))
    | DbErr::Query(RuntimeErr::SqlxError(SqlxError::Database(database_error)))) = err
    else {
        return None;
    };

    if database_error.code().as_deref() != Some("23505") {
        // unique violation
        return None;
    }

    match database_error.constraint()? {
        "users_username_key" => Some(DomainError::UsernameAlreadyTaken),
        "users_email_key" => Some(DomainError::EmailAlreadyTaken),
        _ => None,
    }
}
