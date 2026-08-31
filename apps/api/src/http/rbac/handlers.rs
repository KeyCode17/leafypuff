use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::domain::rbac::{AuditEvent, Permission, Role};
use crate::http::auth::Authenticated;
use crate::http::envelope::Envelope;
use crate::http::error::ApiError;
use crate::http::state::AppState;
use crate::http::validated::Validated;

use super::dto::{AssignRoleRequest, AuditEventResponse, GrantedResponse, RoleResponse};

const MESSAGE_ROLES: &str = "Roles";
const MESSAGE_GRANTED: &str = "Permissions granted to the caller";
const MESSAGE_ASSIGNED: &str = "Role assigned";
const MESSAGE_REVOKED: &str = "Role revoked";
const MESSAGE_AUDIT: &str = "Recent audit events";

pub async fn list_roles(State(state): State<AppState>, caller: Authenticated) -> Response {
    if let Err(error) = state
        .rbac
        .require(caller.account_id, Permission::RoleRead)
        .await
    {
        return ApiError::from(error).into_response();
    }
    match state.rbac.all_roles().await {
        Ok(roles) => ok(
            MESSAGE_ROLES,
            roles.into_iter().map(role).collect::<Vec<_>>(),
        ),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn granted(State(state): State<AppState>, caller: Authenticated) -> Response {
    match state.rbac.granted(caller.account_id).await {
        Ok(permissions) => ok(
            MESSAGE_GRANTED,
            GrantedResponse {
                permissions: permissions
                    .into_iter()
                    .map(|held| held.as_str().to_owned())
                    .collect(),
            },
        ),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn assign_role(
    State(state): State<AppState>,
    caller: Authenticated,
    Validated(body): Validated<AssignRoleRequest>,
) -> Response {
    match state
        .rbac
        .assign(caller.account_id, body.account_id, body.role_id)
        .await
    {
        Ok(()) => ok(MESSAGE_ASSIGNED, ()),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn revoke_role(
    State(state): State<AppState>,
    caller: Authenticated,
    Validated(body): Validated<AssignRoleRequest>,
) -> Response {
    match state
        .rbac
        .revoke(caller.account_id, body.account_id, body.role_id)
        .await
    {
        Ok(()) => ok(MESSAGE_REVOKED, ()),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn list_audit(State(state): State<AppState>, caller: Authenticated) -> Response {
    match state.rbac.recent_events(caller.account_id).await {
        Ok(events) => ok(
            MESSAGE_AUDIT,
            events.into_iter().map(event).collect::<Vec<_>>(),
        ),
        Err(error) => ApiError::from(error).into_response(),
    }
}

fn ok<T: serde::Serialize>(message: &str, data: T) -> Response {
    (StatusCode::OK, Json(Envelope::ok(message, data))).into_response()
}

fn role(role: Role) -> RoleResponse {
    RoleResponse {
        id: role.id,
        name: role.name,
        description: role.description,
        permissions: role
            .permissions
            .into_iter()
            .map(|held| held.as_str().to_owned())
            .collect(),
    }
}

fn event(event: AuditEvent) -> AuditEventResponse {
    AuditEventResponse {
        id: event.id,
        actor_id: event.actor_id,
        action: event.action.as_str().to_owned(),
        subject_id: event.subject_id,
        detail: event.detail,
        recorded_at_ms: event.recorded_at_ms,
    }
}
