//! Запуск HTTP и gRPC серверов приложения.

use actix_web::dev::Server as HttpServerFuture;
use actix_web::{App, HttpServer, web};
use actix_web_httpauth::middleware::HttpAuthentication;
use anyhow::Context;
use tokio::sync::oneshot;
use tonic::transport::Server as GrpcServer;
use tracing::info;

use blog_proto::generated::FILE_DESCRIPTOR_SET;
use blog_proto::generated::blog_service_server::BlogServiceServer;

use crate::application::auth_service::AuthService;
use crate::application::blog_service::BlogService;
use crate::infrastructure::config::AppConfig;
use crate::infrastructure::http::cors::build_cors;
use crate::infrastructure::http::headers::default_security_headers;
use crate::infrastructure::persistence::repositories::sea_orm_post_repository::SeaOrmPostRepository;
use crate::infrastructure::persistence::repositories::sea_orm_user_repository::SeaOrmUserRepository;
use crate::infrastructure::security::argon2_password_hasher::Argon2PasswordHasher;
use crate::infrastructure::security::jwt_token_service::JwtTokenService;
use crate::presentation::handlers::auth::configure_auth_routes;
use crate::presentation::handlers::grpc::BlogGrpcApi;
use crate::presentation::handlers::posts::configure_post_routes;
use crate::presentation::handlers::protected::configure_protected_routes;
use crate::presentation::middlewares::jwt_auth::jwt_validator;

type BlogAuthService = AuthService<SeaOrmUserRepository, Argon2PasswordHasher, JwtTokenService>;
type BlogPostService = BlogService<SeaOrmPostRepository>;

/// Зависимости серверного runtime.
#[derive(Debug, Clone)]
pub struct ServerDependencies {
    config: AppConfig,
    auth_service: web::Data<BlogAuthService>,
    blog_service: web::Data<BlogPostService>,
    token_service: web::Data<JwtTokenService>,
}

impl ServerDependencies {
    /// Создает зависимости серверного runtime.
    #[must_use]
    pub const fn new(
        config: AppConfig,
        auth_service: web::Data<BlogAuthService>,
        blog_service: web::Data<BlogPostService>,
        token_service: web::Data<JwtTokenService>,
    ) -> Self {
        Self {
            config,
            auth_service,
            blog_service,
            token_service,
        }
    }
}

/// Запускает HTTP и gRPC серверы и координирует их graceful shutdown.
///
/// # Errors
///
/// Возвращает ошибку, если адрес сервера некорректен, сервер не удалось привязать к порту
/// или один из серверов завершился с ошибкой.
pub async fn run_servers(dependencies: ServerDependencies) -> anyhow::Result<()> {
    let http_bind_address = format!("{}:{}", dependencies.config.host, dependencies.config.port);
    let grpc_bind_address = format!(
        "{}:{}",
        dependencies.config.host, dependencies.config.grpc_port
    );
    let grpc_socket_address = grpc_bind_address
        .parse()
        .with_context(|| format!("invalid gRPC bind address {grpc_bind_address}"))?;

    info!(address = %http_bind_address, "starting HTTP server");
    info!(address = %grpc_bind_address, "starting gRPC server");

    let http_server = build_http_server(&dependencies, &http_bind_address)?;
    let http_handle = http_server.handle();
    let (grpc_shutdown_tx, grpc_shutdown_rx) = oneshot::channel();
    let grpc_server = build_grpc_server(
        dependencies.auth_service.clone(),
        dependencies.blog_service.clone(),
        dependencies.token_service.clone(),
        grpc_socket_address,
        grpc_shutdown_rx,
    )?;

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

fn build_http_server(
    dependencies: &ServerDependencies,
    bind_address: &str,
) -> anyhow::Result<HttpServerFuture> {
    let config = dependencies.config.clone();
    let auth_service = dependencies.auth_service.clone();
    let blog_service = dependencies.blog_service.clone();
    let token_service = dependencies.token_service.clone();

    HttpServer::new(move || {
        App::new()
            .app_data(auth_service.clone())
            .app_data(blog_service.clone())
            .app_data(token_service.clone())
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
    .bind(bind_address)
    .with_context(|| format!("failed to bind HTTP server to {bind_address}"))
    .map(HttpServer::run)
}

fn build_grpc_server(
    auth_service: web::Data<BlogAuthService>,
    blog_service: web::Data<BlogPostService>,
    token_service: web::Data<JwtTokenService>,
    bind_address: std::net::SocketAddr,
    shutdown: oneshot::Receiver<()>,
) -> anyhow::Result<impl Future<Output = Result<(), tonic::transport::Error>>> {
    let auth_service = auth_service.into_inner();
    let blog_service = blog_service.into_inner();
    let token_service = token_service.into_inner();
    let grpc_service = BlogGrpcApi::new(
        (*auth_service).clone(),
        (*blog_service).clone(),
        (*token_service).clone(),
    );
    let reflection_grpc_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()?;

    Ok(GrpcServer::builder()
        .add_service(BlogServiceServer::new(grpc_service))
        .add_service(reflection_grpc_service)
        .serve_with_shutdown(bind_address, async {
            let _ = shutdown.await;
            info!("stopping gRPC server gracefully");
        }))
}
