use std::net::SocketAddr;
use std::sync::Arc;

use leafypuff_api::application::iam::IamServices;
use leafypuff_api::http::{AppState, build_router};
use leafypuff_api::infrastructure::iam::{
    Argon2Hasher, Blake3Otp, JwtTokenIssuer, PgAccountRepository, PgOtpRepository,
    PgRefreshTokenRepository, ResendEmailSender, SystemClock,
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

    let iam = IamServices {
        accounts: Arc::new(PgAccountRepository::new(connection.clone())),
        credentials: Arc::new(PgRefreshTokenRepository::new(connection.clone())),
        otps: Arc::new(PgOtpRepository::new(connection)),
        hasher: Arc::new(Argon2Hasher::new().expect("the password hasher must build")),
        tokens: Arc::new(JwtTokenIssuer::new(&config.jwt_signing_secret)),
        generator: Arc::new(Blake3Otp::new(config.otp_pepper)),
        mail: Arc::new(
            ResendEmailSender::new(config.resend_api_key.clone(), config.mail_from.clone())
                .expect("the mail sender must build"),
        ),
        clock: Arc::new(SystemClock),
    };

    let probe = DependencyProbe::new(config.database_url.clone(), config.s3_endpoint.clone());
    let app = build_router(AppState::new(probe, iam));
    let address = SocketAddr::from(([0, 0, 0, 0], config.port));

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("failed to bind the configured port");
    tracing::info!(%address, "leafypuff-api listening");
    axum::serve(listener, app)
        .await
        .expect("server terminated unexpectedly");
}
