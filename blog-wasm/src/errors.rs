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
    #[error("unauthorized: {0}")]
    Unauthorized(String),

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

impl ApiError {
    /// Возвращает сообщение ошибки для отображения пользователю.
    #[must_use]
    pub(crate) fn user_message(&self) -> String {
        match self {
            Self::Network(_) => {
                "Не удалось связаться с сервером. Проверьте, что API запущен и CORS настроен."
                    .to_string()
            }
            Self::Unauthorized(message) => user_message_from_server(message),
            Self::Forbidden => "Недостаточно прав для этого действия.".to_string(),
            Self::NotFound => "Запрошенная запись не найдена.".to_string(),
            Self::Conflict(message) | Self::InvalidRequest(message) => {
                user_message_from_server(message)
            }
            Self::Server(_) => "На сервере произошла ошибка. Попробуйте позже.".to_string(),
        }
    }
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
        UNAUTHORIZED => ApiError::Unauthorized(message),
        FORBIDDEN => ApiError::Forbidden,
        NOT_FOUND => ApiError::NotFound,
        CONFLICT => ApiError::Conflict(message),
        _ => ApiError::Server(message),
    }
}

fn user_message_from_server(message: &str) -> String {
    match message {
        "email already taken" => "Эта почта уже занята.".to_string(),
        "username already taken" => "Это имя пользователя уже занято.".to_string(),
        "invalid credentials" | "username or password is incorrect" => {
            "Неверное имя пользователя или пароль.".to_string()
        }
        "authentication is required to access this resource"
        | "invalid authorization token"
        | "Unauthorized" => "Нужно войти в аккаунт.".to_string(),
        "post title must contain at least 3 characters" => {
            "Заголовок должен содержать минимум 3 символа.".to_string()
        }
        "post title must contain at most 255 characters" => {
            "Заголовок должен содержать не больше 255 символов.".to_string()
        }
        "post content must not be empty" => "Текст поста не должен быть пустым.".to_string(),
        "post content must contain at most 10000 characters" => {
            "Текст поста должен содержать не больше 10000 символов.".to_string()
        }
        "password must contain at least 8 characters" => {
            "Пароль должен содержать минимум 8 символов.".to_string()
        }
        "username must contain at least 3 characters" => {
            "Имя пользователя должно содержать минимум 3 символа.".to_string()
        }
        "email is invalid" | "email must be valid" => "Введите корректную почту.".to_string(),
        "email must not be empty" => "Почта не должна быть пустой.".to_string(),
        _ => message.to_string(),
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
    fn api_error_user_message_maps_forbidden() {
        assert_eq!(
            ApiError::Forbidden.user_message(),
            "Недостаточно прав для этого действия."
        );
    }

    #[test]
    fn api_error_user_message_maps_invalid_credentials() {
        assert_eq!(
            ApiError::Unauthorized("username or password is incorrect".to_string()).user_message(),
            "Неверное имя пользователя или пароль."
        );
    }

    #[test]
    fn api_error_user_message_maps_auth_required() {
        assert_eq!(
            ApiError::Unauthorized(
                "authentication is required to access this resource".to_string()
            )
            .user_message(),
            "Нужно войти в аккаунт."
        );
    }

    #[test]
    fn api_error_user_message_maps_validation_message() {
        assert_eq!(
            ApiError::InvalidRequest("post content must not be empty".to_string()).user_message(),
            "Текст поста не должен быть пустым."
        );
    }

    #[test]
    fn api_error_user_message_maps_email_validation_messages() {
        assert_eq!(
            ApiError::InvalidRequest("email is invalid".to_string()).user_message(),
            "Введите корректную почту."
        );
        assert_eq!(
            ApiError::InvalidRequest("email must not be empty".to_string()).user_message(),
            "Почта не должна быть пустой."
        );
    }

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
