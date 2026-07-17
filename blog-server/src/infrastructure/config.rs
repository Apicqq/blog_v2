//! Конфигурация серверного приложения.

use anyhow::{Context, Result};
use serde::Deserialize;

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "8080";
const DEFAULT_GRPC_PORT: &str = "50051";
const DEFAULT_CORS_ORIGINS: &str = "*";
const DEFAULT_JWT_TTL_SECONDS: &str = "3600";

/// Конфигурация серверного приложения.
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// Хост HTTP-сервера.
    pub host: String,
    /// Порт HTTP-сервера.
    pub port: u16,
    /// Порт gRPC-сервера.
    pub grpc_port: u16,
    /// Строка подключения к базе данных.
    pub database_url: String,
    /// Секрет для подписи JWT-токенов.
    pub jwt_secret: String,
    /// Время жизни JWT-токена в секундах.
    pub jwt_ttl_seconds: i64,
    /// Разрешенные CORS origins.
    #[serde(default)]
    pub cors_origins: Vec<String>,
}

impl AppConfig {
    /// Читает конфигурацию из переменных окружения.
    ///
    /// Загружает локальный `.env`, если файл существует.
    ///
    /// # Errors
    ///
    /// Возвращает ошибку, если обязательные переменные не заданы или числовые значения имеют
    /// некорректный формат.
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let host = env_or_default("HOST", DEFAULT_HOST);
        let port = env_or_default("PORT", DEFAULT_PORT)
            .parse()
            .context("invalid PORT")?;
        let grpc_port = env_or_default("GRPC_PORT", DEFAULT_GRPC_PORT)
            .parse()
            .context("invalid GRPC_PORT")?;
        let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
        let jwt_secret = std::env::var("JWT_SECRET").context("JWT_SECRET must be set")?;
        let jwt_ttl_seconds = env_or_default("JWT_TTL_SECONDS", DEFAULT_JWT_TTL_SECONDS)
            .parse()
            .context("invalid JWT_TTL_SECONDS")?;
        let cors_origins =
            parse_cors_origins(&env_or_default("CORS_ORIGINS", DEFAULT_CORS_ORIGINS));

        Ok(Self {
            host,
            port,
            grpc_port,
            database_url,
            jwt_secret,
            jwt_ttl_seconds,
            cors_origins,
        })
    }
}

fn env_or_default(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

fn parse_cors_origins(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(str::trim)
        .filter(|origin| !origin.is_empty())
        .map(ToString::to_string)
        .collect()
}
