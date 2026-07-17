//! Интеграция JWT-аутентификации с `actix-web-httpauth`.

use actix_web::dev::ServiceRequest;
use actix_web::{Error, HttpMessage, error, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use tracing::{debug, warn};

use crate::application::ports::token_service::TokenService;
use crate::infrastructure::security::jwt_token_service::JwtTokenService;
use crate::presentation::auth::AuthenticatedUser;

/// Проверяет bearer-токен и кладет текущего пользователя в request extensions.
///
/// # Errors
///
/// Возвращает `401 Unauthorized`, если токен отсутствует, некорректен или истек. Возвращает
/// `500 Internal Server Error`, если сервис проверки токенов не зарегистрирован в приложении.
pub async fn jwt_validator(
    request: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let path = request.path().to_string();
    let Some(token_service) = request.app_data::<web::Data<JwtTokenService>>() else {
        warn!(path = %path, "JWT token service is missing");
        let error = error::ErrorInternalServerError("JWT token service missing");
        return Err((error, request));
    };

    let Ok(user_id) = token_service.verify(credentials.token()) else {
        warn!(path = %path, "JWT token rejected");
        let error = error::ErrorUnauthorized("invalid authorization token");
        return Err((error, request));
    };

    debug!(path = %path, user_id = %user_id, "JWT token accepted");
    request
        .extensions_mut()
        .insert(AuthenticatedUser::new(user_id));

    Ok(request)
}
