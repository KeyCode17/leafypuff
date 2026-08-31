use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use uuid::Uuid;

use super::error::ApiError;
use super::state::AppState;

pub const ERR_UNAUTHENTICATED: &str = "UNAUTHENTICATED";
pub const ERR_DEVICE_UNIDENTIFIED: &str = "DEVICE_UNIDENTIFIED";

const BEARER_PREFIX: &str = "Bearer ";
const DEVICE_HEADER: &str = "x-device-id";
const DETAIL_UNAUTHENTICATED: &str = "Bearer token missing or invalid";
const DETAIL_DEVICE_UNIDENTIFIED: &str = "X-Device-Id must carry a uuid";

pub struct Authenticated {
    pub account_id: Uuid,
    pub device_id: Uuid,
}

impl FromRequestParts<AppState> for Authenticated {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let unauthenticated = || {
            ApiError::new(
                StatusCode::UNAUTHORIZED,
                ERR_UNAUTHENTICATED,
                DETAIL_UNAUTHENTICATED,
            )
        };
        let token = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix(BEARER_PREFIX))
            .ok_or_else(unauthenticated)?;
        let account_id = state
            .iam
            .verifier
            .account_id(token)
            .map_err(|_| unauthenticated())?;

        let device_id = parts
            .headers
            .get(DEVICE_HEADER)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| Uuid::parse_str(value).ok())
            .ok_or_else(|| {
                ApiError::new(
                    StatusCode::BAD_REQUEST,
                    ERR_DEVICE_UNIDENTIFIED,
                    DETAIL_DEVICE_UNIDENTIFIED,
                )
            })?;

        Ok(Self {
            account_id,
            device_id,
        })
    }
}
