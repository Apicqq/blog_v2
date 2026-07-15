//! Миграции базы данных блога.

pub use sea_orm_migration::prelude::*;
use tokio as _;

mod m20260715_000001_create_table;

/// Набор миграций базы данных блога.
#[derive(Debug)]
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20260715_000001_create_table::Migration)]
    }
}
