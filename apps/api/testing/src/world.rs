use std::sync::Arc;

use chrono::{TimeZone, Utc};
use leafypuff_api::application::iam::{
    CompleteSignIn, IamServices, RefreshSession, RegisterAccount, StartSignIn, VerifyEmail,
};

use crate::adapters::{CountingHasher, FixedClock, RecordingMailer, ScriptedOtp, SequentialIssuer};
use crate::repositories::{InMemoryAccounts, InMemoryCredentials, InMemoryOtps};

pub struct World {
    pub accounts: InMemoryAccounts,
    pub credentials: InMemoryCredentials,
    pub otps: InMemoryOtps,
    pub clock: FixedClock,
    pub mailer: RecordingMailer,
    pub hasher: CountingHasher,
    pub generator: ScriptedOtp,
    pub issuer: SequentialIssuer,
    pub services: IamServices,
}

impl Default for World {
    fn default() -> Self {
        let accounts = InMemoryAccounts::default();
        let credentials = InMemoryCredentials::default();
        let otps = InMemoryOtps::default();
        let clock = FixedClock::new(
            Utc.with_ymd_and_hms(2026, 9, 1, 12, 0, 0)
                .single()
                .expect("the fixed instant is unambiguous"),
        );
        let mailer = RecordingMailer::default();
        let hasher = CountingHasher::default();
        let generator = ScriptedOtp::default();
        let issuer = SequentialIssuer::default();

        let services = IamServices {
            accounts: Arc::new(accounts.clone()),
            credentials: Arc::new(credentials.clone()),
            otps: Arc::new(otps.clone()),
            hasher: Arc::new(hasher.clone()),
            tokens: Arc::new(issuer.clone()),
            generator: Arc::new(generator.clone()),
            mail: Arc::new(mailer.clone()),
            clock: Arc::new(clock.clone()),
        };

        Self {
            accounts,
            credentials,
            otps,
            clock,
            mailer,
            hasher,
            generator,
            issuer,
            services,
        }
    }
}

impl World {
    pub fn register(&self) -> RegisterAccount {
        self.services.register()
    }

    pub fn verify_email(&self) -> VerifyEmail {
        self.services.verify_email()
    }

    pub fn start_sign_in(&self) -> StartSignIn {
        self.services.start_sign_in()
    }

    pub fn complete_sign_in(&self) -> CompleteSignIn {
        self.services.complete_sign_in()
    }

    pub fn refresh(&self) -> RefreshSession {
        self.services.refresh()
    }
}
