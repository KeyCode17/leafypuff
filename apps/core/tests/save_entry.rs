#![cfg(feature = "test-support")]

use chrono::{NaiveDate, TimeZone, Utc};
use leafypuff_core::application::SaveEntry;
use leafypuff_core::domain::{CoreError, Entry, EntryId, Mood};
use leafypuff_core::infrastructure::{FixedClock, InMemoryEntryRepository, SystemClock};

fn entry(title: &str, body: &str) -> Entry {
    let now = Utc::now();
    Entry {
        id: EntryId::new(),
        date: NaiveDate::from_ymd_opt(2026, 8, 31).expect("a real date"),
        mood: Mood::Calm,
        title: title.to_owned(),
        body: body.to_owned(),
        tags: vec!["#slowday".to_owned()],
        weather: None,
        location: None,
        photos: Vec::new(),
        stickers: Vec::new(),
        created_at: now,
        updated_at: now,
    }
}

#[tokio::test]
async fn a_written_entry_comes_back_from_the_repository() {
    let use_case = SaveEntry::new(InMemoryEntryRepository::new(), SystemClock);
    let saved = use_case
        .execute(entry("Quiet morning", "Tea on the balcony."))
        .await
        .expect("a valid entry should save");

    assert_eq!(saved.title, "Quiet morning");
    assert_eq!(saved.mood, Mood::Calm);
}

#[tokio::test]
async fn an_entry_with_no_title_and_no_body_is_rejected_before_it_reaches_storage() {
    let use_case = SaveEntry::new(InMemoryEntryRepository::new(), SystemClock);
    let result = use_case.execute(entry("   ", "")).await;

    assert!(matches!(result, Err(CoreError::Invalid(_))));
}

#[tokio::test]
async fn save_entry_stamps_created_at_once() {
    let first = Utc.with_ymd_and_hms(2026, 8, 31, 8, 0, 0).unwrap();
    let second = Utc.with_ymd_and_hms(2026, 8, 31, 21, 0, 0).unwrap();

    let repository = InMemoryEntryRepository::new();
    let draft = entry("Quiet morning", "Tea on the balcony.");
    let id = draft.id;

    let created = SaveEntry::new(&repository, FixedClock::new(first))
        .execute(draft.clone())
        .await
        .expect("a valid entry should save");
    assert_eq!(created.created_at, first);
    assert_eq!(created.updated_at, first);

    let updated = SaveEntry::new(&repository, FixedClock::new(second))
        .execute(Entry {
            title: "Quiet evening".to_owned(),
            ..draft
        })
        .await
        .expect("an existing entry should save");
    assert_eq!(updated.id, id);
    assert_eq!(updated.created_at, first);
    assert_eq!(updated.updated_at, second);
}

#[tokio::test]
async fn a_nil_id_is_rejected_before_it_reaches_storage() {
    let repository = InMemoryEntryRepository::new();
    let draft = Entry {
        id: EntryId(uuid::Uuid::nil()),
        ..entry("Title", "Body")
    };
    let result = SaveEntry::new(&repository, FixedClock::new(Utc::now()))
        .execute(draft)
        .await;

    assert!(matches!(result, Err(CoreError::Invalid(_))));
}
