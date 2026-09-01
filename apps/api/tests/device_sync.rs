use std::sync::Arc;

use api_migration::{Migrator, MigratorTrait};
use api_testing::admin_repositories::{InMemoryDirectory, InMemoryMetrics};
use api_testing::catalog_repositories::InMemoryCatalog;
use api_testing::media_repositories::{InMemoryMedia, InMemoryObjects};
use api_testing::privacy_repositories::{InMemoryRequests, RecordingEraser};
use api_testing::rbac_repositories::{InMemoryAudit, InMemoryRoles};
use api_testing::release_repositories::{InMemoryCampaigns, InMemoryGates};
use leafypuff_api::application::admin::AdminServices;
use leafypuff_api::application::catalog::CatalogServices;
use leafypuff_api::application::iam::IamServices;
use leafypuff_api::application::media::MediaServices;
use leafypuff_api::application::privacy::PrivacyServices;
use leafypuff_api::application::rbac::RbacServices;
use leafypuff_api::application::release::ReleaseServices;
use leafypuff_api::application::sync::SyncServices;
use leafypuff_api::domain::iam::{Account, AccountRepository, TokenIssuer, TokenVerifier};
use leafypuff_api::http::{AppState, build_router};
use leafypuff_api::infrastructure::DependencyProbe;
use leafypuff_api::infrastructure::iam::{
    Argon2Hasher, Blake3Otp, JwtTokenIssuer, PgAccountRepository, PgOtpRepository,
    PgRefreshTokenRepository, ResendEmailSender, SystemClock,
};
use leafypuff_api::infrastructure::sync::{
    PgCheckpointStore, PgConflictSink, PgEntryStore, PgIdempotencyStore, PgWrappedKeyStore,
};
use leafypuff_core::domain::crypto::{FIELD_TITLE, FieldContext, KeyVault, RecoveryCode, open};
use leafypuff_core::domain::{Entry, EntryId, EntryRepository, Mood};
use leafypuff_core::infrastructure::{
    SqliteEntryRepository, SqliteVaultStore, SyncClient, SyncOutbox, VaultSealer, db,
};
use sea_orm::{Database, DatabaseConnection};
use tokio::sync::OnceCell;
use uuid::Uuid;

const SIGNING_SECRET: &str = "a-thirty-two-byte-signing-secret!";
const PASSPHRASE: &str = "a decent sync passphrase";
const TITLE: &str = "the roof leaked again";
const BODY: &str = "but the rain smelled like the old house";

static MIGRATED: OnceCell<()> = OnceCell::const_new();

async fn server() -> Option<(String, DatabaseConnection, Uuid, String)> {
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

    let owner = PgAccountRepository::new(connection.clone())
        .insert(Account {
            id: Uuid::new_v4(),
            email: format!("{}@leafypuff.test", Uuid::new_v4().simple()),
            password_hash: "$argon2id$v=19$m=19456,t=2,p=1$c2FsdA$aGFzaA".to_owned(),
            display_name: None,
            email_verified_at: None,
        })
        .await
        .expect("the owning account lands");

    let issuer = Arc::new(JwtTokenIssuer::new(SIGNING_SECRET));
    let token = issuer.access_token(owner.id).expect("a token mints");
    let roles = InMemoryRoles::default();
    let audit = InMemoryAudit::default();
    let rbac = RbacServices {
        roles: Arc::new(roles.clone()),
        permissions: Arc::new(roles),
        audit: Arc::new(audit.clone()),
    };
    let state = AppState {
        readiness: DependencyProbe::new(url, "127.0.0.1:3900".to_owned()),
        iam: IamServices {
            accounts: Arc::new(PgAccountRepository::new(connection.clone())),
            credentials: Arc::new(PgRefreshTokenRepository::new(connection.clone())),
            otps: Arc::new(PgOtpRepository::new(connection.clone())),
            hasher: Arc::new(Argon2Hasher::new().expect("the hasher builds")),
            tokens: Arc::clone(&issuer) as Arc<dyn TokenIssuer>,
            verifier: issuer as Arc<dyn TokenVerifier>,
            generator: Arc::new(Blake3Otp::new([0_u8; 32])),
            mail: Arc::new(
                ResendEmailSender::new("re_unused".to_owned(), "unused".to_owned())
                    .expect("the mailer builds"),
            ),
            clock: Arc::new(SystemClock),
        },
        sync: SyncServices {
            entries: Arc::new(PgEntryStore::new(connection.clone())),
            checkpoints: Arc::new(PgCheckpointStore::new(connection.clone())),
            idempotency: Arc::new(PgIdempotencyStore::new(connection.clone())),
            conflicts: Arc::new(PgConflictSink::new(connection.clone())),
            keys: Arc::new(PgWrappedKeyStore::new(connection.clone())),
        },
        media: MediaServices {
            objects: Arc::new(InMemoryObjects::default()),
            media: Arc::new(InMemoryMedia::default()),
        },
        rbac: rbac.clone(),
        admin: AdminServices {
            directory: Arc::new(InMemoryDirectory::default()),
            metrics: Arc::new(InMemoryMetrics::default()),
            audit: Arc::new(audit.clone()),
            rbac: rbac.clone(),
        },
        catalog: CatalogServices {
            store: Arc::new(InMemoryCatalog::default()),
            audit: Arc::new(audit.clone()),
            rbac: rbac.clone(),
        },
        privacy: PrivacyServices {
            requests: Arc::new(InMemoryRequests::default()),
            eraser: Arc::new(RecordingEraser::default()),
            objects: Arc::new(InMemoryObjects::default()),
            audit: Arc::new(audit.clone()),
            rbac: rbac.clone(),
        },
        release: ReleaseServices {
            gates: Arc::new(InMemoryGates::default()),
            campaigns: Arc::new(InMemoryCampaigns::default()),
            audit: Arc::new(audit),
            rbac,
        },
    };

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("an ephemeral port binds");
    let address = listener.local_addr().expect("the port is known");
    tokio::spawn(async move {
        let _ = axum::serve(listener, build_router(state)).await;
    });

    Some((format!("http://{address}"), connection, owner.id, token))
}

