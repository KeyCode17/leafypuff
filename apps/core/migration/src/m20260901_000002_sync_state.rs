use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum SyncState {
    Table,
    Id,
    DeviceId,
    Cursor,
    LastSyncedAt,
}

#[derive(DeriveIden)]
enum Entries {
    Table,
    SyncedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SyncState::Table)
                    .if_not_exists()
                    // One row, like the vault. A device has one identity and one cursor.
                    .col(
                        ColumnDef::new(SyncState::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .default(1),
                    )
                    .col(ColumnDef::new(SyncState::DeviceId).string().not_null())
                    .col(
                        ColumnDef::new(SyncState::Cursor)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(SyncState::LastSyncedAt).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Entries::Table)
                    // Null means the row has never reached the server. Comparing it against
                    // updated_at is what makes a local edit push again without a dirty flag that
                    // a crash could leave stale.
                    .add_column_if_not_exists(ColumnDef::new(Entries::SyncedAt).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Entries::Table)
                    .drop_column(Entries::SyncedAt)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(SyncState::Table).to_owned())
            .await
    }
}
