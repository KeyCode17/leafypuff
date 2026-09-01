use chrono::Utc;
use sea_orm::sea_query::{Expr, OnConflict};
use sea_orm::{
    ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect,
};
use uuid::Uuid;

use crate::domain::error::ERR_TIMESTAMP_UNREADABLE;
use crate::domain::{CoreError, EntryId, OutboundEntry};

use super::entity::{entries, sync_state, tags};

const ONLY_ROW: i32 = 1;
const PUSH_BATCH: u64 = 100;

/// The rows a device still owes the server, and the cursor it has reached. Both live beside the
/// entries themselves so an interrupted exchange resumes rather than restarting.
pub struct SyncOutbox {
    connection: DatabaseConnection,
}

impl SyncOutbox {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }

    pub async fn device_id(&self) -> Result<String, CoreError> {
        if let Some(row) = sync_state::Entity::find_by_id(ONLY_ROW)
            .one(&self.connection)
            .await?
        {
            return Ok(row.device_id);
        }
        let minted = Uuid::new_v4().hyphenated().to_string();
        sync_state::Entity::insert(sync_state::ActiveModel {
            id: ActiveValue::Set(ONLY_ROW),
            device_id: ActiveValue::Set(minted.clone()),
            cursor: ActiveValue::Set(0),
            last_synced_at: ActiveValue::Set(None),
        })
        .exec(&self.connection)
        .await?;
        Ok(minted)
    }

    pub async fn cursor(&self) -> Result<i64, CoreError> {
        let row = sync_state::Entity::find_by_id(ONLY_ROW)
            .one(&self.connection)
            .await?;
        Ok(row.map_or(0, |held| held.cursor))
    }

    pub async fn advance(&self, cursor: i64) -> Result<(), CoreError> {
        sync_state::Entity::update_many()
            .col_expr(sync_state::Column::Cursor, cursor.into())
            .col_expr(
                sync_state::Column::LastSyncedAt,
                Utc::now().to_rfc3339().into(),
            )
            .filter(sync_state::Column::Id.eq(ONLY_ROW))
            .exec(&self.connection)
            .await?;
        Ok(())
    }

    /// Never synced, or edited since it last was. Comparing the two timestamps rather than
    /// holding a dirty flag means a crash between the write and the flag cannot lose an edit.
    pub async fn pending(&self) -> Result<Vec<OutboundEntry>, CoreError> {
        let rows =
            entries::Entity::find()
                .filter(entries::Column::SyncedAt.is_null().or(
                    Expr::col(entries::Column::UpdatedAt).gt(Expr::col(entries::Column::SyncedAt)),
                ))
                .limit(PUSH_BATCH)
                .all(&self.connection)
                .await?;

        let mut outbound = Vec::with_capacity(rows.len());
        for row in rows {
            let id = EntryId::parse(&row.id)?;
            let labels = tags::Entity::find()
                .filter(tags::Column::EntryId.eq(row.id.clone()))
                .all(&self.connection)
                .await?;
            outbound.push(OutboundEntry {
                id,
                date: row.date,
                mood: row.mood,
                tags: labels.into_iter().map(|tag| tag.tag).collect(),
                sticker_placements: "[]".to_owned(),
                device_updated_at_ms: millis(&row.updated_at)?,
                deleted_at_ms: None,
                title_ciphertext: row.title,
                title_nonce: row.title_nonce.unwrap_or_default(),
                body_ciphertext: row.body,
                body_nonce: row.body_nonce.unwrap_or_default(),
                updated_at: stamp(&row.updated_at)?,
            });
        }
        Ok(outbound)
    }

    pub async fn mark_synced(&self, ids: &[EntryId]) -> Result<(), CoreError> {
        if ids.is_empty() {
            return Ok(());
        }
        let now = Utc::now().to_rfc3339();
        entries::Entity::update_many()
            .col_expr(entries::Column::SyncedAt, now.into())
            .filter(
                entries::Column::Id
                    .is_in(ids.iter().map(|id| id.to_text()).collect::<Vec<String>>()),
            )
            .exec(&self.connection)
            .await?;
        Ok(())
    }

    pub async fn accept(&self, inbound: entries::ActiveModel) -> Result<(), CoreError> {
        entries::Entity::insert(inbound)
            .on_conflict(
                OnConflict::column(entries::Column::Id)
                    .update_columns([
                        entries::Column::Date,
                        entries::Column::Mood,
                        entries::Column::Title,
                        entries::Column::TitleNonce,
                        entries::Column::Body,
                        entries::Column::BodyNonce,
                        entries::Column::UpdatedAt,
                        entries::Column::SyncedAt,
                    ])
                    .to_owned(),
            )
            .exec(&self.connection)
            .await?;
        Ok(())
    }
}

fn stamp(raw: &str) -> Result<chrono::DateTime<Utc>, CoreError> {
    chrono::DateTime::parse_from_rfc3339(raw)
        .map(|held| held.with_timezone(&Utc))
        .map_err(|_| CoreError::Storage(format!("{ERR_TIMESTAMP_UNREADABLE}: {raw}")))
}

fn millis(raw: &str) -> Result<i64, CoreError> {
    Ok(stamp(raw)?.timestamp_millis())
}
