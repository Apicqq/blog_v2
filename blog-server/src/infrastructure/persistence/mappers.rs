//! Преобразования между моделями хранения и доменными моделями.

use crate::domain::post::Post;
use crate::domain::user::User;
use crate::infrastructure::persistence::entities::{post, user};

impl From<user::Model> for User {
    fn from(model: user::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            email: model.email,
            password_hash: model.password_hash,
            created_at: model.created_at,
        }
    }
}

impl From<post::Model> for Post {
    fn from(model: post::Model) -> Self {
        Self {
            id: model.id,
            title: model.title,
            content: model.content,
            author_id: model.author_id,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
