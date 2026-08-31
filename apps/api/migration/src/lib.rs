pub use sea_orm_migration::prelude::*;

mod m20260901_000001_accounts;
mod m20260901_000002_otp_codes;
mod m20260901_000003_sync_entries;
mod m20260901_000004_sync_state;
mod m20260901_000005_sync_conflicts;
mod m20260901_000006_media_objects;

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
        ]
    }
}
