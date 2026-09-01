use chrono::{DateTime, Utc};

use super::entry::EntryId;

/// One entry's sealed fields exactly as they sit on disk. The device never re-encrypts to push:
/// the local ciphertext and its nonce are what the server stores, so an upload is a copy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutboundEntry {
    pub id: EntryId,
    pub date: String,
    pub mood: String,
    pub tags: Vec<String>,
    pub sticker_placements: String,
    pub device_updated_at_ms: i64,
    pub deleted_at_ms: Option<i64>,
    pub title_ciphertext: Vec<u8>,
    pub title_nonce: Vec<u8>,
    pub body_ciphertext: Vec<u8>,
    pub body_nonce: Vec<u8>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyncOutcome {
    pub pushed: u32,
    pub pulled: u32,
    pub cursor: i64,
}
