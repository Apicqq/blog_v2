//! Ошибки клиентской библиотеки.

use thiserror::Error;

/// Ошибка клиентской библиотеки блога.
#[derive(Debug, Error)]
pub enum BlogClientError {
    /// Ошибка HTTP-запроса.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// Ошибка gRPC-статуса.
    #[error("gRPC request failed: {0}")]
    GrpcStatus(tonic::Status),

    /// Ошибка подключения gRPC-транспорта.
    #[error("gRPC transport failed: {0}")]
    GrpcTransport(#[from] tonic::transport::Error),

    /// Ресурс не найден.
    #[error("resource not found")]
    NotFound,

    /// Запрос не авторизован.
    #[error("unauthorized")]
    Unauthorized,

    /// Доступ к ресурсу запрещен.
    #[error("forbidden")]
    Forbidden,

    /// Конфликт состояния ресурса.
    #[error("conflict: {0}")]
    Conflict(String),

    /// Некорректный запрос.
    #[error("invalid request: {0}")]
    InvalidRequest(String),

    /// Для операции нужен JWT-токен.
    #[error("token is required")]
    MissingToken,
}

impl From<tonic::Status> for BlogClientError {
    fn from(status: tonic::Status) -> Self {
        match status.code() {
            tonic::Code::Unauthenticated => Self::Unauthorized,
            tonic::Code::PermissionDenied => Self::Forbidden,
            tonic::Code::NotFound => Self::NotFound,
            tonic::Code::AlreadyExists | tonic::Code::Aborted => {
                Self::Conflict(status.message().to_string())
            }
            tonic::Code::InvalidArgument
            | tonic::Code::FailedPrecondition
            | tonic::Code::OutOfRange => Self::InvalidRequest(status.message().to_string()),
            _ => Self::GrpcStatus(status),
        }
    }
}
