use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use leafypuff_api::domain::admin::{AccountDirectory, AccountSummary, AdminError};
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct InMemoryDirectory {
    rows: Arc<Mutex<Vec<AccountSummary>>>,
}

impl InMemoryDirectory {
    pub fn add(&self, summary: AccountSummary) {
        self.rows
            .lock()
            .expect("the directory lock holds")
            .push(summary);
    }

    pub fn snapshot(&self) -> Vec<AccountSummary> {
        self.rows.lock().expect("the directory lock holds").clone()
    }
}

#[async_trait]
impl AccountDirectory for InMemoryDirectory {
    async fn summaries(&self, limit: u64) -> Result<Vec<AccountSummary>, AdminError> {
        let rows = self.rows.lock().expect("the directory lock holds");
        let mut held = rows.clone();
        held.truncate(usize::try_from(limit).unwrap_or(usize::MAX));
        Ok(held)
    }

    async fn summary(&self, account_id: Uuid) -> Result<AccountSummary, AdminError> {
        let rows = self.rows.lock().expect("the directory lock holds");
        rows.iter()
            .find(|row| row.account_id == account_id)
            .cloned()
            .ok_or(AdminError::AccountNotFound)
    }

    async fn set_suspended(
        &self,
        account_id: Uuid,
        at: Option<DateTime<Utc>>,
    ) -> Result<(), AdminError> {
        let mut rows = self.rows.lock().expect("the directory lock holds");
        for row in rows.iter_mut().filter(|row| row.account_id == account_id) {
            row.suspended = at.is_some();
        }
        Ok(())
    }
}
