//! Настройка телеметрии серверного приложения.

use tracing_subscriber::{EnvFilter, fmt};

const DEFAULT_LOG_FILTER: &str = "info,blog_server=debug";

/// Инициализирует логирование приложения.
///
/// По умолчанию используется уровень `info`, а для серверного крейта — `debug`.
/// Фильтр можно переопределить через переменную окружения `RUST_LOG`.
pub fn init_logging() {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEFAULT_LOG_FILTER));

    let subscriber = fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_level(true)
        .with_timer(fmt::time::UtcTime::rfc_3339())
        .json()
        .finish();

    let _ = tracing::subscriber::set_global_default(subscriber);
}
