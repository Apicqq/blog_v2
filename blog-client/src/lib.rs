//! Библиотека клиентского доступа к API блога.

/// Ошибки клиентской библиотеки.
pub mod errors;
/// gRPC-клиент для API блога.
pub mod grpc_client;
/// HTTP-клиент для API блога.
pub mod http_client;
/// Клиентские модели API блога.
pub mod models;
/// Выбор транспорта и фасад клиентской библиотеки.
pub mod transport;

pub use errors::BlogClientError;
pub use transport::{BlogClient, Transport};
