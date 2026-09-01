use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

const OWNER: &str = "a0000000-0000-4000-8000-000000000001";
const SUPPORT: &str = "a0000000-0000-4000-8000-000000000002";
const AUDITOR: &str = "a0000000-0000-4000-8000-000000000003";

const OWNER_GRANTS: [&str; 17] = [
    "account:list",
    "account:read",
    "account:suspend",
    "account:restore",
    "entry:metadata.read",
    "entry:count.read",
    "media:usage.read",
    "catalog:read",
    "catalog:write",
    "catalog:publish",
    "release:read",
    "release:write",
    "audit:read",
    "role:read",
    "role:write",
    "data_request:read",
    "data_request:fulfil",
];

// Support deliberately lacks role:write, catalog:publish and release:write. Someone who can read
// every account should not also be able to grant themselves more, or ship a release.
const SUPPORT_GRANTS: [&str; 8] = [
    "account:list",
    "account:read",
    "account:suspend",
    "account:restore",
    "entry:metadata.read",
    "entry:count.read",
    "media:usage.read",
    "data_request:read",
];

const AUDITOR_GRANTS: [&str; 2] = ["audit:read", "role:read"];

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for (role, grants) in [
            (OWNER, OWNER_GRANTS.as_slice()),
            (SUPPORT, SUPPORT_GRANTS.as_slice()),
            (AUDITOR, AUDITOR_GRANTS.as_slice()),
        ] {
            let values = grants
                .iter()
                .map(|permission| format!("('{role}', '{permission}')"))
                .collect::<Vec<String>>()
                .join(", ");
            manager
                .get_connection()
                .execute_unprepared(&format!(
                    "INSERT INTO role_permissions (role_id, permission) VALUES {values} \
                     ON CONFLICT (role_id, permission) DO NOTHING"
                ))
                .await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DELETE FROM role_permissions")
            .await?;
        Ok(())
    }
}
