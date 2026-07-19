//! DTO HTTP API фронтенда.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub(crate) struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_username: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub(crate) struct PostPage {
    pub posts: Vec<Post>,
    pub total: u64,
    pub limit: u64,
    pub offset: u64,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub(crate) struct User {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub(crate) struct AuthResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Serialize)]
pub(crate) struct RegisterRequest<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Serialize)]
pub(crate) struct LoginRequest<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Serialize)]
pub(crate) struct CreatePostRequest<'a> {
    pub title: &'a str,
    pub content: &'a str,
}

#[derive(Debug, Serialize)]
pub(crate) struct UpdatePostRequest<'a> {
    pub title: &'a str,
    pub content: &'a str,
}
