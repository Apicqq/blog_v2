//! HTTP API фронтенда.

use crate::errors::{ApiError, ensure_success};
use crate::models::PostPage;
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
