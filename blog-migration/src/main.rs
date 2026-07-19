//! CLI для управления миграциями базы данных блога.

use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    cli::run_cli(blog_migration::Migrator).await;
}
