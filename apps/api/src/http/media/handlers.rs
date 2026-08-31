use axum::Json;
use axum::body::Bytes;
use axum::extract::{Path, Query, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::media::{MAX_OBJECT_BYTES, MediaError, Variant};
use crate::http::auth::Authenticated;
use crate::http::envelope::Envelope;
use crate::http::error::ApiError;
use crate::http::state::AppState;

pub const ERR_UNKNOWN_VARIANT: &str = "UNKNOWN_VARIANT";

const DETAIL_UNKNOWN_VARIANT: &str = "variant must be original or derivative";
const OCTET_STREAM: &str = "application/octet-stream";
const MESSAGE_STORED: &str = "Object stored";
const MESSAGE_FORGOTTEN: &str = "Object forgotten";

#[derive(Deserialize)]
pub struct EntryQuery {
    pub entry_id: Uuid,
}

pub async fn put_object(
    State(state): State<AppState>,
    caller: Authenticated,
    Path((photo_id, variant)): Path<(Uuid, String)>,
    Query(query): Query<EntryQuery>,
    ciphertext: Bytes,
) -> Response {
    let Some(variant) = Variant::parse(&variant) else {
        return unknown_variant();
    };
    if ciphertext.len() > MAX_OBJECT_BYTES {
        return ApiError::from(MediaError::TooLarge).into_response();
    }

    match state
        .media
        .store()
        .execute(
            caller.account_id,
            query.entry_id,
            photo_id,
            variant,
            ciphertext.to_vec(),
        )
        .await
    {
        Ok(()) => (StatusCode::CREATED, Json(Envelope::ok(MESSAGE_STORED, ()))).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn get_object(
    State(state): State<AppState>,
    caller: Authenticated,
    Path((photo_id, variant)): Path<(Uuid, String)>,
) -> Response {
    let Some(variant) = Variant::parse(&variant) else {
        return unknown_variant();
    };

    match state
        .media
        .read()
        .execute(caller.account_id, photo_id, variant)
        .await
    {
        Ok(ciphertext) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, OCTET_STREAM)],
            ciphertext,
        )
            .into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn delete_object(
    State(state): State<AppState>,
    caller: Authenticated,
    Path(photo_id): Path<Uuid>,
) -> Response {
    match state
        .media
        .forget()
        .execute(caller.account_id, photo_id)
        .await
    {
        Ok(()) => (StatusCode::OK, Json(Envelope::ok(MESSAGE_FORGOTTEN, ()))).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

fn unknown_variant() -> Response {
    ApiError::new(
        StatusCode::UNPROCESSABLE_ENTITY,
        ERR_UNKNOWN_VARIANT,
        DETAIL_UNKNOWN_VARIANT,
    )
    .into_response()
}
