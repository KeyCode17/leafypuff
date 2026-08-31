use std::sync::Arc;

use api_migration::{Migrator, MigratorTrait};
use leafypuff_api::application::sync::SyncServices;
use leafypuff_api::domain::iam::{Account, AccountRepository};
use leafypuff_api::domain::sync::{EntryRecord, FieldEnvelope, SyncCursor};
use leafypuff_api::infrastructure::iam::PgAccountRepository;
use leafypuff_api::infrastructure::sync::{
    PgCheckpointStore, PgConflictSink, PgEntryStore, PgIdempotencyStore, PgWrappedKeyStore,
};
use leafypuff_core::domain::Mood;
use sea_orm::{ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, Statement};
use tokio::sync::OnceCell;
use uuid::Uuid;

static MIGRATED: OnceCell<()> = OnceCell::const_new();

async fn connect() -> Option<DatabaseConnection> {
    let Ok(url) = std::env::var("TEST_DATABASE_URL") else {
        eprintln!("skipped: TEST_DATABASE_URL is unset. CI always sets it and fails when missing.");
        return None;
    };
    let connection = Database::connect(&url)
        .await
        .expect("the test database must accept a connection");
    MIGRATED
        .get_or_init(|| async {
            Migrator::up(&connection, None)
                .await
                .expect("the migrations must apply");
        })
        .await;
    Some(connection)
}

async fn world() -> Option<(SyncServices, Uuid, DatabaseConnection)> {
    let connection = connect().await?;
    let accounts = PgAccountRepository::new(connection.clone());
    let owner = accounts
        .insert(Account {
            id: Uuid::new_v4(),
            email: format!("{}@leafypuff.test", Uuid::new_v4().simple()),
            password_hash: "$argon2id$v=19$m=19456,t=2,p=1$c2FsdA$aGFzaA".to_owned(),
            display_name: None,
            email_verified_at: None,
        })
        .await
        .expect("the owning account must land");

    let services = SyncServices {
        entries: Arc::new(PgEntryStore::new(connection.clone())),
        checkpoints: Arc::new(PgCheckpointStore::new(connection.clone())),
        idempotency: Arc::new(PgIdempotencyStore::new(connection.clone())),
        conflicts: Arc::new(PgConflictSink::new(connection.clone())),
        keys: Arc::new(PgWrappedKeyStore::new(connection.clone())),
    };
    Some((services, owner.id, connection))
}

fn envelope(ciphertext: &[u8], updated_at_ms: i64, device_id: Uuid) -> FieldEnvelope {
    FieldEnvelope {
        ciphertext: ciphertext.to_vec(),
        nonce: vec![7_u8; 24],
        updated_at_ms,
        device_id,
    }
}

fn record(account_id: Uuid, id: Uuid, device_id: Uuid, at: i64) -> EntryRecord {
    EntryRecord {
        id,
        account_id,
        date: "2026-09-01".to_owned(),
        mood: Mood::Calm,
        tags: vec!["rain".to_owned()],
        sticker_placements: "[]".to_owned(),
        revision: 0,
        device_updated_at_ms: at,
        deleted_at_ms: None,
        title: envelope(b"sealed-title", at, device_id),
        body: envelope(b"sealed-body", at, device_id),
    }
}

async fn conflict_count(connection: &DatabaseConnection, account_id: Uuid) -> i64 {
    let row = connection
        .query_one(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            "SELECT count(*) AS total FROM sync_field_conflicts WHERE account_id = $1",
            [account_id.into()],
        ))
        .await
        .expect("the count must run")
        .expect("the count returns a row");
    row.try_get("", "total").expect("the count is a bigint")
}

#[tokio::test]
async fn pull_returns_rows_above_the_cursor_in_revision_order() {
    let Some((sync, account_id, _connection)) = world().await else {
        return;
    };
    let device = Uuid::new_v4();
    let first = Uuid::new_v4();
    let second = Uuid::new_v4();

    sync.push()
        .execute(
            account_id,
            device,
            &Uuid::new_v4().to_string(),
            vec![
                record(account_id, first, device, 1_000),
                record(account_id, second, device, 2_000),
            ],
        )
        .await
        .expect("the batch must land");

    let pulled = sync
        .pull()
        .execute(account_id, Uuid::new_v4(), Some(SyncCursor::START))
        .await
        .expect("the pull must run");

    let ids: Vec<Uuid> = pulled.records.iter().map(|row| row.id).collect();
    assert_eq!(ids, vec![first, second]);
    assert!(pulled.cursor > SyncCursor::START);
}

