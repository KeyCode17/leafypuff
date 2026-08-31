use async_trait::async_trait;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::domain::sync::error::ERR_UNKNOWN_KEY_KIND;
use crate::domain::sync::{
    CheckpointStore, ConflictSink, FieldConflict, IdempotencyStore, KeyKind, SyncCursor, SyncError,
    WrappedKeyRow, WrappedKeyStore,
};

use super::entity::{sync_checkpoints, sync_field_conflicts, sync_requests, wrapped_content_keys};
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

pub struct PgIdempotencyStore {
    connection: DatabaseConnection,
}

impl PgIdempotencyStore {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl IdempotencyStore for PgIdempotencyStore {
    async fn recall(&self, key: &str) -> Result<Option<String>, SyncError> {
        let row = sync_requests::Entity::find_by_id(key.to_owned())
            .one(&self.connection)
            .await
            .map_err(storage)?;
        Ok(row.map(|found| found.response_hash))
    }

    async fn remember(
        &self,
        key: &str,
        account_id: Uuid,
        device_id: Uuid,
        response_hash: &str,
    ) -> Result<(), SyncError> {
        sync_requests::Entity::insert(sync_requests::ActiveModel {
            idempotency_key: ActiveValue::Set(key.to_owned()),
            account_id: ActiveValue::Set(account_id),
            device_id: ActiveValue::Set(device_id),
            response_hash: ActiveValue::Set(response_hash.to_owned()),
            created_at: ActiveValue::Set(now_ms()),
        })
        .on_conflict(
            OnConflict::column(sync_requests::Column::IdempotencyKey)
                .do_nothing()
                .to_owned(),
        )
        .do_nothing()
        .exec(&self.connection)
        .await
        .map_err(storage)?;
        Ok(())
    }
}

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

pub struct PgWrappedKeyStore {
    connection: DatabaseConnection,
}

impl PgWrappedKeyStore {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl WrappedKeyStore for PgWrappedKeyStore {
    async fn read_all(&self, account_id: Uuid) -> Result<Vec<WrappedKeyRow>, SyncError> {
        let rows = wrapped_content_keys::Entity::find()
            .filter(wrapped_content_keys::Column::AccountId.eq(account_id))
            .all(&self.connection)
            .await
            .map_err(storage)?;

        rows.into_iter()
            .map(|row| {
                let kind = KeyKind::parse(&row.kind)
                    .ok_or_else(|| SyncError::Storage(ERR_UNKNOWN_KEY_KIND.to_owned()))?;
                Ok(WrappedKeyRow {
                    kind,
                    blob: row.blob,
                    salt: row.salt,
                    updated_at_ms: row.updated_at,
                })
            })
            .collect()
    }

    async fn put(&self, account_id: Uuid, row: WrappedKeyRow) -> Result<(), SyncError> {
        wrapped_content_keys::Entity::insert(wrapped_content_keys::ActiveModel {
            account_id: ActiveValue::Set(account_id),
            kind: ActiveValue::Set(row.kind.as_str().to_owned()),
            blob: ActiveValue::Set(row.blob),
            salt: ActiveValue::Set(row.salt),
            updated_at: ActiveValue::Set(row.updated_at_ms),
        })
        .on_conflict(
            OnConflict::columns([
                wrapped_content_keys::Column::AccountId,
                wrapped_content_keys::Column::Kind,
            ])
            .update_columns([
                wrapped_content_keys::Column::Blob,
                wrapped_content_keys::Column::Salt,
                wrapped_content_keys::Column::UpdatedAt,
            ])
            .to_owned(),
        )
        .exec(&self.connection)
        .await
        .map_err(storage)?;
        Ok(())
    }
}

fn now_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}
