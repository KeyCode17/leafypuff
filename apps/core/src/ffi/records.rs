use chrono::{DateTime, NaiveDate, Utc};

use crate::domain::error::{ERR_DATE_UNREADABLE, ERR_TIMESTAMP_UNREADABLE};
use crate::domain::{
    CoreError, Entry, EntryId, Location, Mood, PhotoRef, PlacedSticker, Sticker, Weather,
};

pub const ISO_DATE: &str = "%Y-%m-%d";

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct FfiPhoto {
    pub id: String,
    pub path: String,
    pub ordinal: i32,
    pub taken_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct FfiPlacedSticker {
    pub key: String,
    pub sticker: Sticker,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub rotation: f32,
}

#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct FfiEntry {
    pub id: String,
    pub date: String,
    pub mood: Mood,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
    pub weather: Option<Weather>,
    pub location: Option<Location>,
    pub photos: Vec<FfiPhoto>,
    pub stickers: Vec<FfiPlacedSticker>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<PhotoRef> for FfiPhoto {
    fn from(photo: PhotoRef) -> Self {
        Self {
            id: photo.id,
            path: photo.path,
            ordinal: photo.ordinal,
            taken_at: photo.taken_at.map(|at| at.to_rfc3339()),
        }
    }
}

fn read_timestamp(raw: &str) -> Result<DateTime<Utc>, CoreError> {
    DateTime::parse_from_rfc3339(raw)
        .map(|stamped| stamped.with_timezone(&Utc))
        .map_err(|_| CoreError::Invalid(format!("{ERR_TIMESTAMP_UNREADABLE}: {raw}")))
}

impl From<Entry> for FfiEntry {
    fn from(entry: Entry) -> Self {
        Self {
            id: entry.id.to_text(),
            date: entry.date.format(ISO_DATE).to_string(),
            mood: entry.mood,
            title: entry.title,
            body: entry.body,
            tags: entry.tags,
            weather: entry.weather,
            location: entry.location,
            photos: entry.photos.into_iter().map(FfiPhoto::from).collect(),
            stickers: entry
                .stickers
                .into_iter()
                .map(|sticker| FfiPlacedSticker {
                    key: sticker.key,
                    sticker: sticker.sticker,
                    x: sticker.x,
                    y: sticker.y,
                    size: sticker.size,
                    rotation: sticker.rotation,
                })
                .collect(),
            created_at: entry.created_at.to_rfc3339(),
            updated_at: entry.updated_at.to_rfc3339(),
        }
    }
}

impl TryFrom<FfiEntry> for Entry {
    type Error = CoreError;

    fn try_from(record: FfiEntry) -> Result<Self, Self::Error> {
        let mut photos = Vec::with_capacity(record.photos.len());
        for photo in record.photos {
            let taken_at = match photo.taken_at {
                None => None,
                Some(ref raw) => Some(read_timestamp(raw)?),
            };
            photos.push(PhotoRef {
                id: photo.id,
                path: photo.path,
                ordinal: photo.ordinal,
                taken_at,
            });
        }

        Ok(Self {
            id: EntryId::parse(&record.id)?,
            date: NaiveDate::parse_from_str(&record.date, ISO_DATE).map_err(|_| {
                CoreError::Invalid(format!("{ERR_DATE_UNREADABLE}: {}", record.date))
            })?,
            mood: record.mood,
            title: record.title,
            body: record.body,
            tags: record.tags,
            weather: record.weather,
            location: record.location,
            photos,
            stickers: record
                .stickers
                .into_iter()
                .map(|sticker| {
                    PlacedSticker::new(
                        sticker.key,
                        sticker.sticker,
                        sticker.x,
                        sticker.y,
                        sticker.size,
                        sticker.rotation,
                    )
                })
                .collect(),
            created_at: read_timestamp(&record.created_at)?,
            updated_at: read_timestamp(&record.updated_at)?,
        })
    }
}
