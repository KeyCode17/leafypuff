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

// A third copy of the content key, wrapped under a key this device keeps in its hardware keystore.
// It exists so a returning owner is not asked for the account password on every launch. It is
// deliberately NOT part of the vault: the vault travels to the server so a new device can restore,
// and this row must never travel anywhere. A new device has no row, which is exactly why it asks
// for the password.
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DeviceSlot::Table)
                    .if_not_exists()
                    // One row, like the vault. A device holds one content key.
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
