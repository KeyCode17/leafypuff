use sea_orm_migration::prelude::*;

use crate::idens::{Entries, Photos, Stickers, Tags};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Entries::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Entries::Id).text().not_null().primary_key())
                    .col(ColumnDef::new(Entries::Date).text().not_null())
                    .col(ColumnDef::new(Entries::Mood).text().not_null())
                    .col(ColumnDef::new(Entries::Title).binary().not_null())
                    .col(ColumnDef::new(Entries::TitleNonce).binary().null())
                    .col(ColumnDef::new(Entries::Body).binary().not_null())
                    .col(ColumnDef::new(Entries::BodyNonce).binary().null())
                    .col(
                        ColumnDef::new(Entries::Revision)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Entries::Weather).text().null())
                    .col(ColumnDef::new(Entries::Location).text().null())
                    .col(ColumnDef::new(Entries::CreatedAt).text().not_null())
                    .col(ColumnDef::new(Entries::UpdatedAt).text().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Photos::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Photos::Id).text().not_null().primary_key())
                    .col(ColumnDef::new(Photos::EntryId).text().not_null())
                    .col(ColumnDef::new(Photos::Path).text().not_null())
                    .col(ColumnDef::new(Photos::Ordinal).integer().not_null())
                    .col(ColumnDef::new(Photos::TakenAt).text().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Photos::Table, Photos::EntryId)
                            .to(Entries::Table, Entries::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Stickers::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Stickers::Id).text().not_null().primary_key())
                    .col(ColumnDef::new(Stickers::EntryId).text().not_null())
                    .col(ColumnDef::new(Stickers::Kind).text().not_null())
                    .col(ColumnDef::new(Stickers::X).float().not_null())
                    .col(ColumnDef::new(Stickers::Y).float().not_null())
                    .col(ColumnDef::new(Stickers::Size).float().not_null())
                    .col(ColumnDef::new(Stickers::Rotation).float().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Stickers::Table, Stickers::EntryId)
                            .to(Entries::Table, Entries::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Tags::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Tags::EntryId).text().not_null())
                    .col(ColumnDef::new(Tags::Tag).text().not_null())
                    .primary_key(Index::create().col(Tags::EntryId).col(Tags::Tag))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Tags::Table, Tags::EntryId)
                            .to(Entries::Table, Entries::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        create_indexes(manager).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tags::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Stickers::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Photos::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Entries::Table).to_owned())
            .await
    }
}

async fn create_indexes(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .create_index(
            Index::create()
                .name("idx_entries_date")
                .table(Entries::Table)
                .col(Entries::Date)
                .to_owned(),
        )
        .await?;
    manager
        .create_index(
            Index::create()
                .name("idx_entries_date_created_at")
                .table(Entries::Table)
                .col(Entries::Date)
                .col(Entries::CreatedAt)
                .to_owned(),
        )
        .await?;
    manager
        .create_index(
            Index::create()
                .name("idx_photos_entry_id_ordinal")
                .table(Photos::Table)
                .col(Photos::EntryId)
                .col(Photos::Ordinal)
                .unique()
                .to_owned(),
        )
        .await?;
    manager
        .create_index(
            Index::create()
                .name("idx_stickers_entry_id")
                .table(Stickers::Table)
                .col(Stickers::EntryId)
                .to_owned(),
        )
        .await?;
    manager
        .create_index(
            Index::create()
                .name("idx_tags_tag")
                .table(Tags::Table)
                .col(Tags::Tag)
                .to_owned(),
        )
        .await
}
