use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Entries {
    Table,
    PhotoRefs,
}

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
