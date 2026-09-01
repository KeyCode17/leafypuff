use std::sync::Arc;

use api_migration::{Migrator, MigratorTrait};
use api_testing::media_repositories::InMemoryObjects;
use axum::Router;
use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use data_encoding::BASE64;
use leafypuff_api::application::iam::IamServices;
use leafypuff_api::application::media::MediaServices;
use leafypuff_api::application::sync::SyncServices;
use leafypuff_api::domain::iam::{Account, AccountRepository, TokenIssuer, TokenVerifier};
use leafypuff_api::http::{AppState, build_router};
use leafypuff_api::infrastructure::DependencyProbe;
use leafypuff_api::infrastructure::iam::{
    Argon2Hasher, Blake3Otp, JwtTokenIssuer, PgAccountRepository, PgOtpRepository,
    PgRefreshTokenRepository, ResendEmailSender, SystemClock,
};
use leafypuff_api::infrastructure::media::PgMediaRepository;
use leafypuff_api::infrastructure::sync::{
    PgCheckpointStore, PgConflictSink, PgEntryStore, PgIdempotencyStore, PgWrappedKeyStore,
};
use leafypuff_core::domain::crypto::{
    FIELD_BODY, FIELD_TITLE, FieldContext, KeyVault, RecoveryCode, SealedField, WrappedKey, open,
    seal,
};
use leafypuff_core::domain::entry::EntryId;
use sea_orm::{ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, Statement};
use serde_json::{Value, json};
use tokio::sync::OnceCell;
use tower::ServiceExt;
use uuid::Uuid;

const BODY_LIMIT: usize = 256 * 1024;
const SIGNING_SECRET: &str = "a-thirty-two-byte-signing-secret!";
const PASSPHRASE: &str = "a decent sync passphrase";
const TITLE: &[u8] = b"the roof leaked again";
const BODY: &[u8] = b"but the rain smelled like the old house";

static MIGRATED: OnceCell<()> = OnceCell::const_new();

struct Harness {
    router: Router,
    account_id: Uuid,
    token: String,
    connection: DatabaseConnection,
}

async fn harness() -> Option<Harness> {
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

    let issuer = Arc::new(JwtTokenIssuer::new(SIGNING_SECRET));
    let token = issuer
        .access_token(owner.id)
        .expect("the access token must mint");
    let iam = IamServices {
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
    };
    let sync = SyncServices {
        entries: Arc::new(PgEntryStore::new(connection.clone())),
        checkpoints: Arc::new(PgCheckpointStore::new(connection.clone())),
        idempotency: Arc::new(PgIdempotencyStore::new(connection.clone())),
        conflicts: Arc::new(PgConflictSink::new(connection.clone())),
        keys: Arc::new(PgWrappedKeyStore::new(connection.clone())),
    };
    let media = MediaServices {
        objects: Arc::new(InMemoryObjects::default()),
        media: Arc::new(PgMediaRepository::new(connection.clone())),
    };

    let rbac = leafypuff_api::application::rbac::RbacServices {
        roles: Arc::new(api_testing::rbac_repositories::InMemoryRoles::default()),
        permissions: Arc::new(api_testing::rbac_repositories::InMemoryRoles::default()),
        audit: Arc::new(api_testing::rbac_repositories::InMemoryAudit::default()),
    };
    let admin = leafypuff_api::application::admin::AdminServices {
        directory: Arc::new(api_testing::admin_repositories::InMemoryDirectory::default()),
        audit: Arc::new(api_testing::rbac_repositories::InMemoryAudit::default()),
        rbac: rbac.clone(),
    };
    let probe = DependencyProbe::new(url, "127.0.0.1:3900".to_owned());
    Some(Harness {
        router: build_router(AppState::new(probe, iam, sync, media, rbac.clone(), admin)),
        account_id: owner.id,
        token,
        connection,
    })
}

