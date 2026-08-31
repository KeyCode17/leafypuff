#![cfg(feature = "sqlite")]

use chrono::{NaiveDate, TimeZone, Utc};
use leafypuff_core::domain::crypto::{KeyVault, RecoveryCode};
use leafypuff_core::domain::{CoreError, Entry, EntryId, EntryRepository, Mood};
use leafypuff_core::infrastructure::entity::entries;
use leafypuff_core::infrastructure::{SqliteEntryRepository, SqliteVaultStore, VaultSealer, db};
use sea_orm::{DatabaseConnection, EntityTrait};

const PASSPHRASE: &str = "the roof leaked again";
const TITLE: &str = "a title nobody else should read";
const BODY: &str = "and a body that is just as private";

async fn open_device() -> (tempfile::TempDir, DatabaseConnection) {
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
    (dir, connection)
}

fn entry() -> Entry {
    let at = Utc
        .with_ymd_and_hms(2026, 9, 1, 8, 0, 0)
        .single()
        .expect("an unambiguous instant");
    Entry {
        id: EntryId::new(),
        date: NaiveDate::from_ymd_opt(2026, 9, 1).expect("a real date"),
        mood: Mood::Calm,
        title: TITLE.to_owned(),
        body: BODY.to_owned(),
        tags: vec!["rain".to_owned()],
        weather: None,
        location: None,
        photos: Vec::new(),
        stickers: Vec::new(),
        created_at: at,
        updated_at: at,
    }
}

#[tokio::test]
async fn a_saved_entry_leaves_no_plaintext_in_the_device_database() {
    let (_dir, connection) = open_device().await;
    let code = RecoveryCode::generate().expect("a recovery code");
    let (vault, key) = KeyVault::create(PASSPHRASE, &code).expect("the vault builds");
    SqliteVaultStore::new(connection.clone())
        .create(&vault)
        .await
        .expect("the vault stores");
    let sealer = VaultSealer::new();
    sealer.unlock(key).expect("the sealer unlocks");
    let repository = SqliteEntryRepository::new(connection.clone(), sealer);

    let saved = repository.save(entry()).await.expect("the entry saves");

    let row = entries::Entity::find_by_id(saved.id.to_text())
        .one(&connection)
        .await
        .expect("the row query runs")
        .expect("the row exists");
    assert!(
        !row.title
            .windows(TITLE.len())
            .any(|w| w == TITLE.as_bytes())
    );
    assert!(!row.body.windows(BODY.len()).any(|w| w == BODY.as_bytes()));
    assert!(row.title_nonce.is_some());
    assert!(row.body_nonce.is_some());

    let read_back = repository
        .by_id(saved.id)
        .await
        .expect("the read runs")
        .expect("the entry comes back");
    assert_eq!(read_back.title, TITLE);
    assert_eq!(read_back.body, BODY);
}

#[tokio::test]
async fn a_locked_device_refuses_to_write_rather_than_storing_plaintext() {
    let (_dir, connection) = open_device().await;
    let repository = SqliteEntryRepository::new(connection.clone(), VaultSealer::new());

    let refused = repository
        .save(entry())
        .await
        .expect_err("a locked device must refuse the write");

    assert!(matches!(refused, CoreError::Locked));
    assert_eq!(
        entries::Entity::find()
            .all(&connection)
            .await
            .expect("the count runs")
            .len(),
        0
    );
}

#[tokio::test]
async fn the_recovery_code_opens_the_same_entries_as_the_passphrase() {
    let (_dir, connection) = open_device().await;
    let code = RecoveryCode::generate().expect("a recovery code");
    let (vault, key) = KeyVault::create(PASSPHRASE, &code).expect("the vault builds");
    let store = SqliteVaultStore::new(connection.clone());
    store.create(&vault).await.expect("the vault stores");
    let writer = VaultSealer::new();
    writer.unlock(key).expect("the sealer unlocks");
    let saved = SqliteEntryRepository::new(connection.clone(), writer)
        .save(entry())
        .await
        .expect("the entry saves");

    let recovered = store.read().await.expect("the vault reads back");
    let reader = VaultSealer::new();
    reader
        .unlock(
            recovered
                .unlock_with_recovery_code(&code)
                .expect("the recovery code opens the vault"),
        )
        .expect("the sealer unlocks");

    let read_back = SqliteEntryRepository::new(connection, reader)
        .by_id(saved.id)
        .await
        .expect("the read runs")
        .expect("the entry comes back");
    assert_eq!(read_back.title, TITLE);
}
