//! Серверное приложение блога.

use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use anyhow::{Context, ensure};
use chrono::Duration;
use tracing::info;

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

use application::auth_service::AuthService;
use blog_proto as _;
use infrastructure::config::AppConfig;
use infrastructure::database::db_connection;
use infrastructure::persistence::repositories::sea_orm_user_repository::SeaOrmUserRepository;
use infrastructure::security::argon2_password_hasher::Argon2PasswordHasher;
use infrastructure::security::jwt_token_service::JwtTokenService;
use presentation::handlers::auth::configure_auth_routes;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    infrastructure::telemetry::init_logging();

    let config = AppConfig::from_env()?;
    ensure!(
        config.jwt_ttl_seconds > 0,
        "JWT_TTL_SECONDS must be greater than zero"
    );

    let db = db_connection(&config.database_url)
        .await
        .context("failed to connect to database")?;
    let user_repository = SeaOrmUserRepository::new(db);
    let password_hasher = Argon2PasswordHasher::new();
    let token_service = JwtTokenService::new(
        config.jwt_secret.clone(),
        Duration::seconds(config.jwt_ttl_seconds),
    );
    let auth_service = web::Data::new(AuthService::new(
        Arc::new(user_repository),
        Arc::new(password_hasher),
        Arc::new(token_service),
    ));
    let bind_address = format!("{}:{}", config.host, config.port);

    info!(address = %bind_address, "starting HTTP server");

    HttpServer::new(move || {
        App::new()
            .app_data(auth_service.clone())
            .service(web::scope("/api").configure(configure_auth_routes))
    })
    .bind(&bind_address)
    .with_context(|| format!("failed to bind HTTP server to {bind_address}"))?
    .run()
    .await
    .context("HTTP server failed")
}
