use async_trait::async_trait;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::domain::sync::error::ERR_UNKNOWN_KEY_KIND;
use crate::domain::sync::{KeyKind, SyncError, WrappedKeyRow, WrappedKeyStore};

use super::entity::wrapped_content_keys;
use super::mapper::storage;

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
