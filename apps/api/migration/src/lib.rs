pub use sea_orm_migration::prelude::*;

mod m20260901_000001_accounts;
mod m20260901_000002_otp_codes;
mod m20260901_000003_sync_entries;
mod m20260901_000004_sync_state;
mod m20260901_000005_sync_conflicts;
mod m20260901_000006_media_objects;
mod m20260901_000007_rbac;
mod m20260901_000008_role_grants;
mod m20260901_000009_audit;
mod m20260901_000010_account_suspension;
mod m20260901_000011_catalog;
mod m20260901_000013_data_requests;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260901_000001_accounts::Migration),
            Box::new(m20260901_000002_otp_codes::Migration),
            Box::new(m20260901_000003_sync_entries::Migration),
            Box::new(m20260901_000004_sync_state::Migration),
            Box::new(m20260901_000005_sync_conflicts::Migration),
            Box::new(m20260901_000006_media_objects::Migration),
            Box::new(m20260901_000007_rbac::Migration),
            Box::new(m20260901_000008_role_grants::Migration),
            Box::new(m20260901_000009_audit::Migration),
            Box::new(m20260901_000010_account_suspension::Migration),
            Box::new(m20260901_000011_catalog::Migration),
            Box::new(m20260901_000013_data_requests::Migration),
        ]
    }
}
