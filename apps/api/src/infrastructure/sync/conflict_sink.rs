use async_trait::async_trait;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};
use uuid::Uuid;

use crate::domain::sync::{ConflictSink, FieldConflict, SyncError};

use super::clock::now_ms;
use super::entity::sync_field_conflicts;
use super::mapper::storage;

pub struct PgConflictSink {
    connection: DatabaseConnection,
}

impl PgConflictSink {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl ConflictSink for PgConflictSink {
    async fn record(&self, account_id: Uuid, conflict: FieldConflict) -> Result<(), SyncError> {
        sync_field_conflicts::Entity::insert(sync_field_conflicts::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            account_id: ActiveValue::Set(account_id),
            entry_id: ActiveValue::Set(conflict.entry_id),
            field: ActiveValue::Set(conflict.field.as_str().to_owned()),
            winner_updated_at: ActiveValue::Set(conflict.winner_updated_at_ms),
            loser_updated_at: ActiveValue::Set(conflict.loser_updated_at_ms),
            loser_device_id: ActiveValue::Set(conflict.loser_device_id),
            loser_ciphertext_hash: ActiveValue::Set(conflict.loser_ciphertext_hash),
            loser_byte_len: ActiveValue::Set(conflict.loser_byte_len),
            created_at: ActiveValue::Set(now_ms()),
        })
        .exec(&self.connection)
        .await
        .map_err(storage)?;
        Ok(())
    }
}
