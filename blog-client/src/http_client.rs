//! HTTP-клиент для взаимодействия с API блога.

use crate::errors::BlogClientError;
use crate::models::{AuthResponse, Post, PostPage};
use serde::Serialize;

/// HTTP-клиент блога.
#[derive(Debug, Clone)]
pub struct HttpClient {
    base_url: String,
    client: reqwest::Client,
}

#[derive(Debug, Serialize)]
struct RegisterRequest<'a> {
    username: &'a str,
    email: &'a str,
    password: &'a str,
}

#[derive(Debug, Serialize)]
struct LoginRequest<'a> {
    username: &'a str,
    password: &'a str,
}

#[derive(Debug, Serialize)]
struct CreatePostRequest<'a> {
    title: &'a str,
    content: &'a str,
}

#[derive(Debug, Serialize)]
struct UpdatePostRequest<'a> {
    title: &'a str,
    content: &'a str,
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
    pub const fn client(&self) -> &reqwest::Client {
        &self.client
    }

    fn endpoint(&self, path: &str) -> String {
        format!("{}/{}", self.base_url, path.trim_start_matches('/'))
    }

    fn api_endpoint(&self, path: &str) -> String {
        self.endpoint(&format!("/api/{}", path.trim_start_matches('/')))
    }

    /// Регистрирует пользователя через HTTP API.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если HTTP-запрос не выполнен, сервер вернул ошибочный статус
    /// или ответ не удалось десериализовать.
    pub async fn register(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        let response = self
            .client
            .post(self.api_endpoint("/auth/register"))
            .json(&RegisterRequest {
                username,
                email,
                password,
            })
            .send()
            .await?;
        self.handle_json(response).await
    }

    /// Выполняет вход через HTTP API.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если HTTP-запрос не выполнен, учетные данные отклонены,
    /// сервер вернул ошибочный статус или ответ не удалось десериализовать.
    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        let response = self
            .client
            .post(self.api_endpoint("/auth/login"))
            .json(&LoginRequest { username, password })
            .send()
            .await?;
        self.handle_json(response).await
    }

    /// Создает пост через HTTP API.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если HTTP-запрос не выполнен, токен отклонен, данные поста
    /// не прошли валидацию, сервер вернул ошибочный статус или ответ не удалось десериализовать.
    pub async fn create_post(
        &self,
        token: &str,
        title: &str,
        content: &str,
    ) -> Result<Post, BlogClientError> {
        let response = self
            .client()
            .post(self.api_endpoint("/posts"))
            .bearer_auth(token)
            .json(&CreatePostRequest { title, content })
            .send()
            .await?;
        self.handle_json(response).await
    }

    /// Возвращает пост через HTTP API.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если HTTP-запрос не выполнен, пост не найден, сервер вернул
    /// ошибочный статус или ответ не удалось десериализовать.
    pub async fn get_post(&self, id: i64) -> Result<Post, BlogClientError> {
        let response = self
            .client()
            .get(self.api_endpoint(&format!("/posts/{id}")))
            .send()
            .await?;
        self.handle_json(response).await
    }

    /// Обновляет пост через HTTP API.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если HTTP-запрос не выполнен, токен отклонен, доступ запрещен,
    /// пост не найден, данные не прошли валидацию или ответ не удалось десериализовать.
    pub async fn update_post(
        &self,
        token: &str,
        id: i64,
        title: &str,
        content: &str,
    ) -> Result<Post, BlogClientError> {
        let response = self
            .client()
            .put(self.api_endpoint(&format!("/posts/{id}")))
            .bearer_auth(token)
            .json(&UpdatePostRequest { title, content })
            .send()
            .await?;

        self.handle_json(response).await
    }

    /// Удаляет пост через HTTP API.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если HTTP-запрос не выполнен, токен отклонен, доступ запрещен,
    /// пост не найден или сервер вернул ошибочный статус.
    pub async fn delete_post(&self, token: &str, id: i64) -> Result<(), BlogClientError> {
        let response = self
            .client()
            .delete(self.api_endpoint(&format!("/posts/{id}")))
            .bearer_auth(token)
            .send()
            .await?;

        self.handle_empty(response).await
    }

    /// Возвращает страницу постов через HTTP API.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если HTTP-запрос не выполнен, сервер вернул ошибочный статус
    /// или ответ не удалось десериализовать.
    pub async fn list_posts(&self, limit: u64, offset: u64) -> Result<PostPage, BlogClientError> {
        let response = self
            .client()
            .get(self.api_endpoint(&format!("/posts?limit={limit}&offset={offset}")))
            .send()
            .await?;

        self.handle_json(response).await
    }

    async fn handle_json<T>(&self, response: reqwest::Response) -> Result<T, BlogClientError>
    where
        T: serde::de::DeserializeOwned,
    {
        match response.status() {
            status if status.is_success() => Ok(response.json().await?),
            reqwest::StatusCode::UNAUTHORIZED => Err(BlogClientError::Unauthorized),
            reqwest::StatusCode::FORBIDDEN => Err(BlogClientError::Forbidden),
            reqwest::StatusCode::NOT_FOUND => Err(BlogClientError::NotFound),
            reqwest::StatusCode::CONFLICT => {
                let body = response.text().await.unwrap_or_default();
                Err(BlogClientError::Conflict(body))
            }
            status if status.is_client_error() => {
                let body = response.text().await.unwrap_or_default();
                Err(BlogClientError::InvalidRequest(body))
            }
            _ => Err(response.error_for_status().unwrap_err().into()),
        }
    }

    async fn handle_empty(&self, response: reqwest::Response) -> Result<(), BlogClientError> {
        match response.status() {
            status if status.is_success() => Ok(()),
            reqwest::StatusCode::UNAUTHORIZED => Err(BlogClientError::Unauthorized),
            reqwest::StatusCode::FORBIDDEN => Err(BlogClientError::Forbidden),
            reqwest::StatusCode::NOT_FOUND => Err(BlogClientError::NotFound),
            reqwest::StatusCode::CONFLICT => {
                let body = response.text().await.unwrap_or_default();
                Err(BlogClientError::Conflict(body))
            }
            status if status.is_client_error() => {
                let body = response.text().await.unwrap_or_default();
                Err(BlogClientError::InvalidRequest(body))
            }
            _ => Err(response.error_for_status().unwrap_err().into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_trims_trailing_slash_from_base_url() {
        let client = HttpClient::new("http://localhost:8080/");

        assert_eq!(client.base_url(), "http://localhost:8080");
    }

    #[test]
    fn api_endpoint_builds_url_under_api_scope() {
        let client = HttpClient::new("http://localhost:8080/");

        assert_eq!(
            client.api_endpoint("/auth/register"),
            "http://localhost:8080/api/auth/register"
        );
        assert_eq!(
            client.api_endpoint("posts?limit=10&offset=0"),
            "http://localhost:8080/api/posts?limit=10&offset=0"
        );
    }
}
