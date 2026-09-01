use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum ReleaseGates {
    Table,
    Platform,
    MinimumBuild,
    ForceUpdate,
    Message,
    UpdatedAt,
    UpdatedBy,
}

#[derive(DeriveIden)]
enum Campaigns {
    Table,
    Id,
    Title,
    Body,
    Platform,
    StartsAt,
    EndsAt,
    Published,
    CreatedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ReleaseGates::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ReleaseGates::Platform)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ReleaseGates::MinimumBuild)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(ReleaseGates::ForceUpdate)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(ColumnDef::new(ReleaseGates::Message).text().null())
                    .col(
                        ColumnDef::new(ReleaseGates::UpdatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ReleaseGates::UpdatedBy).uuid().null())
                    .to_owned(),
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "-- An open gate per platform, so a device asking before an operator has ever set
                 -- one gets a definite 'you are current' rather than a missing row it would have
                 -- to guess about.
                 INSERT INTO release_gates (platform, minimum_build, force_update, updated_at)
                 VALUES ('android', 0, false, 0), ('web', 0, false, 0)
                 ON CONFLICT (platform) DO NOTHING",
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Campaigns::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Campaigns::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Campaigns::Title).text().not_null())
                    .col(ColumnDef::new(Campaigns::Body).text().not_null())
                    .col(ColumnDef::new(Campaigns::Platform).text().not_null())
                    .col(ColumnDef::new(Campaigns::StartsAt).big_integer().not_null())
                    .col(ColumnDef::new(Campaigns::EndsAt).big_integer().not_null())
                    .col(
                        ColumnDef::new(Campaigns::Published)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Campaigns::CreatedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_campaigns_window")
                    .table(Campaigns::Table)
                    .col(Campaigns::Platform)
                    .col(Campaigns::StartsAt)
                    .col(Campaigns::EndsAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Campaigns::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(ReleaseGates::Table).to_owned())
            .await
    }
}
