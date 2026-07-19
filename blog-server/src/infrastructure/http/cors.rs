use crate::infrastructure::config::AppConfig;
use actix_cors::Cors;

/// Собирает CORS middleware по конфигурации приложения.
///
/// Значение `*` разрешает любые origin без credentials. Для явного списка origin
/// включается поддержка credentials, чтобы браузер мог отправлять авторизационные заголовки.
pub fn build_cors(config: &AppConfig) -> Cors {
    let cors = Cors::default()
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allow_any_header()
        .max_age(3600);

    if config.cors_origins.iter().any(|origin| origin == "*") {
        cors.allow_any_origin()
    } else {
        config
            .cors_origins
            .iter()
            .fold(cors.supports_credentials(), |cors, origin| {
                cors.allowed_origin(origin)
            })
    }
}
