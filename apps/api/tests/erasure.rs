use api_migration::{Migrator, MigratorTrait};
use leafypuff_api::domain::iam::{Account, AccountRepository};
use leafypuff_api::domain::privacy::{Eraser, PrivacyError};
use leafypuff_api::domain::rbac::{AuditAction, AuditEvent, AuditLog};
use leafypuff_api::infrastructure::iam::PgAccountRepository;
use leafypuff_api::infrastructure::privacy::PgEraser;
use leafypuff_api::infrastructure::rbac::PgAuditLog;
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

async fn subject(connection: &DatabaseConnection, account_id: Uuid, email: &str) -> Uuid {
    let subject_id = Uuid::new_v4();
    connection
        .execute(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            "INSERT INTO audit_subjects (id, account_id, email) VALUES ($1, $2, $3)",
            [subject_id.into(), account_id.into(), email.into()],
        ))
        .await
        .expect("the subject inserts");
    subject_id
}

async fn count(connection: &DatabaseConnection, sql: &str, account_id: Uuid) -> i64 {
    connection
        .query_one(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            sql,
            [account_id.into()],
        ))
        .await
        .expect("the count runs")
        .expect("the count returns a row")
        .try_get("", "total")
        .expect("the count is a bigint")
}

#[tokio::test]
async fn erasure_removes_the_person_and_leaves_the_operator_record_standing() {
    let Some(connection) = connect().await else {
        return;
    };
    let email = format!("{}@leafypuff.test", Uuid::new_v4().simple());
    let owner = PgAccountRepository::new(connection.clone())
        .insert(Account {
            id: Uuid::new_v4(),
            email: email.clone(),
            password_hash: "$argon2id$v=19$m=19456,t=2,p=1$c2FsdA$aGFzaA".to_owned(),
            display_name: Some("Daffa".to_owned()),
            email_verified_at: None,
        })
        .await
        .expect("the account lands");
    let subject_id = subject(&connection, owner.id, &email).await;

    let actor = Uuid::new_v4();
    let audit = PgAuditLog::new(connection.clone());
    audit
        .record(AuditEvent {
            id: Uuid::new_v4(),
            actor_id: actor,
            action: AuditAction::AccountSuspended,
            subject_id: Some(subject_id),
            detail: owner.id.to_string(),
            recorded_at_ms: 1_756_000_000_000,
        })
        .await
        .expect("the audit row lands");

    PgEraser::new(connection.clone())
        .erase(owner.id)
        .await
        .expect("the erasure runs");

    assert_eq!(
        count(
            &connection,
            "SELECT count(*) AS total FROM accounts WHERE id = $1",
            owner.id
        )
        .await,
        0
    );
    assert_eq!(
        count(
            &connection,
            "SELECT count(*) AS total FROM audit_subjects WHERE account_id = $1",
            owner.id
        )
        .await,
        0,
        "the identity mapping must be nulled"
    );

    let surviving = connection
        .query_one(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            "SELECT count(*) AS total FROM audit_events WHERE subject_id = $1",
            [subject_id.into()],
        ))
        .await
        .expect("the count runs")
        .expect("the count returns a row");
    let total: i64 = surviving.try_get("", "total").expect("a bigint");
    assert_eq!(total, 1, "no audit row is ever deleted");

    let identity = connection
        .query_one(Statement::from_sql_and_values(
            DatabaseBackend::Postgres,
            "SELECT email FROM audit_subjects WHERE id = $1",
            [subject_id.into()],
        ))
        .await
        .expect("the query runs")
        .expect("the subject row survives");
    let held: Option<String> = identity.try_get("", "email").expect("a nullable text");
    assert!(held.is_none(), "the address must not survive the erasure");
}

#[tokio::test]
async fn erasing_an_account_that_is_already_gone_is_not_an_error() {
    let Some(connection) = connect().await else {
        return;
    };

    let outcome: Result<(), PrivacyError> = PgEraser::new(connection).erase(Uuid::new_v4()).await;

    assert!(outcome.is_ok());
}
