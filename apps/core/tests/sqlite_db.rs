#![cfg(feature = "sqlite")]

use leafypuff_core::infrastructure::db;
use sea_orm::{ConnectionTrait, DatabaseBackend};
use sea_orm_migration::MigratorTrait;

fn temp_db() -> (tempfile::TempDir, String) {
    let dir = tempfile::tempdir().expect("a temp dir");
    let path = dir.path().join("diary.sqlite");
    let text = path.to_string_lossy().into_owned();
    (dir, text)
}

#[tokio::test]
async fn the_device_connection_is_sqlite() {
    let (_dir, path) = temp_db();
    let connection = db::open(&path).await.expect("a temp file opens");
    assert_eq!(connection.get_database_backend(), DatabaseBackend::Sqlite);
}

#[tokio::test]
async fn migrations_run_on_empty_file() {
    let (_dir, path) = temp_db();
    let connection = db::open(&path).await.expect("a temp file opens");

    db::run_migrations(&connection)
        .await
        .expect("the first run applies");
    let after_first = core_migration::Migrator::get_applied_migrations(&connection)
        .await
        .expect("applied migrations are readable");

    db::run_migrations(&connection)
        .await
        .expect("the second run is a no-op");
    let after_second = core_migration::Migrator::get_applied_migrations(&connection)
        .await
        .expect("applied migrations are readable");

    assert_eq!(after_first.len(), after_second.len());
    assert!(
        core_migration::Migrator::get_pending_migrations(&connection)
            .await
            .expect("pending migrations are readable")
            .is_empty()
    );
}
