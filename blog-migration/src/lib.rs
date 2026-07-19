//! Миграции базы данных блога.

pub use sea_orm_migration::prelude::*;

// `tokio` используется бинарем через `#[tokio::main]`; явную ссылку беру,
//  чтобы clippy не считал зависимость неиспользуемой.
//  Через as _ не очень красиво :)
extern crate tokio;

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
