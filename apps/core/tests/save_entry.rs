use chrono::{NaiveDate, Utc};
use leafypuff_core::application::SaveEntry;
use leafypuff_core::domain::{CoreError, Entry, EntryId, Mood};
use leafypuff_core::infrastructure::InMemoryEntryRepository;

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
    let use_case = SaveEntry::new(InMemoryEntryRepository::new());
    let saved = use_case
        .execute(entry("Quiet morning", "Tea on the balcony."))
        .await
        .expect("a valid entry should save");

    assert_eq!(saved.title, "Quiet morning");
    assert_eq!(saved.mood, Mood::Calm);
}

#[tokio::test]
async fn an_entry_with_no_title_and_no_body_is_rejected_before_it_reaches_storage() {
    let use_case = SaveEntry::new(InMemoryEntryRepository::new());
    let result = use_case.execute(entry("   ", "")).await;

    assert!(matches!(result, Err(CoreError::Invalid(_))));
}
