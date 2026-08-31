use axum::Json;
use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

use crate::domain::sync::{SyncCursor, SyncError};
use crate::http::auth::Authenticated;
use crate::http::envelope::Envelope;
use crate::http::error::ApiError;
use crate::http::state::AppState;
use crate::http::validated::Validated;

use super::dto::{PullResponse, PushRequest, PushResponse, WrappedKeyRequest};
use super::mapping;

pub const ERR_IDEMPOTENCY_KEY_MISSING: &str = "IDEMPOTENCY_KEY_MISSING";

const IDEMPOTENCY_HEADER: &str = "idempotency-key";
const DETAIL_IDEMPOTENCY: &str = "Idempotency-Key must carry a value";
const MESSAGE_PULLED: &str = "Changes since the cursor";
const MESSAGE_PUSHED: &str = "Changes applied";
const MESSAGE_KEYS: &str = "Wrapped content keys";
const MESSAGE_KEY_STORED: &str = "Wrapped content key stored";

#[derive(Deserialize)]
pub struct CursorQuery {
    pub cursor: Option<i64>,
}

pub async fn pull(
    State(state): State<AppState>,
    caller: Authenticated,
    Query(query): Query<CursorQuery>,
) -> Response {
    let outcome = state
        .sync
        .pull()
        .execute(
            caller.account_id,
            caller.device_id,
            query.cursor.map(SyncCursor),
        )
        .await;

    match outcome {
        Ok(changes) => (
            StatusCode::OK,
            Json(Envelope::ok(
                MESSAGE_PULLED,
                PullResponse {
                    cursor: changes.cursor.0,
                    records: changes
                        .records
                        .into_iter()
                        .map(mapping::record_response)
                        .collect(),
                },
            )),
        )
            .into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn push(
    State(state): State<AppState>,
    caller: Authenticated,
    headers: HeaderMap,
    Validated(body): Validated<PushRequest>,
) -> Response {
    let Some(key) = idempotency_key(&headers) else {
        return ApiError::new(
            StatusCode::BAD_REQUEST,
            ERR_IDEMPOTENCY_KEY_MISSING,
            DETAIL_IDEMPOTENCY,
        )
        .into_response();
    };

    let records: Result<Vec<_>, SyncError> = body
        .records
        .into_iter()
        .map(|record| mapping::record(caller.account_id, record))
        .collect();
    let records = match records {
        Ok(records) => records,
        Err(error) => return ApiError::from(error).into_response(),
    };

    match state
        .sync
        .push()
        .execute(caller.account_id, caller.device_id, &key, records)
        .await
    {
        Ok(receipt) => (
            StatusCode::OK,
            Json(Envelope::ok(
                MESSAGE_PUSHED,
                PushResponse {
                    cursor: receipt.cursor,
                    applied: receipt.applied,
                    replayed: receipt.replayed,
                },
            )),
        )
            .into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn read_keys(State(state): State<AppState>, caller: Authenticated) -> Response {
    match state.sync.keys.read_all(caller.account_id).await {
        Ok(rows) => {
            let payload: Vec<_> = rows
                .into_iter()
                .map(mapping::wrapped_key_response)
                .collect();
            (StatusCode::OK, Json(Envelope::ok(MESSAGE_KEYS, payload))).into_response()
        }
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn put_key(
    State(state): State<AppState>,
    caller: Authenticated,
    Validated(body): Validated<WrappedKeyRequest>,
) -> Response {
    let row = match mapping::wrapped_key(body) {
        Ok(row) => row,
        Err(error) => return ApiError::from(error).into_response(),
    };
    match state.sync.keys.put(caller.account_id, row).await {
        Ok(()) => (StatusCode::OK, Json(Envelope::ok(MESSAGE_KEY_STORED, ()))).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

fn idempotency_key(headers: &HeaderMap) -> Option<String> {
    headers
        .get(IDEMPOTENCY_HEADER)
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
}
