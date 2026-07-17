//! DTO постов блога.

use crate::application::blog_service::PostPage;
use crate::domain::errors::DomainError;
use crate::domain::post::{Post, UpdatePost};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const DEFAULT_POSTS_LIMIT: u64 = 10;
const MAX_POSTS_LIMIT: u64 = 100;

/// Запрос создания поста.
#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    /// Заголовок поста.
    pub title: String,
    /// Содержимое поста.
    pub content: String,
}

/// Запрос обновления поста.
#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    /// Новый заголовок поста.
    pub title: String,
    /// Новое содержимое поста.
    pub content: String,
}

impl TryFrom<UpdatePostRequest> for UpdatePost {
    type Error = DomainError;

    fn try_from(request: UpdatePostRequest) -> Result<Self, Self::Error> {
        Self::new(&request.title, request.content)
    }
}

/// Ответ с постом.
#[derive(Debug, Serialize)]
pub struct PostResponse {
    /// Идентификатор поста.
    pub id: i64,
    /// Заголовок поста.
    pub title: String,
    /// Содержимое поста.
    pub content: String,
    /// Идентификатор автора поста.
    pub author_id: String,
    /// Время создания поста.
    pub created_at: DateTime<Utc>,
    /// Время последнего обновления поста.
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Post> for PostResponse {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            title: post.title,
            content: post.content,
            author_id: post.author_id.to_string(),
            created_at: post.created_at,
            updated_at: post.updated_at,
        }
    }
}

/// Query-параметры списка постов.
#[derive(Debug, Deserialize)]
pub struct ListPostsQuery {
    /// Размер страницы.
    pub limit: Option<u64>,
    /// Смещение страницы.
    pub offset: Option<u64>,
}

impl ListPostsQuery {
    /// Возвращает нормализованный размер страницы.
    #[must_use]
    pub fn limit(&self) -> u64 {
        self.limit
            .unwrap_or(DEFAULT_POSTS_LIMIT)
            .clamp(1, MAX_POSTS_LIMIT)
    }

    /// Возвращает нормализованное смещение страницы.
    #[must_use]
    pub fn offset(&self) -> u64 {
        self.offset.unwrap_or(0)
    }
}

/// Ответ со списком постов.
#[derive(Debug, Serialize)]
pub struct ListPostsResponse {
    /// Посты текущей страницы.
    pub posts: Vec<PostResponse>,
    /// Общее количество постов.
    pub total: u64,
    /// Размер страницы.
    pub limit: u64,
    /// Смещение страницы.
    pub offset: u64,
}

impl ListPostsResponse {
    /// Создает ответ со списком постов.
    #[must_use]
    pub fn new(page: PostPage, limit: u64, offset: u64) -> Self {
        Self {
            posts: page.posts.into_iter().map(PostResponse::from).collect(),
            total: page.total,
            limit,
            offset,
        }
    }
}
