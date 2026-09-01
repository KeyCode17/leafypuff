use async_trait::async_trait;
use sea_orm::{ConnectionTrait, DatabaseBackend, DatabaseConnection, DbErr, Statement};

use crate::domain::admin::{AdminError, ServiceMetrics, ServiceOverview};

const OVERVIEW: &str = "
    SELECT
        (SELECT count(*) FROM accounts) AS account_count,
        (SELECT count(*) FROM accounts WHERE email_verified_at IS NOT NULL)
            AS verified_account_count,
        (SELECT count(*) FROM accounts WHERE suspended_at IS NOT NULL)
            AS suspended_account_count,
        (SELECT count(*) FROM entries WHERE deleted_at IS NULL) AS entry_count,
        (SELECT count(*) FROM entries WHERE deleted_at IS NOT NULL)
            AS tombstoned_entry_count,
        (SELECT count(*) FROM sync_checkpoints) AS device_count,
        (SELECT count(*) FROM sync_checkpoints WHERE updated_at >= $1)
            AS devices_synced_last_day,
        (SELECT count(*) FROM sync_field_conflicts) AS field_conflict_count,
        (SELECT count(*) FROM media_objects) AS media_object_count,
        (SELECT coalesce(sum(byte_len), 0) FROM media_objects) AS media_bytes
";

pub struct PgServiceMetrics {
    connection: DatabaseConnection,
}

impl PgServiceMetrics {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl ServiceMetrics for PgServiceMetrics {
    async fn overview(&self, since_ms: i64) -> Result<ServiceOverview, AdminError> {
        let row = self
            .connection
            .query_one(Statement::from_sql_and_values(
                DatabaseBackend::Postgres,
                OVERVIEW,
                [since_ms.into()],
            ))
            .await
            .map_err(storage)?
            .ok_or_else(|| AdminError::Storage("the overview returned no row".to_owned()))?;

        Ok(ServiceOverview {
            account_count: row.try_get("", "account_count").map_err(storage)?,
            verified_account_count: row.try_get("", "verified_account_count").map_err(storage)?,
            suspended_account_count: row
                .try_get("", "suspended_account_count")
                .map_err(storage)?,
            entry_count: row.try_get("", "entry_count").map_err(storage)?,
            tombstoned_entry_count: row.try_get("", "tombstoned_entry_count").map_err(storage)?,
            device_count: row.try_get("", "device_count").map_err(storage)?,
            devices_synced_last_day: row
                .try_get("", "devices_synced_last_day")
                .map_err(storage)?,
            field_conflict_count: row.try_get("", "field_conflict_count").map_err(storage)?,
            media_object_count: row.try_get("", "media_object_count").map_err(storage)?,
            media_bytes: row.try_get("", "media_bytes").map_err(storage)?,
        })
    }
}

fn storage(error: DbErr) -> AdminError {
    AdminError::Storage(error.to_string())
}
