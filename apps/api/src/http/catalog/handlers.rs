use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use uuid::Uuid;

use crate::domain::catalog::CatalogBundle;
use crate::http::auth::Authenticated;
use crate::http::envelope::Envelope;
use crate::http::error::ApiError;
use crate::http::state::AppState;
use crate::http::validated::Validated;

use super::dto::{BundleResponse, DraftBundleRequest};

const MESSAGE_PUBLISHED: &str = "The published catalog";
const MESSAGE_BUNDLES: &str = "Catalog bundles";
const MESSAGE_DRAFTED: &str = "Bundle drafted";
const MESSAGE_LIVE: &str = "Bundle published";

pub async fn read_published(State(state): State<AppState>, _caller: Authenticated) -> Response {
    match state.catalog.published().await {
        Ok(found) => ok(MESSAGE_PUBLISHED, bundle(found)),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn list_bundles(State(state): State<AppState>, caller: Authenticated) -> Response {
    match state.catalog.list(caller.account_id).await {
        Ok(rows) => ok(
            MESSAGE_BUNDLES,
            rows.into_iter().map(bundle).collect::<Vec<_>>(),
        ),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn draft_bundle(
    State(state): State<AppState>,
    caller: Authenticated,
    Validated(body): Validated<DraftBundleRequest>,
) -> Response {
    match state
        .catalog
        .draft(caller.account_id, body.payload.to_string())
        .await
    {
        Ok(created) => (
            StatusCode::CREATED,
            Json(Envelope::ok(MESSAGE_DRAFTED, bundle(created))),
        )
            .into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn publish_bundle(
    State(state): State<AppState>,
    caller: Authenticated,
    Path(bundle_id): Path<Uuid>,
) -> Response {
    match state.catalog.publish(caller.account_id, bundle_id).await {
        Ok(()) => ok(MESSAGE_LIVE, ()),
        Err(error) => ApiError::from(error).into_response(),
    }
}

fn ok<T: serde::Serialize>(message: &str, data: T) -> Response {
    (StatusCode::OK, Json(Envelope::ok(message, data))).into_response()
}

fn bundle(found: CatalogBundle) -> BundleResponse {
    BundleResponse {
        id: found.id,
        version: found.version,
        payload: serde_json::from_str(&found.payload).unwrap_or(serde_json::Value::Null),
        published: found.is_published(),
        published_at_ms: found.published_at_ms,
        created_at_ms: found.created_at_ms,
    }
}
