//! Protected/debug handlers.

use actix_web::{HttpResponse, get, web};
use serde::Serialize;

use crate::presentation::auth::AuthenticatedUser;

/// Ответ проверки аутентификации.
#[derive(Debug, Serialize)]
struct AuthenticationCheckResponse {
    /// Признак успешной аутентификации.
    authenticated: bool,
}

/// Настраивает protected routes.
pub fn configure_protected_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(current_user);
}

/// Проверяет, что запрос прошел JWT-аутентификацию.
#[get("/me")]
async fn current_user(_user: AuthenticatedUser) -> HttpResponse {
    HttpResponse::Ok().json(AuthenticationCheckResponse {
        authenticated: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::http::StatusCode;
    use actix_web::http::header::AUTHORIZATION;
    use actix_web::{App, test, web};
    use chrono::Duration;
    use serde_json::json;
    use uuid::Uuid;

    use crate::application::ports::token_service::TokenService;
    use crate::infrastructure::security::jwt_token_service::JwtTokenService;
    use crate::presentation::middlewares::jwt_auth::JwtAuthMiddleware;

    const JWT_SECRET: &str = "test-secret";

    #[actix_web::test]
    async fn current_user_requires_authorization_header() {
        let token_service = JwtTokenService::new(JWT_SECRET.to_string(), Duration::minutes(15));
        let app = test::init_service(
            App::new().service(
                web::scope("")
                    .wrap(JwtAuthMiddleware::new(token_service))
                    .configure(configure_protected_routes),
            ),
        )
        .await;

        let request = test::TestRequest::get().uri("/me").to_request();
        let error = test::try_call_service(&app, request)
            .await
            .expect_err("request should be rejected");

        assert_eq!(
            error.as_response_error().status_code(),
            StatusCode::UNAUTHORIZED
        );
    }

    #[actix_web::test]
    async fn current_user_accepts_valid_token() {
        let token_service = JwtTokenService::new(JWT_SECRET.to_string(), Duration::minutes(15));
        let token = token_service
            .issue_new(Uuid::new_v4())
            .expect("token should be issued");
        let app = test::init_service(
            App::new().service(
                web::scope("")
                    .wrap(JwtAuthMiddleware::new(token_service))
                    .configure(configure_protected_routes),
            ),
        )
        .await;

        let request = test::TestRequest::get()
            .uri("/me")
            .insert_header((AUTHORIZATION, format!("Bearer {token}")))
            .to_request();
        let response = test::call_service(&app, request).await;
        let status = response.status();
        let body: serde_json::Value = test::read_body_json(response).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, json!({ "authenticated": true }));
    }
}
