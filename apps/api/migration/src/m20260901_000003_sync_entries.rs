use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Accounts {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Entries {
    Table,
    Id,
    AccountId,
    Date,
    Mood,
    Tags,
    StickerPlacements,
    Revision,
    DeviceUpdatedAt,
    DeletedAt,
    TitleCiphertext,
    TitleNonce,
    TitleUpdatedAt,
    TitleDeviceId,
    BodyCiphertext,
    BodyNonce,
    BodyUpdatedAt,
    BodyDeviceId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "-- One sequence, not a per-account counter. A cursor only needs revisions to be
                 -- monotonic within an account, and a global sequence gives that atomically
                 -- without a read-modify-write that two devices could race on.
                 CREATE SEQUENCE IF NOT EXISTS entries_revision_seq",
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Entries::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Entries::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Entries::AccountId).uuid().not_null())
                    .col(ColumnDef::new(Entries::Date).text().not_null())
                    .col(ColumnDef::new(Entries::Mood).text().not_null())
                    .col(ColumnDef::new(Entries::Tags).json_binary().not_null())
                    .col(
                        ColumnDef::new(Entries::StickerPlacements)
                            .json_binary()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Entries::Revision).big_integer().not_null())
                    .col(
                        ColumnDef::new(Entries::DeviceUpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Entries::DeletedAt).big_integer().null())
                    .col(ColumnDef::new(Entries::TitleCiphertext).binary().not_null())
                    .col(ColumnDef::new(Entries::TitleNonce).binary().not_null())
                    .col(
                        ColumnDef::new(Entries::TitleUpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Entries::TitleDeviceId).uuid().not_null())
                    .col(ColumnDef::new(Entries::BodyCiphertext).binary().not_null())
                    .col(ColumnDef::new(Entries::BodyNonce).binary().not_null())
                    .col(
                        ColumnDef::new(Entries::BodyUpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Entries::BodyDeviceId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Entries::Table, Entries::AccountId)
                            .to(Accounts::Table, Accounts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_entries_account_revision")
                    .table(Entries::Table)
                    .col(Entries::AccountId)
                    .col(Entries::Revision)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entries::Table).to_owned())
            .await?;
        manager
            .get_connection()
            .execute_unprepared("DROP SEQUENCE IF EXISTS entries_revision_seq")
            .await?;
        Ok(())
    }
}
