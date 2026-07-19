use sea_orm_migration::prelude::*;

#[derive(Debug, DeriveMigrationName)]
pub(crate) struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Users::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(Users::Username)
                            .string_len(64)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string_len(320)
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Users::PasswordHash)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Posts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Posts::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Posts::Title).string_len(255).not_null())
                    .col(ColumnDef::new(Posts::Content).text().not_null())
                    .col(ColumnDef::new(Posts::AuthorId).uuid().not_null())
                    .col(
                        ColumnDef::new(Posts::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Posts::UpdatedAt).timestamp_with_time_zone())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_posts_author_id")
                            .from(Posts::Table, Posts::AuthorId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_posts_created_at")
                    .table(Posts::Table)
                    .col(Posts::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_posts_author_id")
                    .table(Posts::Table)
                    .col(Posts::AuthorId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_posts_author_id")
                    .table(Posts::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_posts_created_at")
                    .table(Posts::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Posts::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Email,
    PasswordHash,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
    Title,
    Content,
    AuthorId,
    CreatedAt,
    UpdatedAt,
}
