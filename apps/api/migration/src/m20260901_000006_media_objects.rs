use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Accounts {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum MediaObjects {
    Table,
    PhotoId,
    Variant,
    AccountId,
    EntryId,
    ByteLen,
    CiphertextHash,
    CreatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MediaObjects::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(MediaObjects::PhotoId).uuid().not_null())
                    .col(ColumnDef::new(MediaObjects::Variant).text().not_null())
                    .col(ColumnDef::new(MediaObjects::AccountId).uuid().not_null())
                    .col(ColumnDef::new(MediaObjects::EntryId).uuid().not_null())
                    .col(
                        ColumnDef::new(MediaObjects::ByteLen)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MediaObjects::CiphertextHash)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MediaObjects::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(MediaObjects::PhotoId)
                            .col(MediaObjects::Variant),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(MediaObjects::Table, MediaObjects::AccountId)
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
                    .name("idx_media_objects_account_entry")
                    .table(MediaObjects::Table)
                    .col(MediaObjects::AccountId)
                    .col(MediaObjects::EntryId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MediaObjects::Table).to_owned())
            .await
    }
}
