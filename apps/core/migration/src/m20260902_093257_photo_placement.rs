use sea_orm_migration::prelude::*;

use crate::idens::Photos;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Placement {
    PlaceX,
    PlaceY,
    PlaceSize,
    PlaceRotation,
}

const COLUMNS: [Placement; 4] = [
    Placement::PlaceX,
    Placement::PlaceY,
    Placement::PlaceSize,
    Placement::PlaceRotation,
];

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for column in COLUMNS {
            manager
                .alter_table(
                    Table::alter()
                        .table(Photos::Table)
                        .add_column(ColumnDef::new(column).double().null())
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for column in COLUMNS {
            manager
                .alter_table(
                    Table::alter()
                        .table(Photos::Table)
                        .drop_column(column)
                        .to_owned(),
                )
                .await?;
        }
        Ok(())
    }
}
