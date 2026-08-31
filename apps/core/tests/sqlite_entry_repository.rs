#![cfg(feature = "sqlite")]

use chrono::{NaiveDate, TimeZone, Utc};
use leafypuff_core::domain::{
    Entry, EntryId, EntryRepository, Location, Mood, PhotoRef, PlacedSticker, Sticker, Weather,
};
use leafypuff_core::infrastructure::SqliteEntryRepository;
use leafypuff_core::infrastructure::db;
use leafypuff_core::infrastructure::entity::{entries, photos, stickers, tags};
use sea_orm::{DatabaseConnection, EntityTrait};

async fn repository() -> (tempfile::TempDir, DatabaseConnection, SqliteEntryRepository) {
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
    let repository = SqliteEntryRepository::new(connection.clone());
    (dir, connection, repository)
}

fn entry(day: u32, title: &str) -> Entry {
    let at = Utc.with_ymd_and_hms(2026, 8, day, 8, 0, 0).unwrap();
    Entry {
        id: EntryId::new(),
        date: NaiveDate::from_ymd_opt(2026, 8, day).expect("a real date"),
        mood: Mood::Loved,
        title: title.to_owned(),
        body: "Tea on the balcony.".to_owned(),
        tags: vec!["#slowday".to_owned(), "#tea".to_owned()],
        weather: Some(Weather::Sunny),
        location: Some(Location::Home),
        photos: vec![PhotoRef {
            id: format!("photo-{day}"),
            path: format!("/photos/{day}.jpg"),
            ordinal: 0,
            taken_at: Some(at),
        }],
        stickers: vec![PlacedSticker::new(
            format!("sticker-{day}"),
            Sticker::BunSleep,
            12.5,
            48.25,
            64.0,
            90.0,
        )],
        created_at: at,
        updated_at: at,
    }
}

#[tokio::test]
async fn save_then_list_desc_returns_newest_first() {
    let (_dir, _connection, repository) = repository().await;
    repository.save(entry(29, "Oldest")).await.expect("saves");
    repository.save(entry(31, "Newest")).await.expect("saves");
    repository.save(entry(30, "Middle")).await.expect("saves");

    let listed = repository.list_desc(10).await.expect("lists");
    let titles: Vec<&str> = listed.iter().map(|found| found.title.as_str()).collect();
    assert_eq!(titles, vec!["Newest", "Middle", "Oldest"]);

    let newest = listed.first().expect("at least one entry");
    assert_eq!(newest.mood, Mood::Loved);
    assert_eq!(newest.weather, Some(Weather::Sunny));
    assert_eq!(newest.tags.len(), 2);
    assert_eq!(newest.stickers.len(), 1);
    assert_eq!(newest.cover().map(|photo| photo.ordinal), Some(0));
}

#[tokio::test]
async fn in_range_and_on_date_read_the_same_shape() {
    let (_dir, _connection, repository) = repository().await;
    let target = entry(30, "Middle");
    let id = target.id;
    repository.save(entry(29, "Oldest")).await.expect("saves");
    repository.save(target).await.expect("saves");
    repository.save(entry(31, "Newest")).await.expect("saves");

    let ranged = repository
        .in_range(
            NaiveDate::from_ymd_opt(2026, 8, 30).expect("a real date"),
            NaiveDate::from_ymd_opt(2026, 8, 31).expect("a real date"),
        )
        .await
        .expect("reads a range");
    assert_eq!(ranged.len(), 2);

    let on_day = repository
        .on_date(NaiveDate::from_ymd_opt(2026, 8, 30).expect("a real date"))
        .await
        .expect("reads a day");
    assert_eq!(on_day.len(), 1);

    let by_id = repository
        .by_id(id)
        .await
        .expect("reads by id")
        .expect("the entry exists");
    assert_eq!(on_day.first(), Some(&by_id));
}

#[tokio::test]
async fn cascade_delete_removes_child_rows() {
    let (_dir, connection, repository) = repository().await;
    let saved = repository.save(entry(31, "Newest")).await.expect("saves");

    entries::Entity::delete_by_id(saved.id.to_text())
        .exec(&connection)
        .await
        .expect("the parent row deletes");

    assert!(
        photos::Entity::find()
            .all(&connection)
            .await
            .expect("photos read")
            .is_empty()
    );
    assert!(
        stickers::Entity::find()
            .all(&connection)
            .await
            .expect("stickers read")
            .is_empty()
    );
    assert!(
        tags::Entity::find()
            .all(&connection)
            .await
            .expect("tags read")
            .is_empty()
    );
}

#[tokio::test]
async fn delete_all_leaves_four_empty_tables() {
    let (_dir, connection, repository) = repository().await;
    repository.save(entry(30, "Middle")).await.expect("saves");
    repository.save(entry(31, "Newest")).await.expect("saves");

    repository.delete_all().await.expect("truncates");

    assert!(
        entries::Entity::find()
            .all(&connection)
            .await
            .expect("entries read")
            .is_empty()
    );
    assert!(
        photos::Entity::find()
            .all(&connection)
            .await
            .expect("photos read")
            .is_empty()
    );
    assert!(
        stickers::Entity::find()
            .all(&connection)
            .await
            .expect("stickers read")
            .is_empty()
    );
    assert!(
        tags::Entity::find()
            .all(&connection)
            .await
            .expect("tags read")
            .is_empty()
    );
}
