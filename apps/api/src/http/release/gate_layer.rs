use axum::extract::{Request, State};
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::domain::release::Platform;
use crate::http::error::ApiError;
use crate::http::state::AppState;

pub const ERR_UPDATE_REQUIRED: &str = "UPDATE_REQUIRED";

const PLATFORM_HEADER: &str = "x-app-platform";
const BUILD_HEADER: &str = "x-app-build";
const DETAIL_UPDATE_REQUIRED: &str = "This build can no longer sync. Update the app.";

pub async fn guard(State(state): State<AppState>, request: Request, next: Next) -> Response {
    let Some((platform, build)) = client(request.headers()) else {
        return next.run(request).await;
    };

    match state.release.gate(platform).await {
        Ok(gate) if gate.blocks(build) => ApiError::new(
            StatusCode::UPGRADE_REQUIRED,
            ERR_UPDATE_REQUIRED,
            gate.message.as_deref().unwrap_or(DETAIL_UPDATE_REQUIRED),
        )
        .into_response(),
        Ok(_) | Err(_) => next.run(request).await,
    }
}

fn client(headers: &HeaderMap) -> Option<(Platform, i32)> {
    let platform = headers
        .get(PLATFORM_HEADER)
        .and_then(|value| value.to_str().ok())
        .and_then(Platform::parse)?;
    let build = headers
        .get(BUILD_HEADER)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse().ok())?;
    Some((platform, build))
}
