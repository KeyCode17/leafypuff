use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::error::{CoreError, ERR_ENTRY_ID_INVALID};
use super::mood::Mood;
use super::photo::{COVER_ORDINAL, PhotoRef};
use super::sticker::PlacedSticker;
use super::weather::{Location, Weather};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntryId(pub Uuid);

impl EntryId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn parse(raw: &str) -> Result<Self, CoreError> {
        Uuid::parse_str(raw)
            .map(Self)
            .map_err(|_| CoreError::Invalid(format!("{ERR_ENTRY_ID_INVALID}: {raw}")))
    }

    pub fn to_text(self) -> String {
        self.0.hyphenated().to_string()
    }

    pub const fn is_nil(self) -> bool {
        self.0.is_nil()
    }
}

impl Default for EntryId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub id: EntryId,
    pub date: NaiveDate,
    pub mood: Mood,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
    pub weather: Option<Weather>,
    pub location: Option<Location>,
    pub photos: Vec<PhotoRef>,
    pub stickers: Vec<PlacedSticker>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Entry {
    pub fn cover(&self) -> Option<&PhotoRef> {
        self.photos
            .iter()
            .find(|photo| photo.ordinal == COVER_ORDINAL)
    }
}
