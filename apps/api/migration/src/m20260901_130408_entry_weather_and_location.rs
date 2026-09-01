use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Entries {
    Table,
    Weather,
    Location,
}

// A record carried neither, so an entry restored onto a new handset came back with its weather
// and location blank even though the writing device had both. Like the mood beside them these are
// short enum labels rather than anything private, so they travel as plain nullable text. Null is
// the honest default for a row written before the client sent them: the entry may have had one,
// and this server never learned it.
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Entries::Table)
                    .add_column(ColumnDef::new(Entries::Weather).text().null())
                    .add_column(ColumnDef::new(Entries::Location).text().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Entries::Table)
                    .drop_column(Entries::Weather)
                    .drop_column(Entries::Location)
                    .to_owned(),
            )
            .await
    }
}
