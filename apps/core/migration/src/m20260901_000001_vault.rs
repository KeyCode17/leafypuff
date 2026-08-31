use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Vault {
    Table,
    Id,
    PassphraseSalt,
    PassphraseNonce,
    PassphraseCiphertext,
    RecoveryNonce,
    RecoveryCiphertext,
    CreatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Vault::Table)
                    .if_not_exists()
                    // One row, always. The device holds exactly one content key, so the primary
                    // key is a constant rather than a generated id nothing would ever vary.
                    .col(
                        ColumnDef::new(Vault::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .default(1),
                    )
                    .col(ColumnDef::new(Vault::PassphraseSalt).binary().not_null())
                    .col(ColumnDef::new(Vault::PassphraseNonce).binary().not_null())
                    .col(
                        ColumnDef::new(Vault::PassphraseCiphertext)
                            .binary()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Vault::RecoveryNonce).binary().not_null())
                    .col(
                        ColumnDef::new(Vault::RecoveryCiphertext)
                            .binary()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Vault::CreatedAt).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Vault::Table).to_owned())
            .await
    }
}
