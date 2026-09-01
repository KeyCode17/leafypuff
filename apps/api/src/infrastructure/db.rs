use std::time::Duration;

use api_migration::MigratorTrait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_CONNECTIONS: u32 = 16;

pub async fn connect_and_migrate(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let mut options = ConnectOptions::new(database_url.to_owned());
    options
        .max_connections(MAX_CONNECTIONS)
        .connect_timeout(CONNECT_TIMEOUT)
        .sqlx_logging(false);

    let connection = Database::connect(options).await?;
    api_migration::Migrator::up(&connection, None).await?;
    Ok(connection)
}
