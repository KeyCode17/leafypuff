use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum SyncCheckpoints {
    Table,
    AccountId,
    DeviceId,
    Cursor,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum SyncRequests {
    Table,
    IdempotencyKey,
    AccountId,
    DeviceId,
    ResponseHash,
    CreatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SyncCheckpoints::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(SyncCheckpoints::AccountId).uuid().not_null())
                    .col(ColumnDef::new(SyncCheckpoints::DeviceId).uuid().not_null())
                    .col(
                        ColumnDef::new(SyncCheckpoints::Cursor)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(SyncCheckpoints::UpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(SyncCheckpoints::AccountId)
                            .col(SyncCheckpoints::DeviceId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SyncRequests::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SyncRequests::IdempotencyKey)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SyncRequests::AccountId).uuid().not_null())
                    .col(ColumnDef::new(SyncRequests::DeviceId).uuid().not_null())
                    .col(ColumnDef::new(SyncRequests::ResponseHash).text().not_null())
                    .col(
                        ColumnDef::new(SyncRequests::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SyncRequests::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(SyncCheckpoints::Table).to_owned())
            .await
    }
}
