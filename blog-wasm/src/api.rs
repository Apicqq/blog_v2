//! HTTP API фронтенда.

use crate::errors::{ApiError, ensure_success};
use crate::models::{
    AuthResponse, CreatePostRequest, LoginRequest, Post, PostPage, RegisterRequest,
    UpdatePostRequest, User,
};
use crate::storage::save_token_to_storage;
use gloo_net::http::Request;

const DEFAULT_API_BASE_URL: &str = "http://127.0.0.1:8080/api";

/// Загружает список постов.
///
/// # Errors
///
/// Возвращает ошибку, если HTTP-запрос завершился неуспешно
/// или ответ не удалось десериализовать.
pub(crate) async fn list_posts(limit: u64, offset: u64) -> Result<PostPage, ApiError> {
    let response = Request::get(&api_url(&format!("/posts?limit={limit}&offset={offset}")))
        .send()
        .await?;

    ensure_success(&response).await?;

    Ok(response.json::<PostPage>().await?)
}

/// Загружает пост по идентификатору.
///
/// # Errors
///
/// Возвращает ошибку, если HTTP-запрос завершился неуспешно
/// или ответ не удалось десериализовать.
pub(crate) async fn get_post(id: i64) -> Result<Post, ApiError> {
    let response = Request::get(&api_url(&format!("/posts/{id}")))
        .send()
        .await?;

    ensure_success(&response).await?;

    Ok(response.json::<Post>().await?)
}

/// Регистрирует пользователя и сохраняет JWT-токен в `localStorage`.
///
/// # Errors
///
/// Возвращает ошибку, если HTTP-запрос завершился неуспешно
/// или ответ не удалось десериализовать.
pub(crate) async fn register(
    username: &str,
    email: &str,
    password: &str,
) -> Result<AuthResponse, ApiError> {
    let response = Request::post(&api_url("/auth/register"))
        .json(&RegisterRequest {
            username,
            email,
            password,
        })?
        .send()
        .await?;

    ensure_success(&response).await?;

    let auth = response.json::<AuthResponse>().await?;
    save_token_to_storage(&auth.token);

    Ok(auth)
}

/// Выполняет вход и сохраняет JWT-токен в `localStorage`.
///
/// # Errors
///
/// Возвращает ошибку, если HTTP-запрос завершился неуспешно
/// или ответ не удалось десериализовать.
pub(crate) async fn login(username: &str, password: &str) -> Result<AuthResponse, ApiError> {
    let response = Request::post(&api_url("/auth/login"))
        .json(&LoginRequest { username, password })?
        .send()
        .await?;

    ensure_success(&response).await?;

    let auth = response.json::<AuthResponse>().await?;
    save_token_to_storage(&auth.token);

    Ok(auth)
}

/// Возвращает текущего пользователя по сохраненному токену.
///
/// # Errors
///
/// Возвращает ошибку, если токен отклонен, HTTP-запрос завершился неуспешно
/// или ответ не удалось десериализовать.
pub(crate) async fn current_user(token: &str) -> Result<User, ApiError> {
    let response = Request::get(&api_url("/me"))
        .header("Authorization", &bearer_token(token))
        .send()
        .await?;

    ensure_success(&response).await?;

    Ok(response.json::<User>().await?)
}

/// Создает пост от имени текущего пользователя.
///
/// # Errors
///
/// Возвращает ошибку, если токен отклонен, HTTP-запрос завершился неуспешно
/// или ответ не удалось десериализовать.
pub(crate) async fn create_post(token: &str, title: &str, content: &str) -> Result<Post, ApiError> {
    let response = Request::post(&api_url("/posts"))
        .header("Authorization", &bearer_token(token))
        .json(&CreatePostRequest { title, content })?
        .send()
        .await?;

    ensure_success(&response).await?;

    Ok(response.json::<Post>().await?)
}

/// Обновляет пост текущего пользователя.
///
/// # Errors
///
/// Возвращает ошибку, если токен отклонен, HTTP-запрос завершился неуспешно
/// или ответ не удалось десериализовать.
pub(crate) async fn update_post(
    token: &str,
    id: i64,
    title: &str,
    content: &str,
) -> Result<Post, ApiError> {
    let response = Request::put(&api_url(&format!("/posts/{id}")))
        .header("Authorization", &bearer_token(token))
        .json(&UpdatePostRequest { title, content })?
        .send()
        .await?;

    ensure_success(&response).await?;

    Ok(response.json::<Post>().await?)
}

/// Удаляет пост текущего пользователя.
///
/// # Errors
///
/// Возвращает ошибку, если токен отклонен или HTTP-запрос завершился неуспешно.
pub(crate) async fn delete_post(token: &str, id: i64) -> Result<(), ApiError> {
    let response = Request::delete(&api_url(&format!("/posts/{id}")))
        .header("Authorization", &bearer_token(token))
        .send()
        .await?;

    ensure_success(&response).await?;

    Ok(())
}

fn api_base_url() -> &'static str {
    option_env!("BLOG_API_BASE_URL").unwrap_or(DEFAULT_API_BASE_URL)
}

fn api_url(path: &str) -> String {
    format!("{}{}", api_base_url().trim_end_matches('/'), path)
}

fn bearer_token(token: &str) -> String {
    format!("Bearer {token}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_url_handles_base_url_without_trailing_slash() {
        assert_eq!(api_url("/posts"), format!("{}/posts", api_base_url()));
    }
}
