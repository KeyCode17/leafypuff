use std::net::SocketAddr;

use leafypuff_api::http::{AppState, build_router};
use leafypuff_api::infrastructure::{Config, DependencyProbe};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).init();

    let config =
        Config::from_env(&|key| std::env::var(key).ok()).expect("configuration is incomplete");

    let probe = DependencyProbe::new(config.database_url.clone(), config.s3_endpoint.clone());
    let app = build_router(AppState::new(probe));
    let address = SocketAddr::from(([0, 0, 0, 0], config.port));

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("failed to bind the configured port");
    tracing::info!(%address, "leafypuff-api listening");
    axum::serve(listener, app)
        .await
        .expect("server terminated unexpectedly");
}
