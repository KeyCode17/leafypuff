pub use sea_orm_migration::prelude::*;

mod idens;
mod m20260831_175427_init_schema;
mod m20260901_000001_vault;
mod m20260901_000002_sync_state;
mod m20260901_111329_normalise_sticker_coordinates;
mod m20260901_111606_device_slot;
mod m20260902_060439_photo_framing;
mod m20260902_093257_photo_placement;

pub use idens::{Entries, Photos, Stickers, Tags};

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260831_175427_init_schema::Migration),
            Box::new(m20260901_000001_vault::Migration),
            Box::new(m20260901_000002_sync_state::Migration),
            Box::new(m20260901_111329_normalise_sticker_coordinates::Migration),
            Box::new(m20260901_111606_device_slot::Migration),
            Box::new(m20260902_060439_photo_framing::Migration),
            Box::new(m20260902_093257_photo_placement::Migration),
        ]
    }
}
