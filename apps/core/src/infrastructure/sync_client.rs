use chrono::{DateTime, Utc};
use data_encoding::BASE64;
use reqwest::Client;
use sea_orm::ActiveValue;
use serde_json::{Value, json};
use uuid::Uuid;

use super::http_client;
use super::http_error::reached;
use crate::domain::{CoreError, EntryId, OutboundEntry, SyncOutcome};

use super::entity::entries;
use super::sync_outbox::{Carried, InboundPhoto, InboundSticker, SyncOutbox};

const PULL_PATH: &str = "/v1/sync/pull";
const PUSH_PATH: &str = "/v1/sync/push";
const ERR_UNREACHABLE: &str = "The sync service could not be reached";
const ERR_REFUSED: &str = "The sync service refused the exchange";
const ERR_SHAPE: &str = "The sync response is not the expected shape";

pub struct SyncClient {
    client: Client,
    base_url: String,
    access_token: String,
}

impl SyncClient {
    pub fn new(base_url: String, access_token: String, device_id: &str) -> Result<Self, CoreError> {
        Ok(Self {
            client: http_client::for_device(device_id, ERR_UNREACHABLE)?,
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
            .map_err(|error| reached(&error, ERR_UNREACHABLE))?;
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
    ) -> Result<(Vec<(entries::ActiveModel, Carried)>, i64), CoreError> {
        let response = self
            .client
            .get(format!("{}{PULL_PATH}?cursor={cursor}", self.base_url))
            .bearer_auth(&self.access_token)
            .header("x-device-id", device_id)
            .send()
            .await
            .map_err(|error| reached(&error, ERR_UNREACHABLE))?;
        if !response.status().is_success() {
            return Err(CoreError::Storage(format!(
                "{ERR_REFUSED}: {}",
                response.status()
            )));
        }
        let body: Value = response
            .json()
            .await
            .map_err(|_| CoreError::Unreadable(ERR_SHAPE.to_owned()))?;

        let advanced = body["data"]["cursor"]
            .as_i64()
            .ok_or_else(|| CoreError::Unreadable(ERR_SHAPE.to_owned()))?;
        let rows = body["data"]["records"]
            .as_array()
            .ok_or_else(|| CoreError::Unreadable(ERR_SHAPE.to_owned()))?;
        let accepted = rows
            .iter()
            .map(inbound)
            .collect::<Result<Vec<(entries::ActiveModel, Carried)>, CoreError>>()?;
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
        "weather": row.weather,
        "location": row.location,
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

fn inbound(row: &Value) -> Result<(entries::ActiveModel, Carried), CoreError> {
    let shape = || CoreError::Unreadable(ERR_SHAPE.to_owned());
    let updated_at_ms = row["title"]["updated_at_ms"].as_i64().ok_or_else(shape)?;
    let updated_at = DateTime::<Utc>::from_timestamp_millis(updated_at_ms)
        .ok_or_else(shape)?
        .to_rfc3339();

    let entry_id = row["id"].as_str().ok_or_else(shape)?.to_owned();
    let carried = Carried {
        photos: inbound_photos(&row["photo_refs"], &entry_id)?,
        stickers: inbound_stickers(&row["sticker_placements"])?,
        tags: inbound_tags(&row["tags"])?,
        entry_id: entry_id.clone(),
    };

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
            weather: ActiveValue::Set(row["weather"].as_str().map(str::to_owned)),
            location: ActiveValue::Set(row["location"].as_str().map(str::to_owned)),
            created_at: ActiveValue::Set(updated_at.clone()),
            updated_at: ActiveValue::Set(updated_at.clone()),
            synced_at: ActiveValue::Set(Some(updated_at)),
        },
        carried,
    ))
}

fn inbound_photos(value: &Value, entry_id: &str) -> Result<Vec<InboundPhoto>, CoreError> {
    let shape = || CoreError::Unreadable(ERR_SHAPE.to_owned());
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

fn inbound_tags(value: &Value) -> Result<Vec<String>, CoreError> {
    let shape = || CoreError::Unreadable(ERR_SHAPE.to_owned());
    let Some(tags) = value.as_array() else {
        return Ok(Vec::new());
    };
    tags.iter()
        .map(|tag| tag.as_str().map(str::to_owned).ok_or_else(shape))
        .collect()
}

fn inbound_stickers(value: &Value) -> Result<Vec<InboundSticker>, CoreError> {
    let shape = || CoreError::Unreadable(ERR_SHAPE.to_owned());
    let Some(encoded) = value.as_str() else {
        return Ok(Vec::new());
    };
    let parsed: Value = serde_json::from_str(encoded).map_err(|_| shape())?;
    parsed
        .as_array()
        .ok_or_else(shape)?
        .iter()
        .map(|sticker| {
            Ok(InboundSticker {
                key: sticker["key"].as_str().ok_or_else(shape)?.to_owned(),
                kind: sticker["kind"].as_str().ok_or_else(shape)?.to_owned(),
                x: number(&sticker["x"])?,
                y: number(&sticker["y"])?,
                size: number(&sticker["size"])?,
                rotation: number(&sticker["rotation"])?,
            })
        })
        .collect()
}

fn number(value: &Value) -> Result<f32, CoreError> {
    value
        .as_f64()
        .map(|held| held as f32)
        .ok_or_else(|| CoreError::Unreadable(ERR_SHAPE.to_owned()))
}

fn bytes(value: &Value) -> Result<Vec<u8>, CoreError> {
    let encoded = value
        .as_str()
        .ok_or_else(|| CoreError::Unreadable(ERR_SHAPE.to_owned()))?;
    BASE64
        .decode(encoded.as_bytes())
        .map_err(|_| CoreError::Unreadable(ERR_SHAPE.to_owned()))
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::{inbound_stickers, inbound_tags};
    use crate::infrastructure::entity::{photos, stickers};
    use crate::infrastructure::sync_outbox::{photo_refs, sticker_placements};

    fn sticker(key: &str, kind: &str) -> stickers::Model {
        stickers::Model {
            id: key.to_owned(),
            entry_id: "e1".to_owned(),
            kind: kind.to_owned(),
            x: 0.25,
            y: 0.5,
            size: 64.0,
            rotation: 90.0,
        }
    }

    #[test]
    fn a_placed_sticker_survives_the_wire_unchanged() {
        let written = sticker_placements(&[sticker("heart-0", "Heart")]).expect("it writes");

        let read = inbound_stickers(&serde_json::Value::String(written)).expect("it reads");

        assert_eq!(read.len(), 1);
        assert_eq!(read[0].key, "heart-0");
        assert_eq!(read[0].kind, "Heart");
        assert!((read[0].x - 0.25).abs() < f32::EPSILON);
        assert!((read[0].y - 0.5).abs() < f32::EPSILON);
        assert!((read[0].size - 64.0).abs() < f32::EPSILON);
        assert!((read[0].rotation - 90.0).abs() < f32::EPSILON);
    }

    #[test]
    fn an_entry_with_no_stickers_writes_and_reads_an_empty_list() {
        let written = sticker_placements(&[]).expect("it writes");

        assert_eq!(written, "[]");
        assert!(
            inbound_stickers(&serde_json::Value::String(written))
                .expect("it reads")
                .is_empty()
        );
    }

    #[test]
    fn a_photo_reference_survives_the_wire_unchanged() {
        let row = photos::Model {
            id: "3f2a91c0-0000-4000-8000-0000000000aa".to_owned(),
            entry_id: "e1".to_owned(),
            path: "/somewhere/on/the/first/device".to_owned(),
            ordinal: 0,
            taken_at: None,
        };

        let written = photo_refs(&[row]).expect("it writes");
        let read =
            super::inbound_photos(&serde_json::Value::String(written), "e1").expect("it reads");

        assert_eq!(read.len(), 1);
        assert_eq!(read[0].id, "3f2a91c0-0000-4000-8000-0000000000aa");
        assert_eq!(read[0].ordinal, 0);
        assert!(read[0].path.is_empty());
    }

    #[test]
    fn a_sticker_the_device_could_not_write_is_refused_rather_than_shipped() {
        let mut broken = sticker("heart-0", "Heart");
        broken.rotation = f32::NAN;

        assert!(sticker_placements(&[broken]).is_err());
    }

    #[test]
    fn a_record_with_no_tag_list_reads_as_no_tags() {
        assert!(
            inbound_tags(&serde_json::Value::Null)
                .expect("it reads")
                .is_empty()
        );
    }

    #[test]
    fn tags_come_back_as_written() {
        let read = inbound_tags(&serde_json::json!(["#rain", "#home"])).expect("it reads");

        assert_eq!(read, vec!["#rain".to_owned(), "#home".to_owned()]);
    }
}
