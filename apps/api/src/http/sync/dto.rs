use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvelopeRequest {
    pub ciphertext: String,
    pub nonce: String,
    pub updated_at_ms: i64,
    pub device_id: Uuid,
}

#[derive(Serialize)]
pub struct EnvelopeResponse {
    pub ciphertext: String,
    pub nonce: String,
    pub updated_at_ms: i64,
    pub device_id: Uuid,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordRequest {
    pub id: Uuid,
    pub date: String,
    pub mood: String,
    pub tags: Vec<String>,
    pub sticker_placements: String,
    pub photo_refs: String,
    pub weather: Option<String>,
    pub location: Option<String>,
    pub device_updated_at_ms: i64,
    pub deleted_at_ms: Option<i64>,
    pub title: EnvelopeRequest,
    pub body: EnvelopeRequest,
}

#[derive(Serialize)]
pub struct RecordResponse {
    pub id: Uuid,
    pub date: String,
    pub mood: String,
    pub tags: Vec<String>,
    pub sticker_placements: String,
    pub photo_refs: String,
    pub weather: Option<String>,
    pub location: Option<String>,
    pub revision: i64,
    pub device_updated_at_ms: i64,
    pub deleted_at_ms: Option<i64>,
    pub title: EnvelopeResponse,
    pub body: EnvelopeResponse,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PushRequest {
    pub records: Vec<RecordRequest>,
}

#[derive(Serialize)]
pub struct PullResponse {
    pub records: Vec<RecordResponse>,
    pub cursor: i64,
}

#[derive(Serialize)]
pub struct PushResponse {
    pub cursor: i64,
    pub applied: Vec<Uuid>,
    pub replayed: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WrappedKeyRequest {
    pub kind: String,
    pub blob: String,
    pub salt: String,
    pub updated_at_ms: i64,
}

#[derive(Serialize)]
pub struct WrappedKeyResponse {
    pub kind: String,
    pub blob: String,
    pub salt: String,
    pub updated_at_ms: i64,
}
