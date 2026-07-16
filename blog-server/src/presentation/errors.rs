//! Преобразование доменных ошибок в HTTP-ответы.

use actix_web::body::BoxBody;
use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use serde_json::json;

use crate::domain::errors::DomainError;

#[derive(Serialize)]
struct ErrorBody<'a> {
    error: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
}

impl ResponseError for DomainError {
    fn status_code(&self) -> StatusCode {
        match self {
            DomainError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            DomainError::Validation(_) => StatusCode::BAD_REQUEST,
            DomainError::InvalidCredentials | DomainError::Unauthorized => StatusCode::UNAUTHORIZED,
            DomainError::Forbidden => StatusCode::FORBIDDEN,
            DomainError::PostNotFound(_) | DomainError::UserNotFound(_) => StatusCode::NOT_FOUND,
            DomainError::UsernameAlreadyTaken | DomainError::EmailAlreadyTaken => {
                StatusCode::CONFLICT
            }
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let message = self.to_string();
        let details = match self {
            DomainError::Validation(message) => Some(json!({ "message": message })),
            DomainError::Internal(_)
            | DomainError::UserNotFound(_)
            | DomainError::UsernameAlreadyTaken
            | DomainError::EmailAlreadyTaken => None,
            DomainError::Forbidden => Some(json!({ "reason": "forbidden" })),
            DomainError::Unauthorized => Some(json!({ "reason": "unauthorized" })),
            DomainError::InvalidCredentials => Some(json!({ "reason": "invalid_credentials" })),
            DomainError::PostNotFound(post_id) => Some(json!({ "post_id": post_id })),
        };
        let body = ErrorBody {
            error: &message,
            details,
        };

        HttpResponse::build(self.status_code()).json(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_maps_to_bad_request() {
        let error = DomainError::Validation("bad input".to_string());

        assert_eq!(error.status_code(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn username_already_taken_maps_to_conflict() {
        let error = DomainError::UsernameAlreadyTaken;

        assert_eq!(error.status_code(), StatusCode::CONFLICT);
    }

    #[actix_web::test]
    async fn email_already_taken_response_has_no_duplicated_details() {
        let error = DomainError::EmailAlreadyTaken;
        let response = error.error_response();
        let status = response.status();
        let body = actix_web::body::to_bytes(response.into_body())
            .await
            .expect("body should be readable");
        let body: serde_json::Value =
            serde_json::from_slice(&body).expect("body should be valid json");

        assert_eq!(status, StatusCode::CONFLICT);
        assert_eq!(body, json!({ "error": "email already taken" }));
    }

    #[test]
    fn post_not_found_maps_to_not_found() {
        let error = DomainError::PostNotFound(42);

        assert_eq!(error.status_code(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn invalid_credentials_maps_to_unauthorized() {
        let error = DomainError::InvalidCredentials;

        assert_eq!(error.status_code(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn internal_maps_to_internal_server_error() {
        let error = DomainError::Internal("database failed".to_string());

        assert_eq!(error.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
