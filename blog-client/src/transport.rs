//! Фасад клиентской библиотеки и выбор транспорта.

use crate::errors::BlogClientError;
use crate::grpc_client::GrpcClient;
use crate::http_client::HttpClient;
use crate::models::{AuthResponse, Post, PostPage};

/// Транспорт клиентской библиотеки.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Transport {
    /// HTTP API с базовым URL.
    Http(String),
    /// gRPC API с endpoint.
    Grpc(String),
}

/// Клиент API блога.
#[derive(Debug, Clone)]
pub struct BlogClient {
    transport: Transport,
    http_client: Option<HttpClient>,
    grpc_client: Option<GrpcClient>,
    token: Option<String>,
}

impl BlogClient {
    /// Создает клиент для выбранного транспорта.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если gRPC-соединение не удалось установить.
    pub async fn new(transport: Transport) -> Result<Self, BlogClientError> {
        let (http_client, grpc_client) = match &transport {
            Transport::Http(base_url) => (Some(HttpClient::new(base_url)), None),
            Transport::Grpc(endpoint) => (None, Some(GrpcClient::connect(endpoint.clone()).await?)),
        };

        Ok(Self {
            transport,
            http_client,
            grpc_client,
            token: None,
        })
    }

    /// Возвращает транспорт клиента.
    #[must_use]
    pub const fn transport(&self) -> &Transport {
        &self.transport
    }

    /// Возвращает HTTP-клиент, если выбран HTTP-транспорт.
    #[must_use]
    pub const fn http_client(&self) -> Option<&HttpClient> {
        self.http_client.as_ref()
    }

    /// Возвращает gRPC-клиент, если выбран gRPC-транспорт.
    #[must_use]
    pub const fn grpc_client(&self) -> Option<&GrpcClient> {
        self.grpc_client.as_ref()
    }

    /// Сохраняет JWT-токен для защищенных операций.
    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    /// Возвращает текущий JWT-токен.
    #[must_use]
    pub fn get_token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    /// Регистрирует пользователя.
    ///
    /// # Errors
    ///
    /// Сейчас возвращает `InvalidRequest`, пока транспортные методы не реализованы.
    pub async fn register(
        &mut self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        let session = match self.active_transport()? {
            ActiveTransport::Http(client) => client.register(username, email, password).await?,
            ActiveTransport::Grpc(client) => client.register(username, email, password).await?,
        };
        self.set_token(session.token.clone());

        Ok(session)
    }

    /// Выполняет вход пользователя.
    ///
    /// # Errors
    ///
    /// Сейчас возвращает `InvalidRequest`, пока транспортные методы не реализованы.
    pub async fn login(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        let session = match self.active_transport()? {
            ActiveTransport::Http(client) => client.login(username, password).await?,
            ActiveTransport::Grpc(client) => client.login(username, password).await?,
        };
        self.set_token(session.token.clone());

        Ok(session)
    }

    /// Создает пост.
    ///
    /// # Errors
    ///
    /// Возвращает `MissingToken`, если токен не задан. Сейчас транспортные методы еще не реализованы.
    pub async fn create_post(&self, title: &str, content: &str) -> Result<Post, BlogClientError> {
        let token = self.require_token()?;

        match self.active_transport()? {
            ActiveTransport::Http(client) => client.create_post(token, title, content).await,
            ActiveTransport::Grpc(client) => client.create_post(token, title, content).await,
        }
    }

    /// Возвращает пост по идентификатору.
    ///
    /// # Errors
    ///
    /// Сейчас возвращает `InvalidRequest`, пока транспортные методы не реализованы.
    pub async fn get_post(&self, id: i64) -> Result<Post, BlogClientError> {
        match self.active_transport()? {
            ActiveTransport::Http(client) => client.get_post(id).await,
            ActiveTransport::Grpc(client) => client.get_post(id).await,
        }
    }

    /// Обновляет пост.
    ///
    /// # Errors
    ///
    /// Возвращает `MissingToken`, если токен не задан. Сейчас транспортные методы еще не реализованы.
    pub async fn update_post(
        &self,
        id: i64,
        title: &str,
        content: &str,
    ) -> Result<Post, BlogClientError> {
        let token = self.require_token()?;

        match self.active_transport()? {
            ActiveTransport::Http(client) => client.update_post(token, id, title, content).await,
            ActiveTransport::Grpc(client) => client.update_post(token, id, title, content).await,
        }
    }

    /// Удаляет пост.
    ///
    /// # Errors
    ///
    /// Возвращает `MissingToken`, если токен не задан. Сейчас транспортные методы еще не реализованы.
    pub async fn delete_post(&self, id: i64) -> Result<(), BlogClientError> {
        let token = self.require_token()?;

        match self.active_transport()? {
            ActiveTransport::Http(client) => client.delete_post(token, id).await,
            ActiveTransport::Grpc(client) => client.delete_post(token, id).await,
        }
    }

    /// Возвращает страницу постов.
    ///
    /// # Errors
    ///
    /// Сейчас возвращает `InvalidRequest`, пока транспортные методы не реализованы.
    pub async fn list_posts(&self, limit: u64, offset: u64) -> Result<PostPage, BlogClientError> {
        match self.active_transport()? {
            ActiveTransport::Http(client) => client.list_posts(limit, offset).await,
            ActiveTransport::Grpc(client) => client.list_posts(limit, offset).await,
        }
    }

    fn require_token(&self) -> Result<&str, BlogClientError> {
        self.get_token().ok_or(BlogClientError::MissingToken)
    }

    fn active_transport(&self) -> Result<ActiveTransport<'_>, BlogClientError> {
        match (&self.transport, &self.http_client, &self.grpc_client) {
            (Transport::Http(_), Some(client), None) => Ok(ActiveTransport::Http(client)),
            (Transport::Grpc(_), None, Some(client)) => Ok(ActiveTransport::Grpc(client)),
            _ => Err(BlogClientError::InvalidRequest(
                "client transport is not initialized".to_string(),
            )),
        }
    }
}

enum ActiveTransport<'a> {
    Http(&'a HttpClient),
    Grpc(&'a GrpcClient),
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn test_client() -> BlogClient {
        BlogClient::new(Transport::Http("http://localhost:8080".to_string()))
            .await
            .expect("HTTP client should be created")
    }

    #[tokio::test]
    async fn new_http_client_keeps_transport_and_has_no_token() {
        let client = test_client().await;

        assert_eq!(
            client.transport(),
            &Transport::Http("http://localhost:8080".to_string())
        );
        assert!(client.http_client().is_some());
        assert!(client.grpc_client().is_none());
        assert_eq!(client.get_token(), None);
    }

    #[tokio::test]
    async fn protected_methods_require_token() {
        let client = test_client().await;

        let result = client.create_post("title", "content").await;

        assert!(matches!(result, Err(BlogClientError::MissingToken)));
    }

    #[tokio::test]
    async fn set_token_updates_current_token() {
        let mut client = test_client().await;

        client.set_token("token".to_string());

        assert_eq!(client.get_token(), Some("token"));
    }
}
