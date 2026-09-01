use chrono::{DateTime, Utc};
use data_encoding::BASE64;
use reqwest::Client;
use sea_orm::ActiveValue;
use serde_json::{Value, json};
use uuid::Uuid;

use crate::domain::{CoreError, EntryId, OutboundEntry, SyncOutcome};

use super::entity::entries;
use super::sync_outbox::{InboundPhoto, SyncOutbox};

const PULL_PATH: &str = "/v1/sync/pull";
const PUSH_PATH: &str = "/v1/sync/push";
const ERR_UNREACHABLE: &str = "The sync service could not be reached";
const ERR_REFUSED: &str = "The sync service refused the exchange";
const ERR_SHAPE: &str = "The sync response is not the expected shape";

/// The device half of the exchange. It uploads the ciphertext it already holds and stores what
/// comes back without opening it, so a sync never needs the vault to be unlocked.
pub struct SyncClient {
    client: Client,
    base_url: String,
    access_token: String,
}

impl SyncClient {
    pub fn new(base_url: String, access_token: String) -> Result<Self, CoreError> {
        let client = Client::builder()
            .build()
            .map_err(|_| CoreError::Storage(ERR_UNREACHABLE.to_owned()))?;
        Ok(Self {
            client,
            base_url,
            access_token,
        })
    }

    pub async fn exchange(&self, outbox: &SyncOutbox) -> Result<SyncOutcome, CoreError> {
        let device_id = outbox.device_id().await?;
        let pending = outbox.pending().await?;
        let pushed = u32::try_from(pending.len()).unwrap_or(u32::MAX);

        if !pending.is_empty() {
            let ids: Vec<EntryId> = pending.iter().map(|row| row.id).collect();
            self.push(&device_id, &pending).await?;
            outbox.mark_synced(&ids).await?;
        }

        let cursor = outbox.cursor().await?;
        let (records, advanced) = self.pull(&device_id, cursor).await?;
        let pulled = u32::try_from(records.len()).unwrap_or(u32::MAX);
        for (record, carried) in records {
            outbox.accept(record, &carried).await?;
        }
        outbox.advance(advanced).await?;

        Ok(SyncOutcome {
            pushed,
            pulled,
            cursor: advanced,
        })
    }

    async fn push(&self, device_id: &str, pending: &[OutboundEntry]) -> Result<(), CoreError> {
        let records: Vec<Value> = pending
            .iter()
            .map(|row| record(row, device_id))
            .collect::<Result<Vec<Value>, CoreError>>()?;
        let response = self
            .client
            .post(format!("{}{PUSH_PATH}", self.base_url))
            .bearer_auth(&self.access_token)
            .header("x-device-id", device_id)
            .header("idempotency-key", Uuid::new_v4().hyphenated().to_string())
            .json(&json!({ "records": records }))
            .send()
            .await
            .map_err(|error| CoreError::Storage(format!("{ERR_UNREACHABLE}: {error}")))?;
        if !response.status().is_success() {
            return Err(CoreError::Storage(format!(
                "{ERR_REFUSED}: {}",
                response.status()
            )));
        }
        Ok(())
    }

    async fn pull(
        &self,
        device_id: &str,
        cursor: i64,
    ) -> Result<(Vec<(entries::ActiveModel, Vec<InboundPhoto>)>, i64), CoreError> {
        let response = self
            .client
            .get(format!("{}{PULL_PATH}?cursor={cursor}", self.base_url))
            .bearer_auth(&self.access_token)
            .header("x-device-id", device_id)
            .send()
            .await
            .map_err(|error| CoreError::Storage(format!("{ERR_UNREACHABLE}: {error}")))?;
        if !response.status().is_success() {
            return Err(CoreError::Storage(format!(
                "{ERR_REFUSED}: {}",
                response.status()
            )));
        }
        let body: Value = response
            .json()
            .await
            .map_err(|_| CoreError::Storage(ERR_SHAPE.to_owned()))?;

        let advanced = body["data"]["cursor"]
            .as_i64()
            .ok_or_else(|| CoreError::Storage(ERR_SHAPE.to_owned()))?;
        let rows = body["data"]["records"]
            .as_array()
            .ok_or_else(|| CoreError::Storage(ERR_SHAPE.to_owned()))?;
        let accepted = rows
            .iter()
            .map(inbound)
            .collect::<Result<Vec<(entries::ActiveModel, Vec<InboundPhoto>)>, CoreError>>()?;
        Ok((accepted, advanced))
    }
}

