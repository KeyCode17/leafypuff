use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum DeviceSlot {
    Table,
    Id,
    Nonce,
    Ciphertext,
    CreatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DeviceSlot::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DeviceSlot::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .default(1),
                    )
                    .col(ColumnDef::new(DeviceSlot::Nonce).blob().not_null())
                    .col(ColumnDef::new(DeviceSlot::Ciphertext).blob().not_null())
                    .col(ColumnDef::new(DeviceSlot::CreatedAt).text().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DeviceSlot::Table).to_owned())
            .await
    }
}