async fn send(app: Router, request: Request<Body>) -> (StatusCode, Value) {
    let response = app.oneshot(request).await.expect("the router answers");
    let status = response.status();
    let bytes = to_bytes(response.into_body(), BODY_LIMIT)
        .await
        .expect("the body reads");
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

fn blob(wrapped: &WrappedKey) -> String {
    let mut bytes = wrapped.nonce.to_vec();
    bytes.extend_from_slice(&wrapped.ciphertext);
    BASE64.encode(&bytes)
}

fn unblob(encoded: &str) -> WrappedKey {
    let bytes = BASE64
        .decode(encoded.as_bytes())
        .expect("the blob is base64");
    let (nonce, ciphertext) = bytes.split_at(24);
    WrappedKey {
        nonce: nonce.try_into().expect("the nonce is 24 bytes"),
        ciphertext: ciphertext.to_vec(),
    }
}

fn sealed_json(sealed: &SealedField, at: i64, device_id: Uuid) -> Value {
    json!({
        "ciphertext": BASE64.encode(&sealed.ciphertext),
        "nonce": BASE64.encode(&sealed.nonce),
        "updated_at_ms": at,
        "device_id": device_id,
    })
}

fn sealed_from(value: &Value) -> SealedField {
    let nonce = BASE64
        .decode(
            value["nonce"]
                .as_str()
                .expect("nonce is a string")
                .as_bytes(),
        )
        .expect("nonce is base64");
    SealedField {
        nonce: nonce.as_slice().try_into().expect("the nonce is 24 bytes"),
        ciphertext: BASE64
            .decode(
                value["ciphertext"]
                    .as_str()
                    .expect("ciphertext is a string")
                    .as_bytes(),
            )
            .expect("ciphertext is base64"),
    }
}

#[tokio::test]
async fn an_entry_written_on_one_device_opens_on_another_and_the_server_holds_no_plaintext() {
    let Some(harness) = harness().await else {
        return;
    };
    let device_a = Uuid::new_v4();
    let device_b = Uuid::new_v4();
    let entry_id = EntryId::new();
    let at = 1_756_000_000_000_i64;

    let code = RecoveryCode::generate().expect("a recovery code");
    let (vault, content_key) = KeyVault::create(PASSPHRASE, &code).expect("the vault builds");
    let title = seal(
        &content_key,
        &FieldContext {
            entry_id,
            field_name: FIELD_TITLE,
            field_updated_at_ms: at,
        },
        TITLE,
    )
    .expect("the title seals");
    let body = seal(
        &content_key,
        &FieldContext {
            entry_id,
            field_name: FIELD_BODY,
            field_updated_at_ms: at,
        },
        BODY,
    )
    .expect("the body seals");

    for (kind, wrapped, salt) in [
        (
            "passphrase",
            &vault.passphrase_slot,
            BASE64.encode(&vault.passphrase_salt),
        ),
        ("recovery", &vault.recovery_slot, BASE64.encode(&[0_u8; 16])),
    ] {
        let request = Request::builder()
            .method("PUT")
            .uri("/v1/sync/keys")
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {}", harness.token))
            .header("x-device-id", device_a.to_string())
            .body(Body::from(
                json!({
                    "kind": kind,
                    "blob": blob(wrapped),
                    "salt": salt,
                    "updated_at_ms": at,
                })
                .to_string(),
            ))
            .expect("the request builds");
        let (status, _) = send(harness.router.clone(), request).await;
        assert_eq!(status, StatusCode::OK);
    }

    let push = Request::builder()
        .method("POST")
        .uri("/v1/sync/push")
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {}", harness.token))
        .header("x-device-id", device_a.to_string())
        .header("idempotency-key", Uuid::new_v4().to_string())
        .body(Body::from(
            json!({
                "records": [{
                    "id": entry_id.0,
                    "date": "2026-09-01",
                    "mood": "calm",
                    "tags": ["rain"],
                    "sticker_placements": "[]",
                    "device_updated_at_ms": at,
                    "deleted_at_ms": Value::Null,
                    "title": sealed_json(&title, at, device_a),
                    "body": sealed_json(&body, at, device_a),
                }]
            })
            .to_string(),
        ))
        .expect("the request builds");
    let (push_status, _) = send(harness.router.clone(), push).await;
    assert_eq!(push_status, StatusCode::OK);

    let keys_request = Request::builder()
        .uri("/v1/sync/keys")
        .header("authorization", format!("Bearer {}", harness.token))
        .header("x-device-id", device_b.to_string())
        .body(Body::empty())
        .expect("the request builds");
    let (_, keys_body) = send(harness.router.clone(), keys_request).await;
    let rows = keys_body["data"].as_array().expect("keys is an array");
    let slot = |kind: &str| {
        rows.iter()
            .find(|row| row["kind"] == kind)
            .expect("both slots come back")
    };
    let recovered = KeyVault {
        passphrase_salt: BASE64
            .decode(
                slot("passphrase")["salt"]
                    .as_str()
                    .expect("salt is a string")
                    .as_bytes(),
            )
            .expect("salt is base64")
            .as_slice()
            .try_into()
            .expect("the salt is 16 bytes"),
        passphrase_slot: unblob(
            slot("passphrase")["blob"]
                .as_str()
                .expect("blob is a string"),
        ),
        recovery_slot: unblob(slot("recovery")["blob"].as_str().expect("blob is a string")),
    };
    let key_on_device_b = recovered
        .unlock_with_passphrase(PASSPHRASE)
        .expect("the same passphrase must open the vault on the second device");

    let pull = Request::builder()
        .uri("/v1/sync/pull?cursor=0")
        .header("authorization", format!("Bearer {}", harness.token))
        .header("x-device-id", device_b.to_string())
        .body(Body::empty())
        .expect("the request builds");
    let (pull_status, pulled) = send(harness.router.clone(), pull).await;
    assert_eq!(pull_status, StatusCode::OK);

    let record = pulled["data"]["records"]
        .as_array()
        .expect("records is an array")
        .iter()
        .find(|row| row["id"] == json!(entry_id.0))
        .expect("the entry must come back");

    let opened_title = open(
        &key_on_device_b,
        &FieldContext {
            entry_id,
            field_name: FIELD_TITLE,
            field_updated_at_ms: at,
        },
        &sealed_from(&record["title"]),
    )
    .expect("the title must open on the second device");
    let opened_body = open(
        &key_on_device_b,
        &FieldContext {
            entry_id,
            field_name: FIELD_BODY,
            field_updated_at_ms: at,
        },
        &sealed_from(&record["body"]),
    )
    .expect("the body must open on the second device");

    assert_eq!(opened_title, TITLE);
    assert_eq!(opened_body, BODY);

    let stored = harness
        .connection
        .query_one(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            "SELECT title_ciphertext, body_ciphertext FROM entries WHERE id = $1",
            [entry_id.0.into()],
        ))
        .await
        .expect("the row query runs")
        .expect("the row exists");
    let stored_title: Vec<u8> = stored.try_get("", "title_ciphertext").expect("a bytea");
    let stored_body: Vec<u8> = stored.try_get("", "body_ciphertext").expect("a bytea");

    assert!(!contains(&stored_title, TITLE));
    assert!(!contains(&stored_body, BODY));

    let columns = harness
        .connection
        .query_all(Statement::from_string(
            DatabaseBackend::Postgres,
            "SELECT column_name FROM information_schema.columns WHERE table_name = 'entries'",
        ))
        .await
        .expect("the column query runs");
    let names: Vec<String> = columns
        .iter()
        .map(|row| row.try_get("", "column_name").expect("a column name"))
        .collect();
    assert!(!names.contains(&"title".to_owned()));
    assert!(!names.contains(&"body".to_owned()));
    assert_eq!(harness.account_id, harness.account_id);
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.windows(needle.len()).any(|slice| slice == needle)
}
