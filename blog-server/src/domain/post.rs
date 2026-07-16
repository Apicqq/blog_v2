use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::errors::DomainError;

const MIN_POST_TITLE_LENGTH: usize = 3;
const MAX_POST_TITLE_LENGTH: usize = 255;
const MIN_POST_CONTENT_LENGTH: usize = 1;
const MAX_POST_CONTENT_LENGTH: usize = 10_000;

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
    title: String,
    /// Содержимое поста.
    content: String,
    /// Идентификатор автора поста.
    author_id: Uuid,
}

impl PostAttributes {
    /// Создает существенные данные поста.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку валидации, если заголовок или содержимое не соответствуют доменным
    /// ограничениям.
    pub fn new(title: &str, content: String, author_id: Uuid) -> Result<Self, DomainError> {
        let title = title.trim().to_string();
        validate_post_data(&title, &content)?;

        Ok(Self {
            title,
            content,
            author_id,
        })
    }

    /// Разбирает данные поста на поля для слоя хранения.
    #[must_use]
    pub fn into_parts(self) -> (String, String, Uuid) {
        (self.title, self.content, self.author_id)
    }
}

/// Данные для обновления поста.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePost {
    /// Новый заголовок поста.
    title: String,
    /// Новое содержимое поста.
    content: String,
}

impl UpdatePost {
    /// Создает данные для обновления поста.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку валидации, если заголовок или содержимое не соответствуют доменным
    /// ограничениям.
    pub fn new(title: &str, content: String) -> Result<Self, DomainError> {
        let title = title.trim().to_string();
        validate_post_data(&title, &content)?;

        Ok(Self { title, content })
    }
}

fn validate_post_data(title: &str, content: &str) -> Result<(), DomainError> {
    let title_length = title.chars().count();
    let content_length = content.chars().count();

    if title_length < MIN_POST_TITLE_LENGTH {
        return Err(DomainError::Validation(
            "post title must contain at least 3 characters".to_string(),
        ));
    }

    if title_length > MAX_POST_TITLE_LENGTH {
        return Err(DomainError::Validation(
            "post title must contain at most 255 characters".to_string(),
        ));
    }

    if content_length < MIN_POST_CONTENT_LENGTH {
        return Err(DomainError::Validation(
            "post content must not be empty".to_string(),
        ));
    }

    if content_length > MAX_POST_CONTENT_LENGTH {
        return Err(DomainError::Validation(
            "post content must contain at most 10000 characters".to_string(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_attributes_creates_post_with_expected_fields() {
        let author_id = Uuid::new_v4();
        let attributes = PostAttributes::new("Заголовок", "Содержимое".to_string(), author_id)
            .expect("post attributes should be valid");

        let post = Post::from_attributes(1, attributes);

        assert_eq!(post.id, 1);
        assert_eq!(post.title, "Заголовок");
        assert_eq!(post.content, "Содержимое");
        assert_eq!(post.author_id, author_id);
        assert!(post.updated_at.is_none());
    }

    #[test]
    fn post_attributes_rejects_short_title_after_trim() {
        let error = PostAttributes::new("  a  ", "Содержимое".to_string(), Uuid::new_v4())
            .expect_err("short title should be rejected");

        assert!(matches!(error, DomainError::Validation(_)));
    }

    #[test]
    fn update_changes_content_and_sets_updated_at() {
        let author_id = Uuid::new_v4();
        let attributes = PostAttributes::new(
            "Старый заголовок",
            "Старое содержимое".to_string(),
            author_id,
        )
        .expect("post attributes should be valid");
        let mut post = Post::from_attributes(1, attributes);

        post.update(
            UpdatePost::new("Новый заголовок", "Новое содержимое".to_string())
                .expect("update should be valid"),
        );

        assert_eq!(post.title, "Новый заголовок");
        assert_eq!(post.content, "Новое содержимое");
        assert!(post.updated_at.is_some());
    }

    #[test]
    fn update_post_rejects_empty_content() {
        let error = UpdatePost::new("Новый заголовок", String::new())
            .expect_err("empty content should be rejected");

        assert!(matches!(error, DomainError::Validation(_)));
    }

    #[test]
    fn is_author_checks_post_owner() {
        let author_id = Uuid::new_v4();
        let other_user_id = Uuid::new_v4();
        let attributes = PostAttributes::new("Заголовок", "Содержимое".to_string(), author_id)
            .expect("post attributes should be valid");
        let post = Post::from_attributes(1, attributes);

        assert!(post.is_author(author_id));
        assert!(!post.is_author(other_user_id));
    }
}
