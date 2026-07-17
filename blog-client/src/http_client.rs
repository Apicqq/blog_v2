//! HTTP-клиент для взаимодействия с API блога.

use crate::errors::BlogClientError;
use crate::models::{AuthResponse, Post, PostPage};

/// HTTP-клиент блога.
#[derive(Debug, Clone)]
pub struct HttpClient {
    base_url: String,
    client: reqwest::Client,
}

impl HttpClient {
    /// Создает новый HTTP-клиент блога.
    #[must_use]
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: reqwest::Client::new(),
        }
    }

    /// Возвращает базовый URL HTTP API.
    #[must_use]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Возвращает внутренний `reqwest`-клиент.
    #[must_use]
    pub const fn inner(&self) -> &reqwest::Client {
        &self.client
    }

    /// Регистрирует пользователя через HTTP API.
    ///
    /// # Errors
    ///
    /// Сейчас возвращает `InvalidRequest`, пока HTTP-запросы не реализованы.
    pub async fn register(
        &self,
        _username: &str,
        _email: &str,
        _password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        std::future::ready(()).await;
        Err(BlogClientError::not_implemented("http register"))
    }

    /// Выполняет вход через HTTP API.
    ///
    /// # Errors
    ///
    /// Сейчас возвращает `InvalidRequest`, пока HTTP-запросы не реализованы.
    pub async fn login(
        &self,
        _username: &str,
        _password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        std::future::ready(()).await;
        Err(BlogClientError::not_implemented("http login"))
    }

    /// Создает пост через HTTP API.
    ///
    /// # Errors
    ///
    /// Сейчас возвращает `InvalidRequest`, пока HTTP-запросы не реализованы.
    pub async fn create_post(
        &self,
        _token: &str,
        _title: &str,
        _content: &str,
    ) -> Result<Post, BlogClientError> {
        std::future::ready(()).await;
        Err(BlogClientError::not_implemented("http create_post"))
    }

    /// Возвращает пост через HTTP API.
    ///
    /// # Errors
    ///
    /// Сейчас возвращает `InvalidRequest`, пока HTTP-запросы не реализованы.
    pub async fn get_post(&self, _id: i64) -> Result<Post, BlogClientError> {
        std::future::ready(()).await;
        Err(BlogClientError::not_implemented("http get_post"))
    }

    /// Обновляет пост через HTTP API.
    ///
    /// # Errors
    ///
    /// Сейчас возвращает `InvalidRequest`, пока HTTP-запросы не реализованы.
    pub async fn update_post(
        &self,
        _token: &str,
        _id: i64,
        _title: &str,
        _content: &str,
    ) -> Result<Post, BlogClientError> {
        std::future::ready(()).await;
        Err(BlogClientError::not_implemented("http update_post"))
    }

    /// Удаляет пост через HTTP API.
    ///
    /// # Errors
    ///
    /// Сейчас возвращает `InvalidRequest`, пока HTTP-запросы не реализованы.
    pub async fn delete_post(&self, _token: &str, _id: i64) -> Result<(), BlogClientError> {
        std::future::ready(()).await;
        Err(BlogClientError::not_implemented("http delete_post"))
    }

    /// Возвращает страницу постов через HTTP API.
    ///
    /// # Errors
    ///
    /// Сейчас возвращает `InvalidRequest`, пока HTTP-запросы не реализованы.
    pub async fn list_posts(&self, _limit: u64, _offset: u64) -> Result<PostPage, BlogClientError> {
        std::future::ready(()).await;
        Err(BlogClientError::not_implemented("http list_posts"))
    }
}
