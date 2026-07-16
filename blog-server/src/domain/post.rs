use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Пост блога.
#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    /// ID поста.
    pub id: i64, // синхронизирую с БД чтобы не плодить TryFrom
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

impl Post {
    /// Создает пост из существенных данных и присвоенного ID.
    #[must_use]
    pub fn from_attributes(id: i64, attributes: PostAttributes) -> Self {
        Self {
            id,
            title: attributes.title,
            content: attributes.content,
            author_id: attributes.author_id,
            created_at: Utc::now(),
            updated_at: None,
        }
    }

    /// Обновляет заголовок и содержимое поста.
    pub fn update(&mut self, update: UpdatePost) {
        self.title = update.title;
        self.content = update.content;
        self.updated_at = Some(Utc::now());
    }

    /// Проверяет, принадлежит ли пост пользователю.
    #[must_use]
    pub fn is_author(&self, user_id: Uuid) -> bool {
        self.author_id == user_id
    }
}

/// Существенные данные поста без инфра-обвязки.
#[derive(Debug, Serialize, Deserialize)]
pub struct PostAttributes {
    /// Заголовок поста.
    pub title: String,
    /// Содержимое поста.
    pub content: String,
    /// Идентификатор автора поста.
    pub author_id: Uuid,
}

impl PostAttributes {
    /// Создает существенные данные поста.
    #[must_use]
    pub fn new(title: String, content: String, author_id: Uuid) -> Self {
        Self {
            title,
            content,
            author_id,
        }
    }
}

/// Данные для обновления поста.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePost {
    /// Новый заголовок поста.
    pub title: String,
    /// Новое содержимое поста.
    pub content: String,
}

impl UpdatePost {
    /// Создает данные для обновления поста.
    #[must_use]
    pub const fn new(title: String, content: String) -> Self {
        Self { title, content }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_attributes_creates_post_with_expected_fields() {
        let author_id = Uuid::new_v4();
        let attributes =
            PostAttributes::new("Заголовок".to_string(), "Содержимое".to_string(), author_id);

        let post = Post::from_attributes(1, attributes);

        assert_eq!(post.id, 1);
        assert_eq!(post.title, "Заголовок");
        assert_eq!(post.content, "Содержимое");
        assert_eq!(post.author_id, author_id);
        assert!(post.updated_at.is_none());
    }

    #[test]
    fn update_changes_content_and_sets_updated_at() {
        let author_id = Uuid::new_v4();
        let attributes = PostAttributes::new(
            "Старый заголовок".to_string(),
            "Старое содержимое".to_string(),
            author_id,
        );
        let mut post = Post::from_attributes(1, attributes);

        post.update(UpdatePost::new(
            "Новый заголовок".to_string(),
            "Новое содержимое".to_string(),
        ));

        assert_eq!(post.title, "Новый заголовок");
        assert_eq!(post.content, "Новое содержимое");
        assert!(post.updated_at.is_some());
    }

    #[test]
    fn is_author_checks_post_owner() {
        let author_id = Uuid::new_v4();
        let other_user_id = Uuid::new_v4();
        let attributes =
            PostAttributes::new("Заголовок".to_string(), "Содержимое".to_string(), author_id);
        let post = Post::from_attributes(1, attributes);

        assert!(post.is_author(author_id));
        assert!(!post.is_author(other_user_id));
    }
}
