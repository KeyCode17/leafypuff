pub use sea_orm_migration::prelude::*;

mod idens;
mod m20260831_175427_init_schema;
mod m20260901_000001_vault;
mod m20260901_000002_sync_state;

pub use idens::{Entries, Photos, Stickers, Tags};

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260831_175427_init_schema::Migration),
            Box::new(m20260901_000001_vault::Migration),
            Box::new(m20260901_000002_sync_state::Migration),
        ]
    }
}
