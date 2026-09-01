use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum CatalogBundles {
    Table,
    Id,
    Version,
    Payload,
    PublishedAt,
    PublishedBy,
    CreatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CatalogBundles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CatalogBundles::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CatalogBundles::Version)
                            .integer()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(CatalogBundles::Payload)
                            .json_binary()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CatalogBundles::PublishedAt)
                            .big_integer()
                            .null(),
                    )
                    .col(ColumnDef::new(CatalogBundles::PublishedBy).uuid().null())
                    .col(
                        ColumnDef::new(CatalogBundles::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "-- One published bundle at a time. A device asking which catalog to use must get
                 -- exactly one answer, and a partial index is what makes that true in the schema
                 -- rather than in whichever handler happens to publish.
                 CREATE UNIQUE INDEX IF NOT EXISTS catalog_bundles_published_key
                   ON catalog_bundles ((published_at IS NOT NULL)) WHERE published_at IS NOT NULL",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CatalogBundles::Table).to_owned())
            .await
    }
}
