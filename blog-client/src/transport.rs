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
    transport: ClientTransport,
    token: Option<String>,
}

#[derive(Debug, Clone)]
enum ClientTransport {
    Http(HttpClient),
    Grpc(GrpcClient),
}

impl BlogClient {
    /// Создает клиент для выбранного транспорта.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если gRPC-соединение не удалось установить.
    pub async fn new(transport: Transport) -> Result<Self, BlogClientError> {
        let transport = match transport {
            Transport::Http(base_url) => ClientTransport::Http(HttpClient::new(&base_url)),
            Transport::Grpc(endpoint) => {
                ClientTransport::Grpc(GrpcClient::connect(endpoint).await?)
            }
        };

        Ok(Self {
            transport,
            token: None,
        })
    }

    /// Возвращает транспорт клиента.
    #[must_use]
    pub fn transport(&self) -> Transport {
        match &self.transport {
            ClientTransport::Http(client) => Transport::Http(client.base_url().to_string()),
            ClientTransport::Grpc(client) => Transport::Grpc(client.endpoint().to_string()),
        }
    }

    /// Возвращает HTTP-клиент, если выбран HTTP-транспорт.
    #[must_use]
    pub const fn http_client(&self) -> Option<&HttpClient> {
        match &self.transport {
            ClientTransport::Http(client) => Some(client),
            ClientTransport::Grpc(_) => None,
        }
    }

    /// Возвращает gRPC-клиент, если выбран gRPC-транспорт.
    #[must_use]
    pub const fn grpc_client(&self) -> Option<&GrpcClient> {
        match &self.transport {
            ClientTransport::Http(_) => None,
            ClientTransport::Grpc(client) => Some(client),
        }
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

    /// Регистрирует пользователя через выбранный транспорт и сохраняет полученный токен.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если запрос не выполнен, сервер отклонил данные регистрации
    /// или ответ не удалось преобразовать в клиентскую модель.
    pub async fn register(
        &mut self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        let session = match &mut self.transport {
            ClientTransport::Http(client) => client.register(username, email, password).await?,
            ClientTransport::Grpc(client) => client.register(username, email, password).await?,
        };
        self.set_token(session.token.clone());

        Ok(session)
    }

    /// Выполняет вход пользователя через выбранный транспорт и сохраняет полученный токен.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если запрос не выполнен, учетные данные отклонены
    /// или ответ не удалось преобразовать в клиентскую модель.
    pub async fn login(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        let session = match &mut self.transport {
            ClientTransport::Http(client) => client.login(username, password).await?,
            ClientTransport::Grpc(client) => client.login(username, password).await?,
        };
        self.set_token(session.token.clone());

        Ok(session)
    }

    /// Создает пост через выбранный транспорт.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если токен не задан, запрос не выполнен, токен отклонен,
    /// данные поста не прошли валидацию или ответ не удалось преобразовать в клиентскую модель.
    pub async fn create_post(
        &mut self,
        title: &str,
        content: &str,
    ) -> Result<Post, BlogClientError> {
        let token = self.require_token()?.to_string();

        match &mut self.transport {
            ClientTransport::Http(client) => client.create_post(&token, title, content).await,
            ClientTransport::Grpc(client) => client.create_post(&token, title, content).await,
        }
    }

    /// Возвращает пост по идентификатору через выбранный транспорт.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если запрос не выполнен, пост не найден, сервер вернул
    /// ошибочный статус или ответ не удалось преобразовать в клиентскую модель.
    pub async fn get_post(&mut self, id: i64) -> Result<Post, BlogClientError> {
        match &mut self.transport {
            ClientTransport::Http(client) => client.get_post(id).await,
            ClientTransport::Grpc(client) => client.get_post(id).await,
        }
    }

    /// Обновляет пост через выбранный транспорт.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если токен не задан, запрос не выполнен, доступ запрещен,
    /// пост не найден, данные не прошли валидацию или ответ не удалось преобразовать.
    pub async fn update_post(
        &mut self,
        id: i64,
        title: &str,
        content: &str,
    ) -> Result<Post, BlogClientError> {
        let token = self.require_token()?.to_string();

        match &mut self.transport {
            ClientTransport::Http(client) => client.update_post(&token, id, title, content).await,
            ClientTransport::Grpc(client) => client.update_post(&token, id, title, content).await,
        }
    }

    /// Удаляет пост через выбранный транспорт.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если токен не задан, запрос не выполнен, доступ запрещен,
    /// пост не найден или сервер вернул ошибочный статус.
    pub async fn delete_post(&mut self, id: i64) -> Result<(), BlogClientError> {
        let token = self.require_token()?.to_string();

        match &mut self.transport {
            ClientTransport::Http(client) => client.delete_post(&token, id).await,
            ClientTransport::Grpc(client) => client.delete_post(&token, id).await,
        }
    }

    /// Возвращает страницу постов через выбранный транспорт.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если запрос не выполнен, сервер вернул ошибочный статус
    /// или ответ не удалось преобразовать в клиентскую модель.
    pub async fn list_posts(
        &mut self,
        limit: u64,
        offset: u64,
    ) -> Result<PostPage, BlogClientError> {
        match &mut self.transport {
            ClientTransport::Http(client) => client.list_posts(limit, offset).await,
            ClientTransport::Grpc(client) => client.list_posts(limit, offset).await,
        }
    }

    fn require_token(&self) -> Result<&str, BlogClientError> {
        self.get_token().ok_or(BlogClientError::MissingToken)
    }
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
            Transport::Http("http://localhost:8080".to_string())
        );
        assert!(client.http_client().is_some());
        assert!(client.grpc_client().is_none());
        assert_eq!(client.get_token(), None);
    }

    #[tokio::test]
    async fn protected_methods_require_token() {
        let mut client = test_client().await;

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
