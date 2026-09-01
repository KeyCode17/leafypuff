use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Entries {
    Table,
    PhotoRefs,
}

// A record carried its stickers but not its photos, so a device restoring an account received
// blobs with nothing to attach them to. Photo ids are not secret -- the blobs behind them are
// sealed under the account's content key, which the server does not hold -- so this is a plain
// string beside sticker_placements rather than another envelope. Existing rows default to an
// empty list, which reads as "this entry has no photos" and is true of every row written before
// the client learned to send them.
const EMPTY_LIST: &str = "[]";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Entries::Table)
                    .add_column(
                        ColumnDef::new(Entries::PhotoRefs)
                            .text()
                            .not_null()
                            .default(EMPTY_LIST),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Entries::Table)
                    .drop_column(Entries::PhotoRefs)
                    .to_owned(),
            )
            .await
    }
}
