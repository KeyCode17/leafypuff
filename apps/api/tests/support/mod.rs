pub mod adapters;
pub mod repositories;

use std::sync::Arc;

use chrono::{TimeZone, Utc};
use leafypuff_api::application::iam::{
    CompleteSignIn, ConsumeChallenge, IssueChallenge, MintSession, RefreshSession, RegisterAccount,
    StartSignIn, VerifyEmail,
};

use adapters::{CountingHasher, FixedClock, RecordingMailer, ScriptedOtp, SequentialIssuer};
use repositories::{InMemoryAccounts, InMemoryCredentials, InMemoryOtps};

pub struct World {
    pub accounts: InMemoryAccounts,
    pub credentials: InMemoryCredentials,
    pub otps: InMemoryOtps,
    pub clock: FixedClock,
    pub mailer: RecordingMailer,
    pub hasher: CountingHasher,
    pub generator: ScriptedOtp,
    pub issuer: SequentialIssuer,
}

impl World {
    pub fn new() -> Self {
        Self {
            accounts: InMemoryAccounts::default(),
            credentials: InMemoryCredentials::default(),
            otps: InMemoryOtps::default(),
            clock: FixedClock::new(
                Utc.with_ymd_and_hms(2026, 9, 1, 12, 0, 0)
                    .single()
                    .expect("the fixed instant is unambiguous"),
            ),
            mailer: RecordingMailer::default(),
            hasher: CountingHasher::default(),
            generator: ScriptedOtp::default(),
            issuer: SequentialIssuer::default(),
        }
    }

    fn issue_challenge(&self) -> IssueChallenge {
        IssueChallenge::new(
            Arc::new(self.otps.clone()),
            Arc::new(self.generator.clone()),
            Arc::new(self.mailer.clone()),
            Arc::new(self.clock.clone()),
        )
    }

    fn consume_challenge(&self) -> ConsumeChallenge {
        ConsumeChallenge::new(
            Arc::new(self.otps.clone()),
            Arc::new(self.generator.clone()),
            Arc::new(self.clock.clone()),
        )
    }

    fn mint_session(&self) -> MintSession {
        MintSession::new(
            Arc::new(self.issuer.clone()),
            Arc::new(self.credentials.clone()),
            Arc::new(self.clock.clone()),
        )
    }

    pub fn register(&self) -> RegisterAccount {
        RegisterAccount::new(
            Arc::new(self.accounts.clone()),
            Arc::new(self.hasher.clone()),
            self.issue_challenge(),
        )
    }

    pub fn verify_email(&self) -> VerifyEmail {
        VerifyEmail::new(
            Arc::new(self.accounts.clone()),
            self.consume_challenge(),
            Arc::new(self.clock.clone()),
        )
    }

    pub fn start_sign_in(&self) -> StartSignIn {
        StartSignIn::new(
            Arc::new(self.accounts.clone()),
            Arc::new(self.hasher.clone()),
            self.issue_challenge(),
        )
    }

    pub fn complete_sign_in(&self) -> CompleteSignIn {
        CompleteSignIn::new(
            Arc::new(self.accounts.clone()),
            self.consume_challenge(),
            self.mint_session(),
        )
    }

    pub fn refresh(&self) -> RefreshSession {
        RefreshSession::new(
            Arc::new(self.credentials.clone()),
            Arc::new(self.issuer.clone()),
            Arc::new(self.clock.clone()),
            self.mint_session(),
        )
    }
}
