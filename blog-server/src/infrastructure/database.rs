//! Подключение к базе данных.

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;

/// Создает пул подключений к базе данных.
///
/// # Errors
///
/// Возвращает ошибку `SeaORM`, если строка подключения некорректна или база данных недоступна.
pub async fn db_connection(connection_string: &str) -> Result<DatabaseConnection, DbErr> {
    let mut options = ConnectOptions::new(connection_string.to_owned());

    options
        .max_connections(10)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_mins(5))
        .max_lifetime(Duration::from_mins(30))
        .sqlx_logging(false);

    Database::connect(options).await
}
