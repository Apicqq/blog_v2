//! gRPC-клиент для взаимодействия с API блога.

/// Заглушка gRPC-клиента блога.
#[derive(Debug, Default)]
pub struct GrpcClient;

impl GrpcClient {
    /// Создает новый gRPC-клиент блога.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}
