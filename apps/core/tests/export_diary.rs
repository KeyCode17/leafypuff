#![cfg(all(feature = "test-support", feature = "export"))]

mod support;

use std::io::{Cursor, Read};

use chrono::{NaiveDate, Utc};
use leafypuff_core::application::ExportDiary;
use leafypuff_core::domain::{
    Entry, EntryId, EntryRepository, Mood, PhotoKind, PhotoRef, PhotoStore,
};
use leafypuff_core::infrastructure::{FilePhotoStore, InMemoryEntryRepository, PlaintextSealer};

const PHOTO_ID: &str = "3f2a91c0-0000-4000-8000-0000000000aa";
const WHOLE_DIARY: u32 = 100;

fn entry(photos: Vec<PhotoRef>) -> Entry {
    let now = Utc::now();
    Entry {
        id: EntryId::new(),
        date: NaiveDate::from_ymd_opt(2026, 8, 31).expect("a real date"),
        mood: Mood::Calm,
        title: "Quiet morning".to_owned(),
        body: "Tea on the balcony.".to_owned(),
        tags: vec!["#slowday".to_owned()],
        weather: None,
        location: None,
        photos,
        stickers: Vec::new(),
        created_at: now,
        updated_at: now,
    }
}

fn names(archive: &[u8]) -> Vec<String> {
    let mut zip = zip::ZipArchive::new(Cursor::new(archive.to_vec())).expect("a readable archive");
    (0..zip.len())
        .map(|index| zip.by_index(index).expect("an entry").name().to_owned())
        .collect()
}

fn read_named(archive: &[u8], name: &str) -> Vec<u8> {
    let mut zip = zip::ZipArchive::new(Cursor::new(archive.to_vec())).expect("a readable archive");
    let mut file = zip.by_name(name).expect("the named member");
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).expect("the member reads");
    bytes
}

#[tokio::test]
async fn the_archive_carries_the_entries_as_json() {
    let repository = InMemoryEntryRepository::new();
    repository
        .save(entry(Vec::new()))
        .await
        .expect("the entry saves");
    let dir = tempfile::tempdir().expect("a temp dir");
    let photos = FilePhotoStore::new(dir.path().join("photos"), PlaintextSealer);

    let archive = ExportDiary::new(&repository, &photos)
        .execute(WHOLE_DIARY)
        .await
        .expect("the archive is written");

    assert_eq!(names(&archive), vec!["entries.json".to_owned()]);
    let manifest = String::from_utf8(read_named(&archive, "entries.json")).expect("utf-8");
    assert!(manifest.contains("Quiet morning"));
    assert!(manifest.contains("#slowday"));
}

#[tokio::test]
async fn every_photo_rides_along_decrypted() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let photos = FilePhotoStore::new(dir.path().join("photos"), PlaintextSealer);
    let bytes = support::solid_jpeg(48, 32);
    let path = photos
        .write(PHOTO_ID, PhotoKind::Original, &bytes)
        .expect("the original is written");

    let repository = InMemoryEntryRepository::new();
    repository
        .save(entry(vec![PhotoRef {
            id: PHOTO_ID.to_owned(),
            path,
            ordinal: 0,
            taken_at: None,
        }]))
        .await
        .expect("the entry saves");

    let archive = ExportDiary::new(&repository, &photos)
        .execute(WHOLE_DIARY)
        .await
        .expect("the archive is written");

    assert!(names(&archive).contains(&format!("photos/{PHOTO_ID}.jpg")));
    assert_eq!(
        read_named(&archive, &format!("photos/{PHOTO_ID}.jpg")),
        bytes
    );
}

#[tokio::test]
async fn an_empty_diary_still_produces_a_readable_archive() {
    let dir = tempfile::tempdir().expect("a temp dir");
    let photos = FilePhotoStore::new(dir.path().join("photos"), PlaintextSealer);

    let archive = ExportDiary::new(&InMemoryEntryRepository::new(), &photos)
        .execute(WHOLE_DIARY)
        .await
        .expect("the archive is written");

    assert_eq!(names(&archive), vec!["entries.json".to_owned()]);
    assert_eq!(read_named(&archive, "entries.json"), b"[]");
}
