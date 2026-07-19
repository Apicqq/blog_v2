//! Клиентские модели API блога.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Пользователь блога.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    /// Имя пользователя.
    pub username: String,
    /// Электронная почта пользователя.
    pub email: String,
}

/// Ответ успешной аутентификации.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthResponse {
    /// JWT-токен доступа.
    pub token: String,
    /// Пользователь, для которого выпущен токен.
    pub user: User,
}

/// Пост блога.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Post {
    /// Идентификатор поста.
    pub id: i64,
    /// Заголовок поста.
    pub title: String,
    /// Содержимое поста.
    pub content: String,
    /// Идентификатор автора.
    pub author_id: String,
    /// Время создания.
    pub created_at: DateTime<Utc>,
    /// Время последнего обновления.
    pub updated_at: Option<DateTime<Utc>>,
}

/// Страница постов.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PostPage {
    /// Посты текущей страницы.
    pub posts: Vec<Post>,
    /// Общее количество постов.
    pub total: u64,
    /// Размер страницы.
    pub limit: u64,
    /// Смещение страницы.
    pub offset: u64,
}

impl From<blog_proto::generated::User> for User {
    fn from(user: blog_proto::generated::User) -> Self {
        Self {
            username: user.username,
            email: user.email,
        }
    }
}

impl From<blog_proto::generated::AuthResponse> for AuthResponse {
    fn from(response: blog_proto::generated::AuthResponse) -> Self {
        let user = response.user.map_or_else(
            || User {
                username: String::new(),
                email: String::new(),
            },
            User::from,
        );

        Self {
            token: response.token,
            user,
        }
    }
}

impl From<blog_proto::generated::Post> for Post {
    fn from(post: blog_proto::generated::Post) -> Self {
        Self {
            id: post.id,
            title: post.title,
            content: post.content,
            author_id: post.author_id,
            created_at: timestamp_to_datetime(post.created_at),
            updated_at: post.updated_at.map(timestamp_to_datetime),
        }
    }
}

impl From<blog_proto::generated::ListPostsResponse> for PostPage {
    fn from(response: blog_proto::generated::ListPostsResponse) -> Self {
        Self {
            posts: response.posts.into_iter().map(Post::from).collect(),
            total: response.total,
            limit: response.limit,
            offset: response.offset,
        }
    }
}

fn timestamp_to_datetime(timestamp: i64) -> DateTime<Utc> {
    DateTime::from_timestamp(timestamp, 0).unwrap_or(DateTime::<Utc>::UNIX_EPOCH)
}
