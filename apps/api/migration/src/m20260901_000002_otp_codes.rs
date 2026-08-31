use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Accounts {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum OtpCodes {
    Table,
    Id,
    AccountId,
    CodeHash,
    Purpose,
    Attempts,
    ExpiresAt,
    ConsumedAt,
    CreatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OtpCodes::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(OtpCodes::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(OtpCodes::AccountId).uuid().not_null())
                    .col(ColumnDef::new(OtpCodes::CodeHash).text().not_null())
                    .col(ColumnDef::new(OtpCodes::Purpose).text().not_null())
                    .col(
                        ColumnDef::new(OtpCodes::Attempts)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(OtpCodes::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OtpCodes::ConsumedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(OtpCodes::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(OtpCodes::Table, OtpCodes::AccountId)
                            .to(Accounts::Table, Accounts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "-- One open challenge per account and purpose. Partial for the same reason the
                 -- refresh index is: a consumed or expired row keeps its (account_id, purpose)
                 -- pair, and issuing the next code must not collide with it.
                 CREATE UNIQUE INDEX IF NOT EXISTS otp_codes_open_purpose_key
                   ON otp_codes (account_id, purpose) WHERE consumed_at IS NULL",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OtpCodes::Table).to_owned())
            .await
    }
}
