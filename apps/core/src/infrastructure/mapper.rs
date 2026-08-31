use chrono::{DateTime, NaiveDate, Utc};
use sea_orm::Set;

use crate::domain::error::{ERR_DATE_UNREADABLE, ERR_TEXT_UNREADABLE, ERR_TIMESTAMP_UNREADABLE};
use crate::domain::{
    CoreError, Entry, EntryId, Location, Mood, PhotoRef, PlacedSticker, Sticker, Weather,
};

use super::entity::{entries, photos, stickers, tags};

const ISO_DATE: &str = "%Y-%m-%d";

fn read_date(raw: &str) -> Result<NaiveDate, CoreError> {
    NaiveDate::parse_from_str(raw, ISO_DATE)
        .map_err(|_| CoreError::Storage(format!("{ERR_DATE_UNREADABLE}: {raw}")))
}

fn read_timestamp(raw: &str) -> Result<DateTime<Utc>, CoreError> {
    DateTime::parse_from_rfc3339(raw)
        .map(|stamped| stamped.with_timezone(&Utc))
        .map_err(|_| CoreError::Storage(format!("{ERR_TIMESTAMP_UNREADABLE}: {raw}")))
}

fn read_text(raw: Vec<u8>) -> Result<String, CoreError> {
    String::from_utf8(raw).map_err(|_| CoreError::Storage(ERR_TEXT_UNREADABLE.to_owned()))
}

fn write_text(value: &str) -> Vec<u8> {
    value.as_bytes().to_vec()
}

impl TryFrom<photos::Model> for PhotoRef {
    type Error = CoreError;

    fn try_from(row: photos::Model) -> Result<Self, Self::Error> {
        let taken_at = match row.taken_at {
            None => None,
            Some(ref raw) => Some(read_timestamp(raw)?),
        };
        Ok(Self {
            id: row.id,
            path: row.path,
            ordinal: row.ordinal,
            taken_at,
        })
    }
}

impl TryFrom<stickers::Model> for PlacedSticker {
    type Error = CoreError;

    fn try_from(row: stickers::Model) -> Result<Self, Self::Error> {
        Ok(Self::new(
            row.id,
            Sticker::parse(&row.kind)?,
            row.x,
            row.y,
            row.size,
            row.rotation,
        ))
    }
}

pub fn assemble(
    entry: entries::Model,
    photo_rows: Vec<photos::Model>,
    sticker_rows: Vec<stickers::Model>,
    tag_rows: Vec<tags::Model>,
) -> Result<Entry, CoreError> {
    let weather = match entry.weather {
        None => None,
        Some(ref raw) => Some(Weather::parse(raw)?),
    };
    let location = match entry.location {
        None => None,
        Some(ref raw) => Some(Location::parse(raw)?),
    };
    let mut photos = photo_rows
        .into_iter()
        .map(PhotoRef::try_from)
        .collect::<Result<Vec<PhotoRef>, CoreError>>()?;
    photos.sort_by_key(|photo| photo.ordinal);

    Ok(Entry {
        id: EntryId::parse(&entry.id)?,
        date: read_date(&entry.date)?,
        mood: Mood::parse(&entry.mood)?,
        title: read_text(entry.title)?,
        body: read_text(entry.body)?,
        tags: tag_rows.into_iter().map(|row| row.tag).collect(),
        weather,
        location,
        photos,
        stickers: sticker_rows
            .into_iter()
            .map(PlacedSticker::try_from)
            .collect::<Result<Vec<PlacedSticker>, CoreError>>()?,
        created_at: read_timestamp(&entry.created_at)?,
        updated_at: read_timestamp(&entry.updated_at)?,
    })
}

pub fn entry_row(entry: &Entry) -> entries::ActiveModel {
    entries::ActiveModel {
        id: Set(entry.id.to_text()),
        date: Set(entry.date.format(ISO_DATE).to_string()),
        mood: Set(entry.mood.as_str().to_owned()),
        title: Set(write_text(&entry.title)),
        title_nonce: Set(None),
        body: Set(write_text(&entry.body)),
        body_nonce: Set(None),
        revision: Set(0),
        weather: Set(entry.weather.map(|value| value.as_str().to_owned())),
        location: Set(entry.location.map(|value| value.as_str().to_owned())),
        created_at: Set(entry.created_at.to_rfc3339()),
        updated_at: Set(entry.updated_at.to_rfc3339()),
    }
}

pub fn photo_row(entry_id: &str, photo: &PhotoRef) -> photos::ActiveModel {
    photos::ActiveModel {
        id: Set(photo.id.clone()),
        entry_id: Set(entry_id.to_owned()),
        path: Set(photo.path.clone()),
        ordinal: Set(photo.ordinal),
        taken_at: Set(photo.taken_at.map(|at| at.to_rfc3339())),
    }
}

pub fn sticker_row(entry_id: &str, sticker: &PlacedSticker) -> stickers::ActiveModel {
    stickers::ActiveModel {
        id: Set(sticker.key.clone()),
        entry_id: Set(entry_id.to_owned()),
        kind: Set(sticker.sticker.as_str().to_owned()),
        x: Set(sticker.x),
        y: Set(sticker.y),
        size: Set(sticker.size),
        rotation: Set(sticker.rotation),
    }
}

pub fn tag_row(entry_id: &str, tag: &str) -> tags::ActiveModel {
    tags::ActiveModel {
        entry_id: Set(entry_id.to_owned()),
        tag: Set(tag.to_owned()),
    }
}
