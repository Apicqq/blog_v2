//! Интеграция JWT-аутентификации с `actix-web-httpauth`.

use actix_web::dev::ServiceRequest;
use actix_web::{Error, HttpMessage, error, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;

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
    let Some(token_service) = request.app_data::<web::Data<JwtTokenService>>() else {
        let error = error::ErrorInternalServerError("JWT token service missing");
        return Err((error, request));
    };

    let Ok(user_id) = token_service.verify(credentials.token()) else {
        let error = error::ErrorUnauthorized("invalid authorization token");
        return Err((error, request));
    };

    request
        .extensions_mut()
        .insert(AuthenticatedUser::new(user_id));

    Ok(request)
}
