use async_trait::async_trait;
use sea_orm::{
    ConnectionTrait, DatabaseBackend, DatabaseConnection, DbErr, Statement, TransactionTrait,
};
use uuid::Uuid;

use crate::domain::privacy::{Eraser, PrivacyError};

const STEPS: [&str; 4] = [
    "UPDATE audit_subjects SET account_id = NULL, email = NULL WHERE account_id = $1",
    "DELETE FROM wrapped_content_keys WHERE account_id = $1",
    "DELETE FROM sync_field_conflicts WHERE account_id = $1",
    "DELETE FROM accounts WHERE id = $1",
];

pub struct PgEraser {
    connection: DatabaseConnection,
}

impl PgEraser {
    pub const fn new(connection: DatabaseConnection) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl Eraser for PgEraser {
    async fn erase(&self, account_id: Uuid) -> Result<(), PrivacyError> {
        let transaction = self.connection.begin().await.map_err(storage)?;
        for step in STEPS {
            transaction
                .execute(Statement::from_sql_and_values(
                    DatabaseBackend::Postgres,
                    step,
                    [account_id.into()],
                ))
                .await
                .map_err(storage)?;
        }
        transaction.commit().await.map_err(storage)
    }
}

fn storage(error: DbErr) -> PrivacyError {
    PrivacyError::Storage(error.to_string())
}
