use axum::Json;
use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use uuid::Uuid;

use crate::domain::media::{MAX_OBJECT_BYTES, MediaError, Variant};
use crate::http::auth::Authenticated;
use crate::http::envelope::Envelope;
use crate::http::error::ApiError;
use crate::http::state::AppState;

use super::dto::{ProfileRequest, ProfileResponse};

const MESSAGE_READ: &str = "Profile read";
const MESSAGE_SAVED: &str = "Profile saved";
const MESSAGE_AVATAR_STORED: &str = "Avatar stored";
const OCTET_STREAM: &str = "application/octet-stream";
const ERR_UNKNOWN_VARIANT: &str = "UNKNOWN_VARIANT";
const DETAIL_UNKNOWN_VARIANT: &str = "variant must be original or derivative";
const ERR_NO_AVATAR: &str = "NO_AVATAR";
const DETAIL_NO_AVATAR: &str = "the profile names no avatar photo";

pub async fn read_profile(State(state): State<AppState>, caller: Authenticated) -> Response {
    match state.iam.read_profile().execute(caller.account_id).await {
        Ok(profile) => settled(MESSAGE_READ, profile.into()),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn save_profile(
    State(state): State<AppState>,
    caller: Authenticated,
    Json(body): Json<ProfileRequest>,
) -> Response {
    match state
        .iam
        .save_profile()
        .execute(caller.account_id, body.into())
        .await
    {
        Ok(profile) => settled(MESSAGE_SAVED, profile.into()),
        Err(error) => ApiError::from(error).into_response(),
    }
}

fn settled(message: &str, profile: ProfileResponse) -> Response {
    (StatusCode::OK, Json(Envelope::ok(message, profile))).into_response()
}

pub async fn put_avatar(
    State(state): State<AppState>,
    caller: Authenticated,
    Path(variant): Path<String>,
    ciphertext: Bytes,
) -> Response {
    let Some(variant) = Variant::parse(&variant) else {
        return unknown_variant();
    };
    if ciphertext.len() > MAX_OBJECT_BYTES {
        return ApiError::from(MediaError::TooLarge).into_response();
    }
    let photo_id = match avatar_of(&state, caller.account_id).await {
        Ok(found) => found,
        Err(refusal) => return refusal.into_response(),
    };

    match state
        .media
        .store_avatar()
        .execute(caller.account_id, photo_id, variant, ciphertext.to_vec())
        .await
    {
        Ok(()) => (
            StatusCode::CREATED,
            Json(Envelope::ok(MESSAGE_AVATAR_STORED, ())),
        )
            .into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn get_avatar(
    State(state): State<AppState>,
    caller: Authenticated,
    Path(variant): Path<String>,
) -> Response {
    let Some(variant) = Variant::parse(&variant) else {
        return unknown_variant();
    };
    let photo_id = match avatar_of(&state, caller.account_id).await {
        Ok(found) => found,
        Err(refusal) => return refusal.into_response(),
    };

    match state
        .media
        .read_avatar()
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

async fn avatar_of(state: &AppState, account_id: Uuid) -> Result<Uuid, ApiError> {
    let profile = state
        .iam
        .read_profile()
        .execute(account_id)
        .await
        .map_err(ApiError::from)?;
    profile.avatar_photo_id.ok_or_else(no_avatar)
}

fn no_avatar() -> ApiError {
    ApiError::new(StatusCode::NOT_FOUND, ERR_NO_AVATAR, DETAIL_NO_AVATAR)
}

fn unknown_variant() -> Response {
    ApiError::new(
        StatusCode::UNPROCESSABLE_ENTITY,
        ERR_UNKNOWN_VARIANT,
        DETAIL_UNKNOWN_VARIANT,
    )
    .into_response()
}
