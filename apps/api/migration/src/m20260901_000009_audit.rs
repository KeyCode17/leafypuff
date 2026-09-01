use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
enum AuditSubjects {
    Table,
    Id,
    AccountId,
    Email,
}

#[derive(DeriveIden)]
enum AuditEvents {
    Table,
    Id,
    ActorId,
    Action,
    SubjectId,
    Detail,
    RecordedAt,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AuditSubjects::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AuditSubjects::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AuditSubjects::AccountId).uuid().null())
                    .col(ColumnDef::new(AuditSubjects::Email).text().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AuditEvents::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AuditEvents::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AuditEvents::ActorId).uuid().not_null())
                    .col(ColumnDef::new(AuditEvents::Action).text().not_null())
                    .col(ColumnDef::new(AuditEvents::SubjectId).uuid().null())
                    .col(ColumnDef::new(AuditEvents::Detail).text().not_null())
                    .col(
                        ColumnDef::new(AuditEvents::RecordedAt)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(AuditEvents::Table, AuditEvents::SubjectId)
                            .to(AuditSubjects::Table, AuditSubjects::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx_audit_events_recorded_at")
                    .table(AuditEvents::Table)
                    .col(AuditEvents::RecordedAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AuditEvents::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AuditSubjects::Table).to_owned())
            .await
    }
}
