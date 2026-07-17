//! gRPC-клиент для взаимодействия с API блога.

use blog_proto::generated::blog_service_client::BlogServiceClient;
use blog_proto::generated::{
    CreatePostRequest, DeletePostRequest, GetPostRequest, ListPostsRequest, LoginRequest,
    RegisterRequest, UpdatePostRequest,
};
use tonic::Request;
use tonic::metadata::MetadataValue;
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

    /// Регистрирует пользователя через gRPC API.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если gRPC-запрос не выполнен, сервер отклонил данные
    /// регистрации или ответ не удалось преобразовать в клиентскую модель.
    pub async fn register(
        &mut self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        let request = Request::new(RegisterRequest {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
        });
        let response = self.client.register(request).await?;
        Ok(response.into_inner().into())
    }

    /// Выполняет вход через gRPC API.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если gRPC-запрос не выполнен, учетные данные отклонены
    /// или ответ не удалось преобразовать в клиентскую модель.
    pub async fn login(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<AuthResponse, BlogClientError> {
        let request = Request::new(LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        });
        let response = self.client.login(request).await?;
        Ok(response.into_inner().into())
    }

    /// Создает пост через gRPC API.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если gRPC-запрос не выполнен, токен отклонен, данные поста
    /// не прошли валидацию или сервер вернул ошибочный статус.
    pub async fn create_post(
        &mut self,
        token: &str,
        title: &str,
        content: &str,
    ) -> Result<Post, BlogClientError> {
        let request = authenticated_request(
            CreatePostRequest {
                title: title.to_string(),
                content: content.to_string(),
            },
            token,
        )?;
        let response = self.client.create_post(request).await?;

        Ok(response.into_inner().into())
    }

    /// Возвращает пост через gRPC API.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если gRPC-запрос не выполнен, пост не найден или сервер
    /// вернул ошибочный статус.
    pub async fn get_post(&mut self, id: i64) -> Result<Post, BlogClientError> {
        let response = self.client.get_post(GetPostRequest { id }).await?;

        Ok(response.into_inner().into())
    }

    /// Обновляет пост через gRPC API.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если gRPC-запрос не выполнен, токен отклонен, доступ запрещен,
    /// пост не найден, данные не прошли валидацию или сервер вернул ошибочный статус.
    pub async fn update_post(
        &mut self,
        token: &str,
        id: i64,
        title: &str,
        content: &str,
    ) -> Result<Post, BlogClientError> {
        let request = authenticated_request(
            UpdatePostRequest {
                id,
                title: title.to_string(),
                content: content.to_string(),
            },
            token,
        )?;
        let response = self.client.update_post(request).await?;

        Ok(response.into_inner().into())
    }

    /// Удаляет пост через gRPC API.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если gRPC-запрос не выполнен, токен отклонен, доступ запрещен,
    /// пост не найден или сервер вернул ошибочный статус.
    pub async fn delete_post(&mut self, token: &str, id: i64) -> Result<(), BlogClientError> {
        let request = authenticated_request(DeletePostRequest { id }, token)?;
        self.client.delete_post(request).await?;
        Ok(())
    }

    /// Возвращает страницу постов через gRPC API.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если gRPC-запрос не выполнен, сервер вернул ошибочный статус
    /// или ответ не удалось преобразовать в клиентскую модель.
    pub async fn list_posts(
        &mut self,
        limit: u64,
        offset: u64,
    ) -> Result<PostPage, BlogClientError> {
        let response = self
            .client
            .list_posts(ListPostsRequest { limit, offset })
            .await?;

        Ok(response.into_inner().into())
    }
}

fn authenticated_request<T>(message: T, token: &str) -> Result<Request<T>, BlogClientError> {
    let mut request = Request::new(message);
    set_authorization_metadata(&mut request, token)?;

    Ok(request)
}

fn set_authorization_metadata<T>(
    request: &mut Request<T>,
    token: &str,
) -> Result<(), BlogClientError> {
    let value = format!("Bearer {token}")
        .parse::<MetadataValue<_>>()
        .map_err(|err| BlogClientError::InvalidRequest(err.to_string()))?;
    request.metadata_mut().insert("authorization", value);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_authorization_metadata_adds_bearer_token() {
        let mut request = Request::new(GetPostRequest { id: 1 });

        set_authorization_metadata(&mut request, "token").expect("metadata should be set");

        assert_eq!(
            request
                .metadata()
                .get("authorization")
                .and_then(|value| value.to_str().ok()),
            Some("Bearer token")
        );
    }
}
