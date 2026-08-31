#![cfg(feature = "ffi")]

use leafypuff_core::ffi::{
    FfiEntry, FfiPhoto, FfiPlacedSticker, LeafyPuffCore, LeafyPuffCoreError,
};

fn draft(id: &str) -> FfiEntry {
    FfiEntry {
        id: id.to_owned(),
        date: "2026-08-31".to_owned(),
        mood: leafypuff_core::domain::Mood::Loved,
        title: "Quiet morning".to_owned(),
        body: "Tea on the balcony.".to_owned(),
        tags: vec!["#slowday".to_owned(), "#tea".to_owned()],
        weather: Some(leafypuff_core::domain::Weather::Sunny),
        location: Some(leafypuff_core::domain::Location::Home),
        photos: vec![FfiPhoto {
            id: "photo-1".to_owned(),
            path: "/photos/one.jpg".to_owned(),
            ordinal: 0,
            taken_at: None,
        }],
        stickers: vec![FfiPlacedSticker {
            key: "sticker-1".to_owned(),
            sticker: leafypuff_core::domain::Sticker::BunSleep,
            x: 12.5,
            y: 48.25,
            size: 64.0,
            rotation: 90.0,
        }],
        created_at: "2026-08-31T08:00:00+00:00".to_owned(),
        updated_at: "2026-08-31T08:00:00+00:00".to_owned(),
    }
}

async fn core() -> (tempfile::TempDir, std::sync::Arc<LeafyPuffCore>) {
    let dir = tempfile::tempdir().expect("a temp dir");
    let path = dir
        .path()
        .join("diary.sqlite")
        .to_string_lossy()
        .into_owned();
    let core = LeafyPuffCore::new(path).await.expect("the core opens");
    (dir, core)
}

#[tokio::test]
async fn an_entry_survives_the_record_round_trip() {
    let (_dir, core) = core().await;
    let id = "11111111-1111-4111-8111-111111111111";

    let saved = core.save_entry(draft(id)).await.expect("saves");
    let read = core
        .entry_by_id(id.to_owned())
        .await
        .expect("reads")
        .expect("the entry exists");

    assert_eq!(read.id, saved.id);
    assert_eq!(read.date, "2026-08-31");
    assert_eq!(read.mood, leafypuff_core::domain::Mood::Loved);
    assert_eq!(read.title, "Quiet morning");
    assert_eq!(read.body, "Tea on the balcony.");
    assert_eq!(read.tags, vec!["#slowday".to_owned(), "#tea".to_owned()]);
    assert_eq!(read.weather, Some(leafypuff_core::domain::Weather::Sunny));
    assert_eq!(read.location, Some(leafypuff_core::domain::Location::Home));
    assert_eq!(read.photos, saved.photos);
    assert_eq!(read.stickers, saved.stickers);
}

#[tokio::test]
async fn missing_entry_maps_to_not_found() {
    let (_dir, core) = core().await;
    let absent = core
        .entry_by_id("22222222-2222-4222-8222-222222222222".to_owned())
        .await
        .expect("a missing entry is not an error for by_id");
    assert!(absent.is_none());

    let malformed = core.entry_by_id("not-a-uuid".to_owned()).await;
    assert!(matches!(malformed, Err(LeafyPuffCoreError::Invalid { .. })));

    let error = LeafyPuffCoreError::from(leafypuff_core::domain::CoreError::NotFound {
        id: "22222222-2222-4222-8222-222222222222".to_owned(),
    });
    assert!(matches!(error, LeafyPuffCoreError::NotFound { .. }));
}
