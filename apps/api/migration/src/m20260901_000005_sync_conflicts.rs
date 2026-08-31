use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum SyncFieldConflicts {
    Table,
    Id,
    AccountId,
    EntryId,
    Field,
    WinnerUpdatedAt,
    LoserUpdatedAt,
    LoserDeviceId,
    LoserCiphertextHash,
    LoserByteLen,
    CreatedAt,
}

#[derive(DeriveIden)]
enum WrappedContentKeys {
    Table,
    AccountId,
    Kind,
    Blob,
    Salt,
    UpdatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SyncFieldConflicts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SyncFieldConflicts::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SyncFieldConflicts::AccountId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SyncFieldConflicts::EntryId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(SyncFieldConflicts::Field).text().not_null())
                    .col(
                        ColumnDef::new(SyncFieldConflicts::WinnerUpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SyncFieldConflicts::LoserUpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SyncFieldConflicts::LoserDeviceId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SyncFieldConflicts::LoserCiphertextHash)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SyncFieldConflicts::LoserByteLen)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SyncFieldConflicts::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(WrappedContentKeys::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WrappedContentKeys::AccountId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(WrappedContentKeys::Kind).text().not_null())
                    .col(ColumnDef::new(WrappedContentKeys::Blob).binary().not_null())
                    .col(ColumnDef::new(WrappedContentKeys::Salt).binary().not_null())
                    .col(
                        ColumnDef::new(WrappedContentKeys::UpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(WrappedContentKeys::AccountId)
                            .col(WrappedContentKeys::Kind),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WrappedContentKeys::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(SyncFieldConflicts::Table).to_owned())
            .await
    }
}
