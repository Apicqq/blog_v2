//! HTTP-клиент для взаимодействия с API блога.

/// Заглушка HTTP-клиента блога.
#[derive(Debug, Default)]
pub struct HttpClient;

impl HttpClient {
    /// Создает новый HTTP-клиент блога.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}
