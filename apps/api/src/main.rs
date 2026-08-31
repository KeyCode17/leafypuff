mod application;
mod domain;
mod http;
mod infrastructure;

use std::net::SocketAddr;

use domain::ReadinessReport;
use http::{AppState, build_router};
use infrastructure::StaticReadinessProbe;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).init();

    let probe = StaticReadinessProbe::new(ReadinessReport {
        database: true,
        object_storage: true,
    });
    let app = build_router(AppState::new(probe));

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(8080);
    let address = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("failed to bind the configured port");
    tracing::info!(%address, "pawnotes-api listening");
    axum::serve(listener, app)
        .await
        .expect("server terminated unexpectedly");
}
