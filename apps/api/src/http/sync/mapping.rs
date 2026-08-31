use data_encoding::BASE64;
use leafypuff_core::domain::Mood;
use uuid::Uuid;

use crate::domain::sync::{EntryRecord, FieldEnvelope, KeyKind, SyncError, WrappedKeyRow};

use super::dto::{
    EnvelopeRequest, EnvelopeResponse, RecordRequest, RecordResponse, WrappedKeyRequest,
    WrappedKeyResponse,
};

const ERR_BASE64: &str = "a field is not base64";
const ERR_MOOD: &str = "mood is not a known variant";
const ERR_KEY_KIND: &str = "kind must be passphrase or recovery";

pub fn record(account_id: Uuid, request: RecordRequest) -> Result<EntryRecord, SyncError> {
    let mood = Mood::parse(&request.mood).map_err(|_| SyncError::Malformed(ERR_MOOD.to_owned()))?;
    Ok(EntryRecord {
        id: request.id,
        account_id,
        date: request.date,
        mood,
        tags: request.tags,
        sticker_placements: request.sticker_placements,
        revision: 0,
        device_updated_at_ms: request.device_updated_at_ms,
        deleted_at_ms: request.deleted_at_ms,
        title: envelope(request.title)?,
        body: envelope(request.body)?,
    })
}

pub fn record_response(record: EntryRecord) -> RecordResponse {
    RecordResponse {
        id: record.id,
        date: record.date,
        mood: record.mood.as_str().to_owned(),
        tags: record.tags,
        sticker_placements: record.sticker_placements,
        revision: record.revision,
        device_updated_at_ms: record.device_updated_at_ms,
        deleted_at_ms: record.deleted_at_ms,
        title: envelope_response(record.title),
        body: envelope_response(record.body),
    }
}

pub fn wrapped_key(request: WrappedKeyRequest) -> Result<WrappedKeyRow, SyncError> {
    let kind = KeyKind::parse(&request.kind)
        .ok_or_else(|| SyncError::Malformed(ERR_KEY_KIND.to_owned()))?;
    Ok(WrappedKeyRow {
        kind,
        blob: bytes(&request.blob)?,
        salt: bytes(&request.salt)?,
        updated_at_ms: request.updated_at_ms,
    })
}

pub fn wrapped_key_response(row: WrappedKeyRow) -> WrappedKeyResponse {
    WrappedKeyResponse {
        kind: row.kind.as_str().to_owned(),
        blob: BASE64.encode(&row.blob),
        salt: BASE64.encode(&row.salt),
        updated_at_ms: row.updated_at_ms,
    }
}

fn envelope(request: EnvelopeRequest) -> Result<FieldEnvelope, SyncError> {
    Ok(FieldEnvelope {
        ciphertext: bytes(&request.ciphertext)?,
        nonce: bytes(&request.nonce)?,
        updated_at_ms: request.updated_at_ms,
        device_id: request.device_id,
    })
}

fn envelope_response(envelope: FieldEnvelope) -> EnvelopeResponse {
    EnvelopeResponse {
        ciphertext: BASE64.encode(&envelope.ciphertext),
        nonce: BASE64.encode(&envelope.nonce),
        updated_at_ms: envelope.updated_at_ms,
        device_id: envelope.device_id,
    }
}

fn bytes(encoded: &str) -> Result<Vec<u8>, SyncError> {
    BASE64
        .decode(encoded.as_bytes())
        .map_err(|_| SyncError::Malformed(ERR_BASE64.to_owned()))
}
