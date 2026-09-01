use std::sync::Arc;

use chrono::{TimeZone, Utc};
use leafypuff_api::application::iam::{
    CompleteSignIn, IamServices, RefreshSession, RegisterAccount, StartSignIn, VerifyEmail,
};
use leafypuff_api::application::media::MediaServices;
use leafypuff_api::application::rbac::RbacServices;
use leafypuff_api::application::sync::SyncServices;

use crate::adapters::{CountingHasher, FixedClock, RecordingMailer, ScriptedOtp, SequentialIssuer};
use crate::media_repositories::{InMemoryMedia, InMemoryObjects};
use crate::rbac_repositories::{InMemoryAudit, InMemoryRoles};
use crate::repositories::{InMemoryAccounts, InMemoryCredentials, InMemoryOtps};
use crate::sync_repositories::{
    InMemoryCheckpoints, InMemoryConflicts, InMemoryEntries, InMemoryIdempotency,
    InMemoryWrappedKeys,
};

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
    pub entries: InMemoryEntries,
    pub conflicts: InMemoryConflicts,
    pub sync: SyncServices,
    pub objects: InMemoryObjects,
    pub media: MediaServices,
    pub roles: InMemoryRoles,
    pub audit: InMemoryAudit,
    pub rbac: RbacServices,
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
            verifier: Arc::new(issuer.clone()),
            generator: Arc::new(generator.clone()),
            mail: Arc::new(mailer.clone()),
            clock: Arc::new(clock.clone()),
        };

        let entries = InMemoryEntries::default();
        let conflicts = InMemoryConflicts::default();
        let sync = SyncServices {
            entries: Arc::new(entries.clone()),
            checkpoints: Arc::new(InMemoryCheckpoints::default()),
            idempotency: Arc::new(InMemoryIdempotency::default()),
            conflicts: Arc::new(conflicts.clone()),
            keys: Arc::new(InMemoryWrappedKeys::default()),
        };

        let objects = InMemoryObjects::default();
        let media = MediaServices {
            objects: Arc::new(objects.clone()),
            media: Arc::new(InMemoryMedia::default()),
        };

        let roles = InMemoryRoles::default();
        let audit = InMemoryAudit::default();
        let rbac = RbacServices {
            roles: Arc::new(roles.clone()),
            permissions: Arc::new(roles.clone()),
            audit: Arc::new(audit.clone()),
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
            entries,
            conflicts,
            sync,
            objects,
            media,
            roles,
            audit,
            rbac,
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
