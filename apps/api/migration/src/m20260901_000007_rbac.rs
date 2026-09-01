use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum Accounts {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Roles {
    Table,
    Id,
    Name,
    Description,
}

#[derive(DeriveIden)]
enum RolePermissions {
    Table,
    RoleId,
    Permission,
}

#[derive(DeriveIden)]
enum AccountRoles {
    Table,
    AccountId,
    RoleId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Roles::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Roles::Name).text().not_null().unique_key())
                    .col(ColumnDef::new(Roles::Description).text().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RolePermissions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(RolePermissions::RoleId).uuid().not_null())
                    .col(
                        ColumnDef::new(RolePermissions::Permission)
                            .text()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(RolePermissions::RoleId)
                            .col(RolePermissions::Permission),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(RolePermissions::Table, RolePermissions::RoleId)
                            .to(Roles::Table, Roles::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AccountRoles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(AccountRoles::AccountId).uuid().not_null())
                    .col(ColumnDef::new(AccountRoles::RoleId).uuid().not_null())
                    .primary_key(
                        Index::create()
                            .col(AccountRoles::AccountId)
                            .col(AccountRoles::RoleId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AccountRoles::Table, AccountRoles::AccountId)
                            .to(Accounts::Table, Accounts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AccountRoles::Table, AccountRoles::RoleId)
                            .to(Roles::Table, Roles::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "-- Stable uuids so the same role means the same thing in every environment, and
                 -- INSERT ... ON CONFLICT DO NOTHING so re-applying seeds nothing twice.
                 INSERT INTO roles (id, name, description) VALUES
                   ('a0000000-0000-4000-8000-000000000001', 'owner',
                    'Every permission, including the ones that change who has permissions'),
                   ('a0000000-0000-4000-8000-000000000002', 'support',
                    'Reads accounts and metadata, suspends and restores, never publishes'),
                   ('a0000000-0000-4000-8000-000000000003', 'auditor',
                    'Reads the audit log and nothing that changes state')
                 ON CONFLICT (id) DO NOTHING",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AccountRoles::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RolePermissions::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Roles::Table).to_owned())
            .await
    }
}
