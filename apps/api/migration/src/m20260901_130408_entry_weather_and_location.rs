use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Entries {
    Table,
    Weather,
    Location,
}

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
