use std::sync::Arc;

use uuid::Uuid;

use crate::domain::iam::{AccountRepository, IamError, Profile};

pub struct ReadProfile {
    accounts: Arc<dyn AccountRepository>,
}

impl ReadProfile {
    pub const fn new(accounts: Arc<dyn AccountRepository>) -> Self {
        Self { accounts }
    }

    pub async fn execute(&self, account_id: Uuid) -> Result<Profile, IamError> {
        self.accounts.profile(account_id).await
    }
}
