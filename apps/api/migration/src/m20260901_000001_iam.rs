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
                    .table(Accounts::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Accounts::Id).uuid().not_null().primary_key())
                    .col(
                        ColumnDef::new(Accounts::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Accounts::PasswordHash).string().not_null())
                    .col(ColumnDef::new(Accounts::DisplayName).string().null())
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
                    .col(ColumnDef::new(RefreshTokens::DeviceId).string().not_null())
                    .col(
                        ColumnDef::new(RefreshTokens::TokenHash)
                            .string()
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
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_refresh_tokens_account_id_device_id")
                    .table(RefreshTokens::Table)
                    .col(RefreshTokens::AccountId)
                    .col(RefreshTokens::DeviceId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(OtpCodes::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(OtpCodes::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(OtpCodes::AccountId).uuid().not_null())
                    .col(ColumnDef::new(OtpCodes::CodeHash).string().not_null())
                    .col(ColumnDef::new(OtpCodes::Purpose).string().not_null())
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
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_otp_codes_account_id_purpose")
                    .table(OtpCodes::Table)
                    .col(OtpCodes::AccountId)
                    .col(OtpCodes::Purpose)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OtpCodes::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RefreshTokens::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Accounts::Table).to_owned())
            .await
    }
}