#[tokio::test]
async fn a_tombstone_survives_a_later_live_write() {
    let Some((sync, account_id, _connection)) = world().await else {
        return;
    };
    let device = Uuid::new_v4();
    let id = Uuid::new_v4();

    let mut deleted = record(account_id, id, device, 1_000);
    deleted.deleted_at_ms = Some(1_500);
    sync.push()
        .execute(
            account_id,
            device,
            &Uuid::new_v4().to_string(),
            vec![deleted],
        )
        .await
        .expect("the tombstone must land");

    sync.push()
        .execute(
            account_id,
            device,
            &Uuid::new_v4().to_string(),
            vec![record(account_id, id, device, 3_000)],
        )
        .await
        .expect("the later write must land");

    let pulled = sync
        .pull()
        .execute(account_id, Uuid::new_v4(), Some(SyncCursor::START))
        .await
        .expect("the pull must run");
    let found = pulled
        .records
        .into_iter()
        .find(|row| row.id == id)
        .expect("the row must come back");
    assert!(found.is_tombstoned());
}

#[tokio::test]
async fn an_identical_replayed_batch_produces_exactly_one_write() {
    let Some((sync, account_id, connection)) = world().await else {
        return;
    };
    let device = Uuid::new_v4();
    let id = Uuid::new_v4();
    let key = Uuid::new_v4().to_string();

    let first = sync
        .push()
        .execute(
            account_id,
            device,
            &key,
            vec![record(account_id, id, device, 1_000)],
        )
        .await
        .expect("the first push must land");
    let conflicts_after_first = conflict_count(&connection, account_id).await;

    let replay = sync
        .push()
        .execute(
            account_id,
            device,
            &key,
            vec![record(account_id, id, device, 1_000)],
        )
        .await
        .expect("the replay must be accepted");

    assert!(!first.replayed);
    assert!(replay.replayed);
    assert!(replay.applied.is_empty());
    assert_eq!(replay.cursor, first.cursor);
    assert_eq!(
        conflict_count(&connection, account_id).await,
        conflicts_after_first
    );
}

#[tokio::test]
async fn a_conflict_row_holds_a_hash_and_a_byte_length_and_no_text() {
    let Some((sync, account_id, connection)) = world().await else {
        return;
    };
    let id = Uuid::new_v4();
    let early = Uuid::from_u128(1);
    let late = Uuid::from_u128(2);

    let mut held = record(account_id, id, late, 2_000);
    held.title = envelope(b"winning-title", 2_000, late);
    sync.push()
        .execute(account_id, late, &Uuid::new_v4().to_string(), vec![held])
        .await
        .expect("the winner must land");

    let mut loser = record(account_id, id, early, 1_000);
    loser.title = envelope(b"losing-title", 1_000, early);
    sync.push()
        .execute(account_id, early, &Uuid::new_v4().to_string(), vec![loser])
        .await
        .expect("the loser must land");

    let row = connection
        .query_one(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            "SELECT loser_ciphertext_hash, loser_byte_len, field FROM sync_field_conflicts \
             WHERE account_id = $1 AND field = 'title'",
            [account_id.into()],
        ))
        .await
        .expect("the query must run")
        .expect("a conflict row must exist");

    let hash: String = row.try_get("", "loser_ciphertext_hash").expect("a hash");
    let byte_len: i64 = row.try_get("", "loser_byte_len").expect("a length");
    assert_eq!(hash.len(), 64);
    assert!(hash.chars().all(|glyph| glyph.is_ascii_hexdigit()));
    assert_eq!(
        byte_len,
        i64::try_from(b"losing-title".len()).expect("the length fits")
    );
    assert!(!hash.contains("losing"));
}
