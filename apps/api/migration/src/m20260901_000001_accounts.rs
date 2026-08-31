use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Accounts {
    Table,
    Id,
    Email,
    PasswordHash,
    DisplayName,
    EmailVerifiedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum RefreshTokens {
    Table,
    Id,
    AccountId,
    DeviceId,
    TokenHash,
    IssuedAt,
    ExpiresAt,
    RevokedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "-- citext gives accounts.email case-insensitive uniqueness without a functional
                 -- index every query has to remember to match. Postgres has trusted the extension
                 -- since 13, so the database owner may create it without being a superuser.
                 CREATE EXTENSION IF NOT EXISTS citext",
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Accounts::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Accounts::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(Accounts::Email)
                            .custom(Alias::new("citext"))
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Accounts::PasswordHash).text().not_null())
                    .col(ColumnDef::new(Accounts::DisplayName).text().null())
                    .col(
                        ColumnDef::new(Accounts::EmailVerifiedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Accounts::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Accounts::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RefreshTokens::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RefreshTokens::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RefreshTokens::AccountId).uuid().not_null())
                    .col(ColumnDef::new(RefreshTokens::DeviceId).text().not_null())
                    .col(
                        ColumnDef::new(RefreshTokens::TokenHash)
                            .text()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(RefreshTokens::IssuedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RefreshTokens::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(RefreshTokens::RevokedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(RefreshTokens::Table, RefreshTokens::AccountId)
                            .to(Accounts::Table, Accounts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "-- One live refresh credential per device. It has to be partial: rotation keeps the
                 -- replaced row, carrying the same (account_id, device_id) pair with revoked_at set,
                 -- so a total unique index would reject every rotation after the first.
                 CREATE UNIQUE INDEX IF NOT EXISTS refresh_tokens_live_device_key
                   ON refresh_tokens (account_id, device_id) WHERE revoked_at IS NULL",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RefreshTokens::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Accounts::Table).to_owned())
            .await
    }
}
