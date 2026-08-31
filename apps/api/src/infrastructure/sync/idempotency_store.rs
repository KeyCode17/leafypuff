use async_trait::async_trait;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};
use uuid::Uuid;

use crate::domain::sync::{IdempotencyStore, SyncError};

use super::clock::now_ms;
use super::entity::sync_requests;
use super::mapper::storage;

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
