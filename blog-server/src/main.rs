//! Серверное приложение блога.

use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use anyhow::{Context, ensure};
use chrono::Duration;
use tokio::sync::oneshot;
use tonic::transport::Server;
use tracing::info;

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

use actix_web_httpauth::middleware::HttpAuthentication;
use application::auth_service::AuthService;
use application::blog_service::BlogService;
use blog_proto::generated::FILE_DESCRIPTOR_SET;
use blog_proto::generated::blog_service_server::BlogServiceServer;
use infrastructure::config::AppConfig;
use infrastructure::database::db_connection;
use infrastructure::http::cors::build_cors;
use infrastructure::http::headers::default_security_headers;
use infrastructure::persistence::repositories::sea_orm_post_repository::SeaOrmPostRepository;
use infrastructure::persistence::repositories::sea_orm_user_repository::SeaOrmUserRepository;
use infrastructure::security::argon2_password_hasher::Argon2PasswordHasher;
use infrastructure::security::jwt_token_service::JwtTokenService;
use presentation::handlers::auth::configure_auth_routes;
use presentation::handlers::grpc::BlogGrpcApi;
use presentation::handlers::posts::configure_post_routes;
use presentation::handlers::protected::configure_protected_routes;
use presentation::middlewares::jwt_auth::jwt_validator;

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
    let http_bind_address = format!("{}:{}", config.host, config.port);
    let grpc_bind_address = format!("{}:{}", config.host, config.grpc_port);
    let grpc_socket_address = grpc_bind_address
        .parse()
        .with_context(|| format!("invalid gRPC bind address {grpc_bind_address}"))?;
    let grpc_service = BlogGrpcApi::new(
        auth_service.get_ref().clone(),
        blog_service.get_ref().clone(),
        token_service_data.get_ref().clone(),
    );

    info!(address = %http_bind_address, "starting HTTP server");
    info!(address = %grpc_bind_address, "starting gRPC server");

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(auth_service.clone())
            .app_data(blog_service.clone())
            .app_data(token_service_data.clone())
            .wrap(build_cors(&config))
            .wrap(default_security_headers())
            .service(
                web::scope("/api")
                    .configure(configure_auth_routes)
                    .configure(configure_post_routes)
                    .service(
                        web::scope("")
                            .wrap(HttpAuthentication::bearer(jwt_validator))
                            .configure(configure_protected_routes),
                    ),
            )
    })
    .disable_signals()
    .shutdown_timeout(10)
    .bind(&http_bind_address)
    .with_context(|| format!("failed to bind HTTP server to {http_bind_address}"))?
    .run();
    let http_handle = http_server.handle();

    let reflection_grpc_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()?;
    let (grpc_shutdown_tx, grpc_shutdown_rx) = oneshot::channel();

    let grpc_server = Server::builder()
        .add_service(BlogServiceServer::new(grpc_service))
        .add_service(reflection_grpc_service)
        .serve_with_shutdown(grpc_socket_address, async {
            let _ = grpc_shutdown_rx.await;
            info!("stopping gRPC server gracefully");
        });

    let mut http_task = tokio::spawn(http_server);
    let mut grpc_task = tokio::spawn(grpc_server);
    let mut grpc_shutdown_tx = Some(grpc_shutdown_tx);

    tokio::select! {
        result = &mut http_task => {
            let http_result = result.context("HTTP server task failed")?;
            if let Some(shutdown) = grpc_shutdown_tx.take() {
                let _ = shutdown.send(());
            }
            grpc_task.await.context("gRPC server task failed")?.context("gRPC server failed")?;
            http_result.context("HTTP server failed")?;
        }
        result = &mut grpc_task => {
            let grpc_result = result.context("gRPC server task failed")?;
            http_handle.stop(true).await;
            http_task.await.context("HTTP server task failed")?.context("HTTP server failed")?;
            grpc_result.context("gRPC server failed")?;
        }
        result = tokio::signal::ctrl_c() => {
            result.context("failed to listen for shutdown signal")?;
            info!("shutdown signal received");
            if let Some(shutdown) = grpc_shutdown_tx.take() {
                let _ = shutdown.send(());
            }
            http_handle.stop(true).await;
            http_task.await.context("HTTP server task failed")?.context("HTTP server failed")?;
            grpc_task.await.context("gRPC server task failed")?.context("gRPC server failed")?;
        }
    }

    Ok(())
}
