//! gRPC-клиент для взаимодействия с API блога.

use blog_proto::generated::blog_service_client::BlogServiceClient;
use tonic::transport::Channel;

use crate::errors::BlogClientError;
use crate::models::{AuthResponse, Post, PostPage};

/// gRPC-клиент блога.
#[derive(Debug, Clone)]
pub struct GrpcClient {
    endpoint: String,
    client: BlogServiceClient<Channel>,
}

impl GrpcClient {
    /// Подключается к gRPC API блога.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если соединение с gRPC API не удалось установить.
    pub async fn connect(endpoint: String) -> Result<Self, BlogClientError> {
        let client = BlogServiceClient::connect(endpoint.clone()).await?;

        Ok(Self { endpoint, client })
    }

    /// Возвращает endpoint gRPC API.
    #[must_use]
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Возвращает внутренний tonic-клиент.
    #[must_use]
    pub const fn inner(&self) -> &BlogServiceClient<Channel> {
        &self.client
    }

    /// Регистрирует пользователя через gRPC API.
    ///
    /// # Errors
    ///
    /// Сейчас возвращает `InvalidRequest`, пока gRPC-запросы не реализованы.
    pub async fn register(
        &self,
        _username: &str,
        _email: &str,
        _password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        std::future::ready(()).await;
        Err(BlogClientError::not_implemented("grpc register"))
    }

    /// Выполняет вход через gRPC API.
    ///
    /// # Errors
    ///
    /// Сейчас возвращает `InvalidRequest`, пока gRPC-запросы не реализованы.
    pub async fn login(
        &self,
        _username: &str,
        _password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        std::future::ready(()).await;
        Err(BlogClientError::not_implemented("grpc login"))
    }

    /// Создает пост через gRPC API.
    ///
    /// # Errors
    ///
    /// Сейчас возвращает `InvalidRequest`, пока gRPC-запросы не реализованы.
    pub async fn create_post(
        &self,
        _token: &str,
        _title: &str,
        _content: &str,
    ) -> Result<Post, BlogClientError> {
        std::future::ready(()).await;
        Err(BlogClientError::not_implemented("grpc create_post"))
    }

    /// Возвращает пост через gRPC API.
    ///
    /// # Errors
    ///
    /// Сейчас возвращает `InvalidRequest`, пока gRPC-запросы не реализованы.
    pub async fn get_post(&self, _id: i64) -> Result<Post, BlogClientError> {
        std::future::ready(()).await;
        Err(BlogClientError::not_implemented("grpc get_post"))
    }

    /// Обновляет пост через gRPC API.
    ///
    /// # Errors
    ///
    /// Сейчас возвращает `InvalidRequest`, пока gRPC-запросы не реализованы.
    pub async fn update_post(
        &self,
        _token: &str,
        _id: i64,
        _title: &str,
        _content: &str,
    ) -> Result<Post, BlogClientError> {
        std::future::ready(()).await;
        Err(BlogClientError::not_implemented("grpc update_post"))
    }

    /// Удаляет пост через gRPC API.
    ///
    /// # Errors
    ///
    /// Сейчас возвращает `InvalidRequest`, пока gRPC-запросы не реализованы.
    pub async fn delete_post(&self, _token: &str, _id: i64) -> Result<(), BlogClientError> {
        std::future::ready(()).await;
        Err(BlogClientError::not_implemented("grpc delete_post"))
    }

    /// Возвращает страницу постов через gRPC API.
    ///
    /// # Errors
    ///
    /// Сейчас возвращает `InvalidRequest`, пока gRPC-запросы не реализованы.
    pub async fn list_posts(&self, _limit: u64, _offset: u64) -> Result<PostPage, BlogClientError> {
        std::future::ready(()).await;
        Err(BlogClientError::not_implemented("grpc list_posts"))
    }
}
