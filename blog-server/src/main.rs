//! Серверное приложение блога.

use std::sync::Arc;

use actix_web::web;
use anyhow::{Context, ensure};
use chrono::Duration;

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

use application::auth_service::AuthService;
use application::blog_service::BlogService;
use infrastructure::config::AppConfig;
use infrastructure::database::db_connection;
use infrastructure::persistence::repositories::sea_orm_post_repository::SeaOrmPostRepository;
use infrastructure::persistence::repositories::sea_orm_user_repository::SeaOrmUserRepository;
use infrastructure::security::argon2_password_hasher::Argon2PasswordHasher;
use infrastructure::security::jwt_token_service::JwtTokenService;
use infrastructure::server::{ServerDependencies, run_servers};

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
    let user_repository = SeaOrmUserRepository::new(db.clone());
    let post_repository = SeaOrmPostRepository::new(db);
    let password_hasher = Argon2PasswordHasher::new();
    let token_service = JwtTokenService::new(
        config.jwt_secret.clone(),
        Duration::seconds(config.jwt_ttl_seconds),
    );

    let token_service_data = web::Data::new(token_service.clone());
    let auth_service = web::Data::new(AuthService::new(
        Arc::new(user_repository),
        Arc::new(password_hasher),
        Arc::new(token_service),
    ));
    let blog_service = web::Data::new(BlogService::new(Arc::new(post_repository)));
    let dependencies =
        ServerDependencies::new(config, auth_service, blog_service, token_service_data);

    run_servers(dependencies).await
}
