use async_trait::async_trait;
use sea_orm::sea_query::OnConflict;
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseBackend, DatabaseConnection, EntityTrait, QueryFilter,
    QueryOrder, QuerySelect, Statement,
};
use uuid::Uuid;

use crate::domain::sync::{ChangeSet, EntryRecord, EntryStore, SyncCursor, SyncError};

use super::entity::entries;
use super::mapper;

const NEXT_REVISION: &str = "SELECT nextval('entries_revision_seq') AS revision";

pub struct PgEntryStore {
    connection: DatabaseConnection,
}

impl PgEntryStore {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl EntryStore for PgEntryStore {
    async fn load(
        &self,
        account_id: Uuid,
        entry_id: Uuid,
    ) -> Result<Option<EntryRecord>, SyncError> {
        let found = entries::Entity::find_by_id(entry_id)
            .filter(entries::Column::AccountId.eq(account_id))
            .one(&self.connection)
            .await
            .map_err(mapper::storage)?;
        found.map(mapper::record).transpose()
    }

    async fn changed_since(
        &self,
        account_id: Uuid,
        cursor: SyncCursor,
        limit: u64,
    ) -> Result<ChangeSet, SyncError> {
        let rows = entries::Entity::find()
            .filter(entries::Column::AccountId.eq(account_id))
            .filter(entries::Column::Revision.gt(cursor.0))
            .order_by_asc(entries::Column::Revision)
            .limit(limit)
            .all(&self.connection)
            .await
            .map_err(mapper::storage)?;

        let mut advanced = cursor;
        let mut records = Vec::with_capacity(rows.len());
        for row in rows {
            advanced = advanced.advanced_to(row.revision);
            records.push(mapper::record(row)?);
        }
        Ok(ChangeSet {
            records,
            cursor: advanced,
        })
    }

    async fn next_revision(&self) -> Result<i64, SyncError> {
        let row = self
            .connection
            .query_one(Statement::from_string(
                DatabaseBackend::Postgres,
                NEXT_REVISION,
            ))
            .await
            .map_err(mapper::storage)?
            .ok_or_else(|| {
                SyncError::Storage("the revision sequence returned no row".to_owned())
            })?;
        row.try_get("", "revision").map_err(mapper::storage)
    }

    async fn upsert(&self, record: EntryRecord) -> Result<(), SyncError> {
        entries::Entity::insert(mapper::row(record))
            .on_conflict(
                OnConflict::column(entries::Column::Id)
                    .update_columns([
                        entries::Column::Date,
                        entries::Column::Mood,
                        entries::Column::Tags,
                        entries::Column::StickerPlacements,
                        entries::Column::Revision,
                        entries::Column::DeviceUpdatedAt,
                        entries::Column::DeletedAt,
                        entries::Column::TitleCiphertext,
                        entries::Column::TitleNonce,
                        entries::Column::TitleUpdatedAt,
                        entries::Column::TitleDeviceId,
                        entries::Column::BodyCiphertext,
                        entries::Column::BodyNonce,
                        entries::Column::BodyUpdatedAt,
                        entries::Column::BodyDeviceId,
                    ])
                    .to_owned(),
            )
            .exec(&self.connection)
            .await
            .map_err(mapper::storage)?;
        Ok(())
    }
}
