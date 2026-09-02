use std::sync::Arc;

use uuid::Uuid;

use crate::domain::iam::{AccountRepository, IamError, Profile};

pub struct SaveProfile {
    accounts: Arc<dyn AccountRepository>,
}

impl SaveProfile {
    pub const fn new(accounts: Arc<dyn AccountRepository>) -> Self {
        Self { accounts }
    }

    pub async fn execute(&self, account_id: Uuid, wanted: Profile) -> Result<Profile, IamError> {
        self.accounts.save_profile(account_id, wanted).await
    }
}
