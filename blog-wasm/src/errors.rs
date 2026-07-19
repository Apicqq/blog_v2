//! Ошибки WASM-клиента.

use gloo_net::http::Response;
use serde::Deserialize;
use thiserror::Error;

const BAD_REQUEST: u16 = 400;
const UNAUTHORIZED: u16 = 401;
const FORBIDDEN: u16 = 403;
const NOT_FOUND: u16 = 404;
const CONFLICT: u16 = 409;
const UNPROCESSABLE_ENTITY: u16 = 422;

/// Ошибка HTTP API во фронтенде.
#[derive(Debug, Error)]
pub(crate) enum ApiError {
    /// Ошибка сетевого запроса или чтения ответа.
    #[error("network error: {0}")]
    Network(#[from] gloo_net::Error),

    /// Пользователь не авторизован.
    #[error("unauthorized")]
    Unauthorized,

    /// Доступ к ресурсу запрещен.
    #[error("forbidden")]
    Forbidden,

    /// Ресурс не найден.
    #[error("not found")]
    NotFound,

    /// Конфликт состояния ресурса.
    #[error("conflict: {0}")]
    Conflict(String),

    /// Некорректный запрос.
    #[error("invalid request: {0}")]
    InvalidRequest(String),

    /// Сервер вернул ошибку.
    #[error("server error: {0}")]
    Server(String),
}

#[derive(Debug, Deserialize)]
struct ErrorBody {
    error: String,
    details: Option<serde_json::Value>,
}

impl ErrorBody {
    fn message(self) -> String {
        if let Some(message) = self
            .details
            .as_ref()
            .and_then(|details| details.get("message"))
            .and_then(serde_json::Value::as_str)
        {
            message.to_string()
        } else {
            self.error
        }
    }
}

/// Проверяет HTTP-ответ и возвращает ошибку API для неуспешного статуса.
///
/// # Errors
///
/// Возвращает `ApiError`, если сервер ответил неуспешным HTTP-статусом
/// или тело ошибки не удалось прочитать.
pub(crate) async fn ensure_success(response: &Response) -> Result<(), ApiError> {
    if response.ok() {
        return Ok(());
    }

    let status = response.status();
    let message = response_message(response).await;

    Err(api_error_from_status(status, message))
}

fn api_error_from_status(status: u16, message: String) -> ApiError {
    match status {
        BAD_REQUEST | UNPROCESSABLE_ENTITY => ApiError::InvalidRequest(message),
        UNAUTHORIZED => ApiError::Unauthorized,
        FORBIDDEN => ApiError::Forbidden,
        NOT_FOUND => ApiError::NotFound,
        CONFLICT => ApiError::Conflict(message),
        _ => ApiError::Server(message),
    }
}

async fn response_message(response: &Response) -> String {
    if let Ok(body) = response.json::<ErrorBody>().await {
        body.message()
    } else {
        let status_text = response.status_text();

        if status_text.is_empty() {
            format!("request failed with status {}", response.status())
        } else {
            status_text
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_error_from_status_maps_validation_statuses() {
        assert!(matches!(
            api_error_from_status(BAD_REQUEST, "bad request".to_string()),
            ApiError::InvalidRequest(_)
        ));
        assert!(matches!(
            api_error_from_status(UNPROCESSABLE_ENTITY, "invalid".to_string()),
            ApiError::InvalidRequest(_)
        ));
    }

    #[test]
    fn error_body_uses_details_message_when_present() {
        let body = ErrorBody {
            error: "validation error".to_string(),
            details: Some(serde_json::json!({
                "message": "title must not be empty"
            })),
        };

        assert_eq!(body.message(), "title must not be empty");
    }

    #[test]
    fn error_body_falls_back_to_error() {
        let body = ErrorBody {
            error: "email already taken".to_string(),
            details: None,
        };

        assert_eq!(body.message(), "email already taken");
    }
}
