use chrono::{DateTime, NaiveDate, Utc};
use sea_orm::Set;

use crate::domain::crypto::{FIELD_BODY, FIELD_TITLE};
use crate::domain::error::{ERR_DATE_UNREADABLE, ERR_TEXT_UNREADABLE, ERR_TIMESTAMP_UNREADABLE};
use crate::domain::{
    CoreError, Entry, EntryId, FieldSealer, Location, Mood, PhotoRef, PlacedSticker, Sticker,
    Weather,
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

fn read_field(
    sealer: &(dyn FieldSealer + Send + Sync),
    entry_id: EntryId,
    field_name: &str,
    updated_at_ms: i64,
    ciphertext: Vec<u8>,
    nonce: Option<Vec<u8>>,
) -> Result<String, CoreError> {
    let nonce = nonce.ok_or_else(|| CoreError::Storage(ERR_TEXT_UNREADABLE.to_owned()))?;
    sealer.open_field(entry_id, field_name, updated_at_ms, &ciphertext, &nonce)
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
    sealer: &(dyn FieldSealer + Send + Sync),
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

    let id = EntryId::parse(&entry.id)?;
    let updated_at = read_timestamp(&entry.updated_at)?;
    let updated_at_ms = updated_at.timestamp_millis();

    Ok(Entry {
        id,
        date: read_date(&entry.date)?,
        mood: Mood::parse(&entry.mood)?,
        title: read_field(
            sealer,
            id,
            FIELD_TITLE,
            updated_at_ms,
            entry.title,
            entry.title_nonce,
        )?,
        body: read_field(
            sealer,
            id,
            FIELD_BODY,
            updated_at_ms,
            entry.body,
            entry.body_nonce,
        )?,
        tags: tag_rows.into_iter().map(|row| row.tag).collect(),
        weather,
        location,
        photos,
        stickers: sticker_rows
            .into_iter()
            .map(PlacedSticker::try_from)
            .collect::<Result<Vec<PlacedSticker>, CoreError>>()?,
        created_at: read_timestamp(&entry.created_at)?,
        updated_at,
    })
}

pub fn entry_row(
    entry: &Entry,
    sealer: &(dyn FieldSealer + Send + Sync),
) -> Result<entries::ActiveModel, CoreError> {
    let updated_at_ms = entry.updated_at.timestamp_millis();
    let title = sealer.seal_field(entry.id, FIELD_TITLE, updated_at_ms, &entry.title)?;
    let body = sealer.seal_field(entry.id, FIELD_BODY, updated_at_ms, &entry.body)?;

    Ok(entries::ActiveModel {
        id: Set(entry.id.to_text()),
        date: Set(entry.date.format(ISO_DATE).to_string()),
        mood: Set(entry.mood.as_str().to_owned()),
        title: Set(title.ciphertext),
        title_nonce: Set(Some(title.nonce.to_vec())),
        body: Set(body.ciphertext),
        body_nonce: Set(Some(body.nonce.to_vec())),
        revision: Set(0),
        weather: Set(entry.weather.map(|value| value.as_str().to_owned())),
        location: Set(entry.location.map(|value| value.as_str().to_owned())),
        created_at: Set(entry.created_at.to_rfc3339()),
        updated_at: Set(entry.updated_at.to_rfc3339()),
        synced_at: Set(None),
    })
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
