use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Accounts {
    Table,
    SealedProfile,
    AvatarPhotoId,
    ProfileUpdatedAtMs,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Accounts::Table)
                    .add_column(ColumnDef::new(Accounts::SealedProfile).text().null())
                    .add_column(ColumnDef::new(Accounts::AvatarPhotoId).uuid().null())
                    .add_column(
                        ColumnDef::new(Accounts::ProfileUpdatedAtMs)
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
            .alter_table(
                Table::alter()
                    .table(Accounts::Table)
                    .drop_column(Accounts::SealedProfile)
                    .drop_column(Accounts::AvatarPhotoId)
                    .drop_column(Accounts::ProfileUpdatedAtMs)
                    .to_owned(),
            )
            .await
    }
}
