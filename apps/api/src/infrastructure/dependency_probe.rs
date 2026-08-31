use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};
use tokio::net::TcpStream;

use crate::domain::{DomainError, ReadinessProbe, ReadinessReport};

#[derive(Clone)]
pub struct DependencyProbe {
    connection: Option<DatabaseConnection>,
    storage_endpoint: String,
}

impl DependencyProbe {
    pub const fn new(connection: DatabaseConnection, storage_endpoint: String) -> Self {
        Self {
            connection: Some(connection),
            storage_endpoint,
        }
    }

    pub const fn unreachable(storage_endpoint: String) -> Self {
        Self {
            connection: None,
            storage_endpoint,
        }
    }

    async fn database_reachable(&self) -> bool {
        let Some(connection) = &self.connection else {
            return false;
        };
        let statement =
            Statement::from_string(connection.get_database_backend(), "SELECT 1".to_owned());
        connection
            .execute(statement)
            .await
            .map_err(|error| tracing::warn!(%error, "database readiness check failed"))
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
