use api_migration::{Migrator, MigratorTrait};
use chrono::{Duration, Utc};
use leafypuff_api::domain::iam::{
    Account, AccountRepository, IamError, OtpCode, OtpPurpose, OtpRepository, RefreshToken,
    RefreshTokenRepository,
};
use leafypuff_api::infrastructure::iam::{
    PgAccountRepository, PgOtpRepository, PgRefreshTokenRepository,
};
use sea_orm::{Database, DatabaseConnection};
use tokio::sync::OnceCell;
use uuid::Uuid;

static MIGRATED: OnceCell<()> = OnceCell::const_new();

async fn connect() -> Option<DatabaseConnection> {
    let Ok(url) = std::env::var("TEST_DATABASE_URL") else {
        eprintln!(
            "skipped: TEST_DATABASE_URL is unset. Point it at a throwaway postgres, for example \
             postgres:
             is missing, so this can never be silently skipped there."
        );
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

fn account(email: String) -> Account {
    Account {
        id: Uuid::new_v4(),
        email,
        password_hash: "$argon2id$v=19$m=19456,t=2,p=1$c2FsdA$aGFzaA".to_owned(),
        display_name: Some("Daffa".to_owned()),
        email_verified_at: None,
    }
}

fn unique_email() -> String {
    format!("{}@leafypuff.test", Uuid::new_v4().simple())
}

#[tokio::test]
async fn an_email_is_unique_regardless_of_its_case() {
    let Some(connection) = connect().await else {
        return;
    };
    let repository = PgAccountRepository::new(connection);
    let email = unique_email();
    repository
        .insert(account(email.clone()))
        .await
        .expect("the first registration must land");

    let conflict = repository
        .insert(account(email.to_uppercase()))
        .await
        .expect_err("the same address in another case must conflict");

    assert!(matches!(conflict, IamError::EmailAlreadyRegistered));
}

#[tokio::test]
async fn an_account_is_found_by_an_address_in_any_case() {
    let Some(connection) = connect().await else {
        return;
    };
    let repository = PgAccountRepository::new(connection);
    let email = unique_email();
    let stored = repository
        .insert(account(email.clone()))
        .await
        .expect("registration must land");

    let found = repository
        .by_email(&email.to_uppercase())
        .await
        .expect("the lookup must succeed")
        .expect("the account must be found");

    assert_eq!(found.id, stored.id);
    assert!(!found.is_verified());
}

#[tokio::test]
async fn marking_an_account_verified_records_when() {
    let Some(connection) = connect().await else {
        return;
    };
    let repository = PgAccountRepository::new(connection);
    let stored = repository
        .insert(account(unique_email()))
        .await
        .expect("registration must land");
    let at = Utc::now();

    repository
        .mark_verified(stored.id, at)
        .await
        .expect("the update must land");

    let found = repository
        .by_id(stored.id)
        .await
        .expect("the lookup must succeed")
        .expect("the account must be found");
    assert!(found.is_verified());
}

#[tokio::test]
async fn a_second_credential_for_one_device_revokes_the_first() {
    let Some(connection) = connect().await else {
        return;
    };
    let accounts = PgAccountRepository::new(connection.clone());
    let tokens = PgRefreshTokenRepository::new(connection);
    let owner = accounts
        .insert(account(unique_email()))
        .await
        .expect("registration must land");

    let replaced_hash = Uuid::new_v4().simple().to_string();
    let live_hash = Uuid::new_v4().simple().to_string();
    tokens
        .insert(credential(owner.id, "device-a", &replaced_hash))
        .await
        .expect("the first must land");
    tokens
        .insert(credential(owner.id, "device-a", &live_hash))
        .await
        .expect("the second must land");

    let replaced = tokens
        .by_hash(&replaced_hash)
        .await
        .expect("the lookup must succeed")
        .expect("the replaced row must survive");
    let live = tokens
        .by_hash(&live_hash)
        .await
        .expect("the lookup must succeed")
        .expect("the new row must be there");

    assert!(!replaced.is_usable(Utc::now()));
    assert!(live.is_usable(Utc::now()));
}

fn credential(account_id: Uuid, device_id: &str, token_hash: &str) -> RefreshToken {
    RefreshToken {
        id: Uuid::new_v4(),
        account_id,
        device_id: device_id.to_owned(),
        token_hash: token_hash.to_owned(),
        expires_at: Utc::now() + Duration::days(30),
        revoked_at: None,
    }
}

#[tokio::test]
async fn a_new_code_replaces_the_open_one_and_attempts_accumulate() {
    let Some(connection) = connect().await else {
        return;
    };
    let accounts = PgAccountRepository::new(connection.clone());
    let codes = PgOtpRepository::new(connection);
    let owner = accounts
        .insert(account(unique_email()))
        .await
        .expect("registration must land");

    codes
        .insert(challenge(owner.id, "digest-one"))
        .await
        .expect("the first code must land");
    codes
        .insert(challenge(owner.id, "digest-two"))
        .await
        .expect("the second code must replace it");

    let open = codes
        .open_for(owner.id, OtpPurpose::VerifyEmail)
        .await
        .expect("the lookup must succeed")
        .expect("one code must be open");
    assert_eq!(open.code_hash, "digest-two");

    codes
        .record_attempt(open.id)
        .await
        .expect("the attempt must be recorded");
    let after = codes
        .open_for(owner.id, OtpPurpose::VerifyEmail)
        .await
        .expect("the lookup must succeed")
        .expect("the code is still open");
    assert_eq!(after.attempts, 1);

    codes
        .consume(open.id, Utc::now())
        .await
        .expect("the code must close");
    let closed = codes
        .open_for(owner.id, OtpPurpose::VerifyEmail)
        .await
        .expect("the lookup must succeed");
    assert!(closed.is_none());
}

fn challenge(account_id: Uuid, code_hash: &str) -> OtpCode {
    OtpCode {
        id: Uuid::new_v4(),
        account_id,
        code_hash: code_hash.to_owned(),
        purpose: OtpPurpose::VerifyEmail,
        attempts: 0,
        expires_at: Utc::now() + Duration::minutes(10),
        consumed_at: None,
    }
}
