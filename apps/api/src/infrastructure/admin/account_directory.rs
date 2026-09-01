use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{
    ConnectionTrait, DatabaseBackend, DatabaseConnection, DbErr, QueryResult, Statement,
};
use uuid::Uuid;

use crate::domain::admin::{AccountDirectory, AccountSummary, AdminError};

const SUMMARY_COLUMNS: &str = "
    a.id AS account_id,
    a.email AS email,
    (a.email_verified_at IS NOT NULL) AS verified,
    (a.suspended_at IS NOT NULL) AS suspended,
    coalesce(e.entry_count, 0) AS entry_count,
    e.first_entry_date AS first_entry_date,
    e.last_entry_date AS last_entry_date,
    coalesce(m.media_object_count, 0) AS media_object_count,
    coalesce(m.media_bytes, 0) AS media_bytes
";

const SUMMARY_JOINS: &str = "
    FROM accounts a
    LEFT JOIN (
        SELECT account_id,
               count(*) AS entry_count,
               min(date) AS first_entry_date,
               max(date) AS last_entry_date
        FROM entries WHERE deleted_at IS NULL GROUP BY account_id
    ) e ON e.account_id = a.id
    LEFT JOIN (
        SELECT account_id,
               count(*) AS media_object_count,
               sum(byte_len) AS media_bytes
        FROM media_objects GROUP BY account_id
    ) m ON m.account_id = a.id
";

pub struct PgAccountDirectory {
    connection: DatabaseConnection,
}

impl PgAccountDirectory {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl AccountDirectory for PgAccountDirectory {
    async fn summaries(&self, limit: u64) -> Result<Vec<AccountSummary>, AdminError> {
        let rows = self
            .connection
            .query_all(Statement::from_sql_and_values(
                DatabaseBackend::Postgres,
                format!("SELECT {SUMMARY_COLUMNS} {SUMMARY_JOINS} ORDER BY a.email LIMIT $1"),
                [i64::try_from(limit).unwrap_or(i64::MAX).into()],
            ))
            .await
            .map_err(storage)?;
        rows.iter().map(summary).collect()
    }

    async fn summary(&self, account_id: Uuid) -> Result<AccountSummary, AdminError> {
        let row = self
            .connection
            .query_one(Statement::from_sql_and_values(
                DatabaseBackend::Postgres,
                format!("SELECT {SUMMARY_COLUMNS} {SUMMARY_JOINS} WHERE a.id = $1"),
                [account_id.into()],
            ))
            .await
            .map_err(storage)?
            .ok_or(AdminError::AccountNotFound)?;
        summary(&row)
    }

    async fn set_suspended(
        &self,
        account_id: Uuid,
        at: Option<DateTime<Utc>>,
    ) -> Result<(), AdminError> {
        self.connection
            .execute(Statement::from_sql_and_values(
                DatabaseBackend::Postgres,
                "UPDATE accounts SET suspended_at = $1, updated_at = now() WHERE id = $2",
                [at.map(|held| held.fixed_offset()).into(), account_id.into()],
            ))
            .await
            .map_err(storage)?;
        Ok(())
    }
}

fn summary(row: &QueryResult) -> Result<AccountSummary, AdminError> {
    Ok(AccountSummary {
        account_id: row.try_get("", "account_id").map_err(storage)?,
        email: row.try_get("", "email").map_err(storage)?,
        verified: row.try_get("", "verified").map_err(storage)?,
        suspended: row.try_get("", "suspended").map_err(storage)?,
        entry_count: row.try_get("", "entry_count").map_err(storage)?,
        first_entry_date: row.try_get("", "first_entry_date").map_err(storage)?,
        last_entry_date: row.try_get("", "last_entry_date").map_err(storage)?,
        media_object_count: row.try_get("", "media_object_count").map_err(storage)?,
        media_bytes: row.try_get("", "media_bytes").map_err(storage)?,
    })
}

fn storage(error: DbErr) -> AdminError {
    AdminError::Storage(error.to_string())
}
