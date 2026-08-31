use core_migration::Migrator;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbErr, Statement};
use sea_orm_migration::MigratorTrait;

use crate::domain::CoreError;

pub const ERR_DATABASE: &str = "Database failure";

const PRAGMAS: [&str; 2] = ["PRAGMA journal_mode = WAL", "PRAGMA foreign_keys = ON"];

impl From<DbErr> for CoreError {
    fn from(error: DbErr) -> Self {
        Self::Storage(format!("{ERR_DATABASE}: {error}"))
    }
}

pub async fn open(path: &str) -> Result<DatabaseConnection, CoreError> {
    let mut options = ConnectOptions::new(format!("sqlite://{path}?mode=rwc"));
    options
        .max_connections(1)
        .min_connections(1)
        .sqlx_logging(false);

    let connection = Database::connect(options).await?;
    for pragma in PRAGMAS {
        let statement =
            Statement::from_string(connection.get_database_backend(), pragma.to_owned());
        connection.execute(statement).await?;
    }
    Ok(connection)
}

pub async fn run_migrations(connection: &DatabaseConnection) -> Result<(), CoreError> {
    Migrator::up(connection, None).await?;
    Ok(())
}
