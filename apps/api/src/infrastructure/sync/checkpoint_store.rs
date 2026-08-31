use async_trait::async_trait;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};
use uuid::Uuid;

use crate::domain::sync::{CheckpointStore, SyncCursor, SyncError};

use super::clock::now_ms;
use super::entity::sync_checkpoints;
use super::mapper::storage;

pub struct PgCheckpointStore {
    connection: DatabaseConnection,
}

impl PgCheckpointStore {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl CheckpointStore for PgCheckpointStore {
    async fn read(&self, account_id: Uuid, device_id: Uuid) -> Result<SyncCursor, SyncError> {
        let row = sync_checkpoints::Entity::find_by_id((account_id, device_id))
            .one(&self.connection)
            .await
            .map_err(storage)?;
        Ok(row.map_or(SyncCursor::START, |found| SyncCursor(found.cursor)))
    }

    async fn advance(
        &self,
        account_id: Uuid,
        device_id: Uuid,
        cursor: SyncCursor,
    ) -> Result<(), SyncError> {
        sync_checkpoints::Entity::insert(sync_checkpoints::ActiveModel {
            account_id: ActiveValue::Set(account_id),
            device_id: ActiveValue::Set(device_id),
            cursor: ActiveValue::Set(cursor.0),
            updated_at: ActiveValue::Set(now_ms()),
        })
        .on_conflict(
            OnConflict::columns([
                sync_checkpoints::Column::AccountId,
                sync_checkpoints::Column::DeviceId,
            ])
            .update_columns([
                sync_checkpoints::Column::Cursor,
                sync_checkpoints::Column::UpdatedAt,
            ])
            .to_owned(),
        )
        .exec(&self.connection)
        .await
        .map_err(storage)?;
        Ok(())
    }
}
