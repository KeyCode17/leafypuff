use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum DataRequests {
    Table,
    Id,
    AccountId,
    Email,
    Kind,
    Status,
    RequestedAt,
    FulfilledAt,
    FulfilledBy,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DataRequests::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DataRequests::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DataRequests::AccountId).uuid().not_null())
                    .col(ColumnDef::new(DataRequests::Email).text().null())
                    .col(ColumnDef::new(DataRequests::Kind).text().not_null())
                    .col(ColumnDef::new(DataRequests::Status).text().not_null())
                    .col(
                        ColumnDef::new(DataRequests::RequestedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(DataRequests::FulfilledAt)
                            .big_integer()
                            .null(),
                    )
                    .col(ColumnDef::new(DataRequests::FulfilledBy).uuid().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_data_requests_status")
                    .table(DataRequests::Table)
                    .col(DataRequests::Status)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DataRequests::Table).to_owned())
            .await
    }
}
