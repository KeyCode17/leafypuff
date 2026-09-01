use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use uuid::Uuid;

use crate::domain::admin::{AccountSummary, ServiceOverview};
use crate::http::auth::Authenticated;
use crate::http::envelope::Envelope;
use crate::http::error::ApiError;
use crate::http::state::AppState;

use super::dto::{AccountSummaryResponse, OverviewResponse};

const MESSAGE_OVERVIEW: &str = "Service overview";
const MESSAGE_ACCOUNTS: &str = "Accounts";
const MESSAGE_ACCOUNT: &str = "Account";
const MESSAGE_SUSPENDED: &str = "Account suspended";
const MESSAGE_RESTORED: &str = "Account restored";

pub async fn read_overview(State(state): State<AppState>, caller: Authenticated) -> Response {
    match state.admin.overview(caller.account_id).await {
        Ok(found) => ok(MESSAGE_OVERVIEW, overview(found)),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn list_accounts(State(state): State<AppState>, caller: Authenticated) -> Response {
    match state.admin.list(caller.account_id).await {
        Ok(rows) => ok(
            MESSAGE_ACCOUNTS,
            rows.into_iter().map(summary).collect::<Vec<_>>(),
        ),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn read_account(
    State(state): State<AppState>,
    caller: Authenticated,
    Path(account_id): Path<Uuid>,
) -> Response {
    match state.admin.read(caller.account_id, account_id).await {
        Ok(found) => ok(MESSAGE_ACCOUNT, summary(found)),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn suspend_account(
    State(state): State<AppState>,
    caller: Authenticated,
    Path(account_id): Path<Uuid>,
) -> Response {
    match state.admin.suspend(caller.account_id, account_id).await {
        Ok(()) => ok(MESSAGE_SUSPENDED, ()),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn restore_account(
    State(state): State<AppState>,
    caller: Authenticated,
    Path(account_id): Path<Uuid>,
) -> Response {
    match state.admin.restore(caller.account_id, account_id).await {
        Ok(()) => ok(MESSAGE_RESTORED, ()),
        Err(error) => ApiError::from(error).into_response(),
    }
}

fn ok<T: serde::Serialize>(message: &str, data: T) -> Response {
    (StatusCode::OK, Json(Envelope::ok(message, data))).into_response()
}

fn overview(found: ServiceOverview) -> OverviewResponse {
    OverviewResponse {
        account_count: found.account_count,
        verified_account_count: found.verified_account_count,
        suspended_account_count: found.suspended_account_count,
        entry_count: found.entry_count,
        tombstoned_entry_count: found.tombstoned_entry_count,
        device_count: found.device_count,
        devices_synced_last_day: found.devices_synced_last_day,
        field_conflict_count: found.field_conflict_count,
        media_object_count: found.media_object_count,
        media_bytes: found.media_bytes,
    }
}

fn summary(found: AccountSummary) -> AccountSummaryResponse {
    AccountSummaryResponse {
        account_id: found.account_id,
        email: found.email,
        verified: found.verified,
        suspended: found.suspended,
        entry_count: found.entry_count,
        first_entry_date: found.first_entry_date,
        last_entry_date: found.last_entry_date,
        media_object_count: found.media_object_count,
        media_bytes: found.media_bytes,
    }
}
