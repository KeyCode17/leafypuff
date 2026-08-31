use std::time::Duration;

use sea_orm::{ConnectOptions, ConnectionTrait, Database, Statement};
use tokio::net::TcpStream;

use crate::domain::{DomainError, ReadinessProbe, ReadinessReport};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(3);

#[derive(Clone)]
pub struct DependencyProbe {
    database_url: String,
    storage_endpoint: String,
}

impl DependencyProbe {
    pub const fn new(database_url: String, storage_endpoint: String) -> Self {
        Self {
            database_url,
            storage_endpoint,
        }
    }

    async fn database_reachable(&self) -> bool {
        let mut options = ConnectOptions::new(self.database_url.clone());
        options
            .max_connections(1)
            .connect_timeout(CONNECT_TIMEOUT)
            .sqlx_logging(false);

        let Ok(connection) = Database::connect(options)
            .await
            .map_err(|error| tracing::warn!(%error, "database connection failed"))
        else {
            return false;
        };

        let statement =
            Statement::from_string(connection.get_database_backend(), "SELECT 1".to_owned());
        connection
            .execute(statement)
            .await
            .map_err(|error| tracing::warn!(%error, "database readiness query failed"))
            .is_ok()
    }

    async fn storage_reachable(&self) -> bool {
        TcpStream::connect(&self.storage_endpoint)
            .await
            .map_err(|error| tracing::warn!(%error, "object storage readiness check failed"))
            .is_ok()
    }
}

impl ReadinessProbe for DependencyProbe {
    async fn check(&self) -> Result<ReadinessReport, DomainError> {
        Ok(ReadinessReport {
            database: self.database_reachable().await,
            object_storage: self.storage_reachable().await,
        })
    }
}
