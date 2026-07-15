//! Таблица постов для `SeaORM`.

use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use uuid::Uuid;

/// Строка таблицы `posts`.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    /// ID поста.
    #[sea_orm(primary_key)]
    pub id: i64,

    /// Заголовок поста.
    pub title: String,

    /// Содержимое поста.
    pub content: String,

    /// ID автора поста.
    pub author_id: Uuid,

    /// Время создания поста.
    pub created_at: DateTime<Utc>,

    /// Время последнего обновления поста.
    pub updated_at: Option<DateTime<Utc>>,
}

/// Связи таблицы `posts`.
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    /// Автор поста.
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(super::user::Entity)
                .from(Column::AuthorId)
                .to(super::user::Column::Id)
                .into(),
        }
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
