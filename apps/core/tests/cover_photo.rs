#![cfg(feature = "sqlite")]

use chrono::{NaiveDate, Utc};
use leafypuff_core::domain::crypto::ContentKey;
use leafypuff_core::domain::{Entry, EntryId, EntryRepository, Mood, PhotoRef};
use leafypuff_core::infrastructure::SqliteEntryRepository;
use leafypuff_core::infrastructure::VaultSealer;
use leafypuff_core::infrastructure::db;

async fn repository() -> (tempfile::TempDir, SqliteEntryRepository<VaultSealer>) {
    let dir = tempfile::tempdir().expect("a temp dir");
    let path = dir
        .path()
        .join("diary.sqlite")
        .to_string_lossy()
        .into_owned();
    let connection = db::open(&path).await.expect("a temp file opens");
    db::run_migrations(&connection)
        .await
        .expect("migrations apply");
    (
        dir,
        SqliteEntryRepository::new(connection, unlocked_sealer()),
    )
}

fn photo(id: &str) -> PhotoRef {
    PhotoRef {
        id: id.to_owned(),
        path: format!("/photos/{id}.jpg"),
        ordinal: 0,
        taken_at: None,
    }
}

fn entry(photos: Vec<PhotoRef>) -> Entry {
    let now = Utc::now();
    Entry {
        id: EntryId::new(),
        date: NaiveDate::from_ymd_opt(2026, 8, 31).expect("a real date"),
        mood: Mood::Calm,
        title: "Three photos".to_owned(),
        body: "One of them is the cover.".to_owned(),
        tags: Vec::new(),
        weather: None,
        location: None,
        photos,
        stickers: Vec::new(),
        created_at: now,
        updated_at: now,
    }
}

#[tokio::test]
async fn save_renumbers_photos_to_a_dense_sequence() {
    let (_dir, repository) = repository().await;
    let mut draft = entry(vec![photo("a"), photo("b"), photo("c")]);
    draft.photos[0].ordinal = 7;
    draft.photos[1].ordinal = 7;
    draft.photos[2].ordinal = 9;

    let saved = repository.save(draft).await.expect("saves");
    let ordinals: Vec<i32> = saved.photos.iter().map(|found| found.ordinal).collect();
    assert_eq!(ordinals, vec![0, 1, 2]);
}

#[tokio::test]
async fn removing_cover_promotes_next_photo() {
    let (_dir, repository) = repository().await;
    let draft = entry(vec![photo("a"), photo("b"), photo("c")]);
    let id = draft.id;

    let saved = repository.save(draft).await.expect("saves");
    assert_eq!(saved.cover().map(|found| found.id.as_str()), Some("a"));

    let survivors: Vec<PhotoRef> = saved
        .photos
        .into_iter()
        .filter(|found| found.ordinal != 0)
        .collect();
    let resaved = repository
        .save(Entry {
            photos: survivors,
            ..saved
        })
        .await
        .expect("saves again");

    let ordinals: Vec<i32> = resaved.photos.iter().map(|found| found.ordinal).collect();
    assert_eq!(ordinals, vec![0, 1]);
    assert_eq!(resaved.cover().map(|found| found.id.as_str()), Some("b"));

    let reread = repository
        .by_id(id)
        .await
        .expect("reads back")
        .expect("the entry exists");
    assert_eq!(reread.cover().map(|found| found.id.as_str()), Some("b"));
    assert_eq!(reread.photos.len(), 2);
}

fn unlocked_sealer() -> VaultSealer {
    let sealer = VaultSealer::new();
    sealer
        .unlock(ContentKey::generate().expect("a content key"))
        .expect("the sealer unlocks");
    sealer
}
