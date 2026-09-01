use std::net::SocketAddr;
use std::sync::Arc;

use leafypuff_api::application::admin::AdminServices;
use leafypuff_api::application::catalog::CatalogServices;
use leafypuff_api::application::iam::IamServices;
use leafypuff_api::application::media::MediaServices;
use leafypuff_api::application::privacy::PrivacyServices;
use leafypuff_api::application::rbac::RbacServices;
use leafypuff_api::application::sync::SyncServices;
use leafypuff_api::domain::iam::{TokenIssuer, TokenVerifier};
use leafypuff_api::http::{AppState, build_router};
use leafypuff_api::infrastructure::admin::{PgAccountDirectory, PgServiceMetrics};
use leafypuff_api::infrastructure::catalog::PgCatalogStore;
use leafypuff_api::infrastructure::iam::{
    Argon2Hasher, Blake3Otp, JwtTokenIssuer, PgAccountRepository, PgOtpRepository,
    PgRefreshTokenRepository, ResendEmailSender, SystemClock,
};
use leafypuff_api::infrastructure::media::{PgMediaRepository, S3ObjectStore, build_s3_client};
use leafypuff_api::infrastructure::privacy::{PgDataRequestStore, PgEraser};
use leafypuff_api::infrastructure::rbac::{PgAuditLog, PgPermissionReader, PgRoleRepository};
use leafypuff_api::infrastructure::sync::{
    PgCheckpointStore, PgConflictSink, PgEntryStore, PgIdempotencyStore, PgWrappedKeyStore,
};
use leafypuff_api::infrastructure::{Config, DependencyProbe, connect_and_migrate};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).init();

    let config =
        Config::from_env(&|key| std::env::var(key).ok()).expect("configuration is incomplete");

    let connection = connect_and_migrate(&config.database_url)
        .await
        .expect("the database must accept a connection and apply its migrations");

    let issuer = Arc::new(JwtTokenIssuer::new(&config.jwt_signing_secret));
    let iam = IamServices {
        accounts: Arc::new(PgAccountRepository::new(connection.clone())),
        credentials: Arc::new(PgRefreshTokenRepository::new(connection.clone())),
        otps: Arc::new(PgOtpRepository::new(connection.clone())),
        hasher: Arc::new(Argon2Hasher::new().expect("the password hasher must build")),
        tokens: Arc::clone(&issuer) as Arc<dyn TokenIssuer>,
        verifier: issuer as Arc<dyn TokenVerifier>,
        generator: Arc::new(Blake3Otp::new(config.otp_pepper)),
        mail: Arc::new(
            ResendEmailSender::new(config.resend_api_key.clone(), config.mail_from.clone())
                .expect("the mail sender must build"),
        ),
        clock: Arc::new(SystemClock),
    };

    let sync = SyncServices {
        entries: Arc::new(PgEntryStore::new(connection.clone())),
        checkpoints: Arc::new(PgCheckpointStore::new(connection.clone())),
        idempotency: Arc::new(PgIdempotencyStore::new(connection.clone())),
        conflicts: Arc::new(PgConflictSink::new(connection.clone())),
        keys: Arc::new(PgWrappedKeyStore::new(connection.clone())),
    };

    let media = MediaServices {
        objects: Arc::new(S3ObjectStore::new(
            build_s3_client(&config),
            config.s3_bucket.clone(),
        )),
        media: Arc::new(PgMediaRepository::new(connection.clone())),
    };

    let rbac = RbacServices {
        roles: Arc::new(PgRoleRepository::new(connection.clone())),
        permissions: Arc::new(PgPermissionReader::new(connection.clone())),
        audit: Arc::new(PgAuditLog::new(connection.clone())),
    };

    let admin = AdminServices {
        directory: Arc::new(PgAccountDirectory::new(connection.clone())),
        metrics: Arc::new(PgServiceMetrics::new(connection.clone())),
        audit: Arc::new(PgAuditLog::new(connection.clone())),
        rbac: rbac.clone(),
    };

    let catalog = CatalogServices {
        store: Arc::new(PgCatalogStore::new(connection.clone())),
        audit: Arc::new(PgAuditLog::new(connection.clone())),
        rbac: rbac.clone(),
    };

    let privacy = PrivacyServices {
        requests: Arc::new(PgDataRequestStore::new(connection.clone())),
        eraser: Arc::new(PgEraser::new(connection.clone())),
        objects: Arc::clone(&media.objects),
        audit: Arc::new(PgAuditLog::new(connection.clone())),
        rbac: rbac.clone(),
    };

    let probe = DependencyProbe::new(config.database_url.clone(), config.s3_endpoint.clone());
    let app = build_router(AppState::new(
        probe, iam, sync, media, rbac, admin, catalog, privacy,
    ));
    let address = SocketAddr::from(([0, 0, 0, 0], config.port));

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("failed to bind the configured port");
    tracing::info!(%address, "leafypuff-api listening");
    axum::serve(listener, app)
        .await
        .expect("server terminated unexpectedly");
}
