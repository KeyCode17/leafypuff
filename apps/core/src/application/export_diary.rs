use std::io::{Cursor, Write};

use serde_json::json;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

use crate::domain::{CoreError, Entry, EntryRepository, PhotoKind, PhotoStore};

const ENTRIES_NAME: &str = "entries.json";
const PHOTO_DIR: &str = "photos";
const PHOTO_SUFFIX: &str = "jpg";
const ERR_ARCHIVE: &str = "The archive could not be written";

/// Writes the whole diary as one archive: every entry as JSON plus the original of every photo.
/// The entries come back decrypted, which is the point of an export — it is the one place the
/// plaintext is meant to leave the device, and only to a path the owner chose.
pub struct ExportDiary<R, S> {
    repository: R,
    photos: S,
}

impl<R: EntryRepository, S: PhotoStore> ExportDiary<R, S> {
    pub const fn new(repository: R, photos: S) -> Self {
        Self { repository, photos }
    }

    pub async fn execute(&self, limit: u32) -> Result<Vec<u8>, CoreError> {
        let entries = self.repository.list_desc(limit).await?;
        let mut archive = ZipWriter::new(Cursor::new(Vec::new()));
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);

        archive
            .start_file(ENTRIES_NAME, options)
            .map_err(archive_failed)?;
        let manifest = serde_json::to_vec_pretty(&entries.iter().map(describe).collect::<Vec<_>>())
            .map_err(|error| CoreError::Storage(format!("{ERR_ARCHIVE}: {error}")))?;
        archive.write_all(&manifest).map_err(archive_failed)?;

        for entry in &entries {
            for photo in &entry.photos {
                let bytes = self.photos.read(&photo.id, PhotoKind::Original)?;
                let name = format!("{PHOTO_DIR}/{}.{PHOTO_SUFFIX}", photo.id);
                archive.start_file(name, options).map_err(archive_failed)?;
                archive.write_all(&bytes).map_err(archive_failed)?;
            }
        }

        Ok(archive.finish().map_err(archive_failed)?.into_inner())
    }
}

fn archive_failed(error: impl std::fmt::Display) -> CoreError {
    CoreError::Storage(format!("{ERR_ARCHIVE}: {error}"))
}

fn describe(entry: &Entry) -> serde_json::Value {
    json!({
        "id": entry.id.to_text(),
        "date": entry.date.to_string(),
        "mood": entry.mood.as_str(),
        "title": entry.title,
        "body": entry.body,
        "tags": entry.tags,
        "weather": entry.weather.map(|weather| weather.as_str()),
        "location": entry.location.map(|location| location.as_str()),
        "photos": entry.photos.iter().map(|photo| json!({
            "id": photo.id,
            "ordinal": photo.ordinal,
        })).collect::<Vec<_>>(),
        "created_at": entry.created_at.to_rfc3339(),
        "updated_at": entry.updated_at.to_rfc3339(),
    })
}
