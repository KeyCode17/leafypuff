use leafypuff_core::domain::Mood;
use sea_orm::{ActiveValue, DbErr};
use serde_json::Value;

use crate::domain::sync::error::{ERR_TAGS_UNREADABLE, ERR_UNKNOWN_MOOD};
use crate::domain::sync::{EntryRecord, FieldEnvelope, SyncError};

use super::entity::entries;

pub fn storage(error: DbErr) -> SyncError {
    SyncError::Storage(error.to_string())
}

pub fn record(row: entries::Model) -> Result<EntryRecord, SyncError> {
    let mood =
        Mood::parse(&row.mood).map_err(|_| SyncError::Storage(ERR_UNKNOWN_MOOD.to_owned()))?;
    Ok(EntryRecord {
        id: row.id,
        account_id: row.account_id,
        date: row.date,
        mood,
        tags: tags(&row.tags)?,
        sticker_placements: row.sticker_placements,
        photo_refs: row.photo_refs,
        weather: row.weather,
        location: row.location,
        revision: row.revision,
        device_updated_at_ms: row.device_updated_at,
        deleted_at_ms: row.deleted_at,
        title: FieldEnvelope {
            ciphertext: row.title_ciphertext,
            nonce: row.title_nonce,
            updated_at_ms: row.title_updated_at,
            device_id: row.title_device_id,
        },
        body: FieldEnvelope {
            ciphertext: row.body_ciphertext,
            nonce: row.body_nonce,
            updated_at_ms: row.body_updated_at,
            device_id: row.body_device_id,
        },
    })
}

pub fn row(record: EntryRecord) -> entries::ActiveModel {
    entries::ActiveModel {
        id: ActiveValue::Set(record.id),
        account_id: ActiveValue::Set(record.account_id),
        date: ActiveValue::Set(record.date),
        mood: ActiveValue::Set(record.mood.as_str().to_owned()),
        tags: ActiveValue::Set(Value::from(record.tags)),
        sticker_placements: ActiveValue::Set(record.sticker_placements),
        photo_refs: ActiveValue::Set(record.photo_refs),
        weather: ActiveValue::Set(record.weather),
        location: ActiveValue::Set(record.location),
        revision: ActiveValue::Set(record.revision),
        device_updated_at: ActiveValue::Set(record.device_updated_at_ms),
        deleted_at: ActiveValue::Set(record.deleted_at_ms),
        title_ciphertext: ActiveValue::Set(record.title.ciphertext),
        title_nonce: ActiveValue::Set(record.title.nonce),
        title_updated_at: ActiveValue::Set(record.title.updated_at_ms),
        title_device_id: ActiveValue::Set(record.title.device_id),
        body_ciphertext: ActiveValue::Set(record.body.ciphertext),
        body_nonce: ActiveValue::Set(record.body.nonce),
        body_updated_at: ActiveValue::Set(record.body.updated_at_ms),
        body_device_id: ActiveValue::Set(record.body.device_id),
    }
}

fn tags(stored: &Value) -> Result<Vec<String>, SyncError> {
    let unreadable = || SyncError::Storage(ERR_TAGS_UNREADABLE.to_owned());
    stored
        .as_array()
        .ok_or_else(unreadable)?
        .iter()
        .map(|tag| tag.as_str().map(str::to_owned).ok_or_else(unreadable))
        .collect()
}
