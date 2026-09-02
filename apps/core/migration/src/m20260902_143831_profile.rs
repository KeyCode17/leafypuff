use sea_orm_migration::prelude::*;

use crate::idens::Profile;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Profile::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Profile::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .default(1),
                    )
                    .col(ColumnDef::new(Profile::DisplayName).text().null())
                    .col(ColumnDef::new(Profile::AvatarPhotoId).text().null())
                    .col(ColumnDef::new(Profile::AvatarX).double().null())
                    .col(ColumnDef::new(Profile::AvatarY).double().null())
                    .col(ColumnDef::new(Profile::AvatarWidth).double().null())
                    .col(
                        ColumnDef::new(Profile::UpdatedAtMs)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Profile::Table).to_owned())
            .await
    }
}
