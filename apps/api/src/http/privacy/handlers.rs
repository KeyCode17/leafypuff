use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use uuid::Uuid;

use crate::domain::privacy::{DataRequest, RequestKind};
use crate::http::auth::Authenticated;
use crate::http::envelope::Envelope;
use crate::http::error::ApiError;
use crate::http::state::AppState;
use crate::http::validated::Validated;

use super::dto::{DataRequestResponse, RaiseRequest};

pub const ERR_UNKNOWN_KIND: &str = "UNKNOWN_REQUEST_KIND";

const DETAIL_UNKNOWN_KIND: &str = "kind must be export or erasure";
const MESSAGE_RAISED: &str = "Request recorded";
const MESSAGE_OPEN: &str = "Open data requests";
const MESSAGE_FULFILLED: &str = "Request fulfilled";

pub async fn raise_request(
    State(state): State<AppState>,
    caller: Authenticated,
    Validated(body): Validated<RaiseRequest>,
) -> Response {
    let Some(kind) = RequestKind::parse(&body.kind) else {
        return ApiError::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            ERR_UNKNOWN_KIND,
            DETAIL_UNKNOWN_KIND,
        )
        .into_response();
    };

    match state
        .privacy
        .raise(caller.account_id, body.email, kind)
        .await
    {
        Ok(recorded) => (
            StatusCode::CREATED,
            Json(Envelope::ok(MESSAGE_RAISED, response(recorded))),
        )
            .into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn list_open(State(state): State<AppState>, caller: Authenticated) -> Response {
    match state.privacy.open(caller.account_id).await {
        Ok(rows) => (
            StatusCode::OK,
            Json(Envelope::ok(
                MESSAGE_OPEN,
                rows.into_iter().map(response).collect::<Vec<_>>(),
            )),
        )
            .into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn fulfil_request(
    State(state): State<AppState>,
    caller: Authenticated,
    Path(request_id): Path<Uuid>,
) -> Response {
    match state.privacy.fulfil(caller.account_id, request_id).await {
        Ok(()) => (StatusCode::OK, Json(Envelope::ok(MESSAGE_FULFILLED, ()))).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

fn response(held: DataRequest) -> DataRequestResponse {
    DataRequestResponse {
        id: held.id,
        account_id: held.account_id,
        email: held.email,
        kind: held.kind.as_str().to_owned(),
        status: held.status.as_str().to_owned(),
        requested_at_ms: held.requested_at_ms,
        fulfilled_at_ms: held.fulfilled_at_ms,
    }
}
