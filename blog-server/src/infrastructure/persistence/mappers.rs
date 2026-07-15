//! Преобразования между моделями хранения и доменными моделями.

use crate::domain::user::User;
use crate::infrastructure::persistence::entities::user;

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
