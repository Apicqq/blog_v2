//! Библиотека клиентского доступа к API блога.

use blog_proto as _;

/// gRPC-клиент для API блога.
pub mod grpc_client;

/// HTTP-клиент для API блога.
pub mod http_client;
