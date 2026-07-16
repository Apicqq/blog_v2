//! Таблица пользователей для `SeaORM`.

use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use uuid::Uuid;

/// Строка таблицы `users`.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    /// ID пользователя.
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    /// Имя пользователя.
    #[sea_orm(unique)]
    pub username: String,

    /// Электронная почта пользователя.
    #[sea_orm(unique)]
    pub email: String,

    /// Хеш пароля пользователя.
    pub password_hash: String,

    /// Время создания пользователя.
    pub created_at: DateTime<Utc>,
}

/// Связи таблицы `users`.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
