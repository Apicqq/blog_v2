//! HTTP API фронтенда.

use crate::errors::{ApiError, ensure_success};
use crate::models::{
    AuthResponse, CreatePostRequest, LoginRequest, Post, PostPage, RegisterRequest,
    UpdatePostRequest, User,
};
use crate::storage::save_token_to_storage;
use gloo_net::http::Request;

const API_BASE_URL: &str = "http://127.0.0.1:8080/api";

/// Загружает список постов.
///
/// # Errors
///
/// Возвращает ошибку, если HTTP-запрос завершился неуспешно
/// или ответ не удалось десериализовать.
pub(crate) async fn list_posts(limit: u64, offset: u64) -> Result<PostPage, ApiError> {
    let response = Request::get(&format!(
        "{API_BASE_URL}/posts?limit={limit}&offset={offset}"
    ))
    .send()
    .await?;

    ensure_success(&response).await?;

    Ok(response.json::<PostPage>().await?)
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
    let response = Request::post(&format!("{API_BASE_URL}/auth/register"))
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
    let response = Request::post(&format!("{API_BASE_URL}/auth/login"))
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
    let response = Request::get(&format!("{API_BASE_URL}/me"))
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
    let response = Request::post(&format!("{API_BASE_URL}/posts"))
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
    let response = Request::put(&format!("{API_BASE_URL}/posts/{id}"))
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
    let response = Request::delete(&format!("{API_BASE_URL}/posts/{id}"))
        .header("Authorization", &bearer_token(token))
        .send()
        .await?;

    ensure_success(&response).await?;

    Ok(())
}

fn bearer_token(token: &str) -> String {
    format!("Bearer {token}")
}
