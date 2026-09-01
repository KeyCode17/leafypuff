use std::sync::Arc;

use chrono::{TimeZone, Utc};
use leafypuff_api::application::admin::AdminServices;
use leafypuff_api::application::catalog::CatalogServices;
use leafypuff_api::application::iam::{
    CompleteSignIn, IamServices, RefreshSession, RegisterAccount, StartSignIn, VerifyEmail,
};
use leafypuff_api::application::media::MediaServices;
use leafypuff_api::application::privacy::PrivacyServices;
use leafypuff_api::application::rbac::RbacServices;
use leafypuff_api::application::release::ReleaseServices;
use leafypuff_api::application::sync::SyncServices;

use crate::adapters::{CountingHasher, FixedClock, RecordingMailer, ScriptedOtp, SequentialIssuer};
use crate::admin_repositories::{InMemoryDirectory, InMemoryMetrics};
use crate::catalog_repositories::InMemoryCatalog;
use crate::media_repositories::{InMemoryMedia, InMemoryObjects};
use crate::privacy_repositories::{InMemoryRequests, RecordingEraser};
use crate::rbac_repositories::{InMemoryAudit, InMemoryRoles};
use crate::release_repositories::{InMemoryCampaigns, InMemoryGates};
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
    pub directory: InMemoryDirectory,
    pub metrics: InMemoryMetrics,
    pub admin: AdminServices,
    pub bundles: InMemoryCatalog,
    pub catalog: CatalogServices,
    pub requests: InMemoryRequests,
    pub eraser: RecordingEraser,
    pub privacy: PrivacyServices,
    pub gates: InMemoryGates,
    pub campaigns: InMemoryCampaigns,
    pub release: ReleaseServices,
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

        let directory = InMemoryDirectory::default();
        let metrics = InMemoryMetrics::default();
        let admin = AdminServices {
            directory: Arc::new(directory.clone()),
            metrics: Arc::new(metrics.clone()),
            audit: Arc::new(audit.clone()),
            rbac: rbac.clone(),
        };

        let bundles = InMemoryCatalog::default();
        let catalog = CatalogServices {
            store: Arc::new(bundles.clone()),
            audit: Arc::new(audit.clone()),
            rbac: rbac.clone(),
        };

        let requests = InMemoryRequests::default();
        let eraser = RecordingEraser::default();
        let privacy = PrivacyServices {
            requests: Arc::new(requests.clone()),
            eraser: Arc::new(eraser.clone()),
            objects: Arc::new(objects.clone()),
            audit: Arc::new(audit.clone()),
            rbac: rbac.clone(),
        };

        let gates = InMemoryGates::default();
        let campaigns = InMemoryCampaigns::default();
        let release = ReleaseServices {
            gates: Arc::new(gates.clone()),
            campaigns: Arc::new(campaigns.clone()),
            audit: Arc::new(audit.clone()),
            rbac: rbac.clone(),
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
            directory,
            metrics,
            admin,
            bundles,
            catalog,
            requests,
            eraser,
            privacy,
            gates,
            campaigns,
            release,
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