fn record(row: &OutboundEntry, device_id: &str) -> Result<Value, CoreError> {
    Ok(json!({
        "id": row.id.0,
        "date": row.date,
        "mood": row.mood,
        "tags": row.tags,
        "sticker_placements": row.sticker_placements,
        "photo_refs": row.photo_refs,
        "device_updated_at_ms": row.device_updated_at_ms,
        "deleted_at_ms": row.deleted_at_ms,
        "title": envelope(&row.title_ciphertext, &row.title_nonce, row.device_updated_at_ms, device_id),
        "body": envelope(&row.body_ciphertext, &row.body_nonce, row.device_updated_at_ms, device_id),
    }))
}

fn envelope(ciphertext: &[u8], nonce: &[u8], at_ms: i64, device_id: &str) -> Value {
    json!({
        "ciphertext": BASE64.encode(ciphertext),
        "nonce": BASE64.encode(nonce),
        "updated_at_ms": at_ms,
        "device_id": device_id,
    })
}

fn inbound(row: &Value) -> Result<(entries::ActiveModel, Vec<InboundPhoto>), CoreError> {
    let shape = || CoreError::Storage(ERR_SHAPE.to_owned());
    let updated_at_ms = row["title"]["updated_at_ms"].as_i64().ok_or_else(shape)?;
    let updated_at = DateTime::<Utc>::from_timestamp_millis(updated_at_ms)
        .ok_or_else(shape)?
        .to_rfc3339();

    let entry_id = row["id"].as_str().ok_or_else(shape)?.to_owned();
    let carried = inbound_photos(&row["photo_refs"], &entry_id)?;

    Ok((
        entries::ActiveModel {
            id: ActiveValue::Set(entry_id),
            date: ActiveValue::Set(row["date"].as_str().ok_or_else(shape)?.to_owned()),
            mood: ActiveValue::Set(row["mood"].as_str().ok_or_else(shape)?.to_owned()),
            title: ActiveValue::Set(bytes(&row["title"]["ciphertext"])?),
            title_nonce: ActiveValue::Set(Some(bytes(&row["title"]["nonce"])?)),
            body: ActiveValue::Set(bytes(&row["body"]["ciphertext"])?),
            body_nonce: ActiveValue::Set(Some(bytes(&row["body"]["nonce"])?)),
            revision: ActiveValue::Set(0),
            weather: ActiveValue::Set(None),
            location: ActiveValue::Set(None),
            created_at: ActiveValue::Set(updated_at.clone()),
            updated_at: ActiveValue::Set(updated_at.clone()),
            synced_at: ActiveValue::Set(Some(updated_at)),
        },
        carried,
    ))
}

/// The server carries photo references as the JSON string the writing device sent. `path` is left
/// empty: this device fills it in when it fetches the blob, and an empty one is how the next sync
/// knows it has not.
fn inbound_photos(value: &Value, entry_id: &str) -> Result<Vec<InboundPhoto>, CoreError> {
    let shape = || CoreError::Storage(ERR_SHAPE.to_owned());
    let Some(encoded) = value.as_str() else {
        return Ok(Vec::new());
    };
    let parsed: Value = serde_json::from_str(encoded).map_err(|_| shape())?;
    parsed
        .as_array()
        .ok_or_else(shape)?
        .iter()
        .map(|photo| {
            Ok(InboundPhoto {
                id: photo["id"].as_str().ok_or_else(shape)?.to_owned(),
                entry_id: entry_id.to_owned(),
                path: String::new(),
                ordinal: i32::try_from(photo["ordinal"].as_i64().ok_or_else(shape)?)
                    .map_err(|_| shape())?,
            })
        })
        .collect()
}

fn bytes(value: &Value) -> Result<Vec<u8>, CoreError> {
    let encoded = value
        .as_str()
        .ok_or_else(|| CoreError::Storage(ERR_SHAPE.to_owned()))?;
    BASE64
        .decode(encoded.as_bytes())
        .map_err(|_| CoreError::Storage(ERR_SHAPE.to_owned()))
}
