use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::release::{Campaign, Platform, ReleaseGate};
use crate::http::auth::Authenticated;
use crate::http::envelope::Envelope;
use crate::http::error::ApiError;
use crate::http::state::AppState;
use crate::http::validated::Validated;

use super::dto::{CampaignResponse, GateResponse, SaveCampaignRequest, SetGateRequest};

pub const ERR_UNKNOWN_PLATFORM: &str = "UNKNOWN_PLATFORM";

const DETAIL_UNKNOWN_PLATFORM: &str = "platform must be android or web";
const MESSAGE_GATE: &str = "Release gate";
const MESSAGE_GATES: &str = "Release gates";
const MESSAGE_CAMPAIGNS: &str = "Campaigns";
const MESSAGE_GATE_SET: &str = "Release gate updated";
const MESSAGE_CAMPAIGN_SAVED: &str = "Campaign saved";

#[derive(Deserialize)]
pub struct DeviceQuery {
    pub platform: String,
    pub build: i32,
}

pub async fn read_gate(
    State(state): State<AppState>,
    _caller: Authenticated,
    Query(query): Query<DeviceQuery>,
) -> Response {
    let Some(platform) = Platform::parse(&query.platform) else {
        return unknown_platform();
    };
    match state.release.gate(platform).await {
        Ok(found) => ok(MESSAGE_GATE, gate(found, query.build)),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn read_campaigns(
    State(state): State<AppState>,
    _caller: Authenticated,
    Query(query): Query<DeviceQuery>,
) -> Response {
    let Some(platform) = Platform::parse(&query.platform) else {
        return unknown_platform();
    };
    match state.release.live_campaigns(platform).await {
        Ok(rows) => ok(
            MESSAGE_CAMPAIGNS,
            rows.into_iter().map(campaign).collect::<Vec<_>>(),
        ),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn list_gates(State(state): State<AppState>, caller: Authenticated) -> Response {
    match state.release.all_gates(caller.account_id).await {
        Ok(rows) => ok(
            MESSAGE_GATES,
            rows.into_iter()
                .map(|held| gate(held, 0))
                .collect::<Vec<_>>(),
        ),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn list_campaigns(State(state): State<AppState>, caller: Authenticated) -> Response {
    match state.release.all_campaigns(caller.account_id).await {
        Ok(rows) => ok(
            MESSAGE_CAMPAIGNS,
            rows.into_iter().map(campaign).collect::<Vec<_>>(),
        ),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn set_gate(
    State(state): State<AppState>,
    caller: Authenticated,
    Validated(body): Validated<SetGateRequest>,
) -> Response {
    let Some(platform) = Platform::parse(&body.platform) else {
        return unknown_platform();
    };
    match state
        .release
        .set_gate(
            caller.account_id,
            platform,
            body.minimum_build,
            body.force_update,
            body.message,
        )
        .await
    {
        Ok(()) => ok(MESSAGE_GATE_SET, ()),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn save_campaign(
    State(state): State<AppState>,
    caller: Authenticated,
    Validated(body): Validated<SaveCampaignRequest>,
) -> Response {
    let Some(platform) = Platform::parse(&body.platform) else {
        return unknown_platform();
    };
    let held = Campaign {
        id: body.id.unwrap_or_else(Uuid::new_v4),
        title: body.title,
        body: body.body,
        platform,
        starts_at_ms: body.starts_at_ms,
        ends_at_ms: body.ends_at_ms,
        published: body.published,
        created_at_ms: Utc::now().timestamp_millis(),
    };
    match state.release.save_campaign(caller.account_id, held).await {
        Ok(()) => ok(MESSAGE_CAMPAIGN_SAVED, ()),
        Err(error) => ApiError::from(error).into_response(),
    }
}

fn ok<T: serde::Serialize>(message: &str, data: T) -> Response {
    (StatusCode::OK, Json(Envelope::ok(message, data))).into_response()
}

fn unknown_platform() -> Response {
    ApiError::new(
        StatusCode::UNPROCESSABLE_ENTITY,
        ERR_UNKNOWN_PLATFORM,
        DETAIL_UNKNOWN_PLATFORM,
    )
    .into_response()
}

fn gate(found: ReleaseGate, build: i32) -> GateResponse {
    GateResponse {
        platform: found.platform.as_str().to_owned(),
        minimum_build: found.minimum_build,
        force_update: found.force_update,
        behind: found.is_behind(build),
        blocked: found.blocks(build),
        message: found.message,
    }
}

fn campaign(found: Campaign) -> CampaignResponse {
    CampaignResponse {
        id: found.id,
        title: found.title,
        body: found.body,
        platform: found.platform.as_str().to_owned(),
        starts_at_ms: found.starts_at_ms,
        ends_at_ms: found.ends_at_ms,
        published: found.published,
    }
}