struct Device {
    _dir: tempfile::TempDir,
    entries: SqliteEntryRepository<VaultSealer>,
    outbox: SyncOutbox,
}

async fn device(vault: Option<KeyVault>) -> (Device, KeyVault) {
    let dir = tempfile::tempdir().expect("a temp dir");
    let path = dir
        .path()
        .join("diary.sqlite")
        .to_string_lossy()
        .into_owned();
    let connection = db::open(&path).await.expect("the device database opens");
    db::run_migrations(&connection)
        .await
        .expect("the device migrations apply");

    let held = match vault {
        Some(existing) => existing,
        None => {
            let code = RecoveryCode::generate().expect("a recovery code");
            KeyVault::create(PASSPHRASE, &code)
                .expect("the vault builds")
                .0
        }
    };
    let key = held
        .unlock_with_passphrase(PASSPHRASE)
        .expect("the passphrase opens the vault");
    SqliteVaultStore::new(connection.clone())
        .create(&held)
        .await
        .expect("the vault stores");
    let sealer = VaultSealer::new();
    sealer.unlock(key).expect("the sealer unlocks");

    (
        Device {
            _dir: dir,
            entries: SqliteEntryRepository::new(connection.clone(), sealer),
            outbox: SyncOutbox::new(connection),
        },
        held,
    )
}

fn entry(id: EntryId, at: chrono::DateTime<chrono::Utc>) -> Entry {
    Entry {
        id,
        date: chrono::NaiveDate::from_ymd_opt(2026, 9, 1).expect("a real date"),
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
async fn an_entry_written_on_one_device_opens_on_another_through_the_real_client() {
    let Some((base_url, _connection, _account_id, token)) = server().await else {
        return;
    };
    let (writer, vault) = device(None).await;
    let (reader, _) = device(Some(vault)).await;

    let at = chrono::Utc::now();
    let entry_id = EntryId::new();
    writer
        .entries
        .save(entry(entry_id, at))
        .await
        .expect("the entry saves locally");

    let pushed = SyncClient::new(base_url.clone(), token.clone())
        .expect("the client builds")
        .exchange(&writer.outbox)
        .await
        .expect("the writer exchanges");
    assert_eq!(pushed.pushed, 1);

    let pulled = SyncClient::new(base_url, token)
        .expect("the client builds")
        .exchange(&reader.outbox)
        .await
        .expect("the reader exchanges");
    assert!(pulled.pulled >= 1);

    let arrived = reader
        .entries
        .by_id(entry_id)
        .await
        .expect("the read runs")
        .expect("the entry crossed");

    assert_eq!(arrived.title, TITLE);
    assert_eq!(arrived.body, BODY);
    assert_eq!(arrived.mood, Mood::Calm);
}

#[tokio::test]
async fn a_second_exchange_pushes_nothing_it_already_sent() {
    let Some((base_url, _connection, _account_id, token)) = server().await else {
        return;
    };
    let (writer, _vault) = device(None).await;
    let at = chrono::Utc::now();
    writer
        .entries
        .save(entry(EntryId::new(), at))
        .await
        .expect("the entry saves locally");

    let first = SyncClient::new(base_url.clone(), token.clone())
        .expect("the client builds")
        .exchange(&writer.outbox)
        .await
        .expect("the first exchange runs");
    let second = SyncClient::new(base_url, token)
        .expect("the client builds")
        .exchange(&writer.outbox)
        .await
        .expect("the second exchange runs");

    assert_eq!(first.pushed, 1);
    assert_eq!(second.pushed, 0, "a synced row must not upload again");
}

#[tokio::test]
async fn the_ciphertext_that_crossed_is_the_one_the_writer_sealed() {
    let Some((base_url, _connection, _account_id, token)) = server().await else {
        return;
    };
    let (writer, vault) = device(None).await;
    let (reader, reader_vault) = device(Some(vault)).await;
    let reader_key = reader_vault
        .unlock_with_passphrase(PASSPHRASE)
        .expect("the second device opens the same vault");

    let at = chrono::Utc::now();
    let entry_id = EntryId::new();
    writer
        .entries
        .save(entry(entry_id, at))
        .await
        .expect("the entry saves locally");
    SyncClient::new(base_url.clone(), token.clone())
        .expect("the client builds")
        .exchange(&writer.outbox)
        .await
        .expect("the writer exchanges");
    SyncClient::new(base_url, token)
        .expect("the client builds")
        .exchange(&reader.outbox)
        .await
        .expect("the reader exchanges");

    let arrived = reader
        .entries
        .by_id(entry_id)
        .await
        .expect("the read runs")
        .expect("the entry crossed");
    let context = FieldContext {
        entry_id,
        field_name: FIELD_TITLE,
        field_updated_at_ms: arrived.updated_at.timestamp_millis(),
    };
    let sealed = leafypuff_core::domain::crypto::seal(&reader_key, &context, TITLE.as_bytes())
        .expect("the reader can seal under the same key");
    let reopened = open(&reader_key, &context, &sealed).expect("and open it again");

    assert_eq!(reopened, TITLE.as_bytes());
    assert_eq!(arrived.title, TITLE);
}
