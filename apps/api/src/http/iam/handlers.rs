use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::domain::iam::IamError;
use crate::http::envelope::Envelope;
use crate::http::error::ApiError;
use crate::http::state::AppState;
use crate::http::validated::Validated;

use super::dto::{
    ChallengeResponse, CompleteSignInRequest, ForgotPasswordRequest, RefreshRequest,
    RegisterRequest, ResetPasswordRequest, SessionResponse, SignInRequest, VerifyEmailRequest,
};

const MESSAGE_REGISTERED: &str = "Check your inbox for a verification code";
const MESSAGE_VERIFIED: &str = "Email verified";
const MESSAGE_CODE_SENT: &str = "Check your inbox for a sign-in code";
const MESSAGE_SIGNED_IN: &str = "Signed in";
const MESSAGE_REFRESHED: &str = "Session refreshed";
const MESSAGE_RESET_SENT: &str = "Check your inbox for a reset code";
const MESSAGE_PASSWORD_CHANGED: &str = "Password changed";

pub async fn register(
    State(state): State<AppState>,
    Validated(body): Validated<RegisterRequest>,
) -> Response {
    match state.iam.register().execute(body.into()).await {
        Ok(()) | Err(IamError::EmailAlreadyRegistered) => challenge(MESSAGE_REGISTERED),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn verify_email(
    State(state): State<AppState>,
    Validated(body): Validated<VerifyEmailRequest>,
) -> Response {
    match state.iam.verify_email().execute(body.into()).await {
        Ok(()) => (StatusCode::OK, Json(Envelope::ok(MESSAGE_VERIFIED, ()))).into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn sign_in(
    State(state): State<AppState>,
    Validated(body): Validated<SignInRequest>,
) -> Response {
    match state.iam.start_sign_in().execute(body.into()).await {
        Ok(()) => challenge(MESSAGE_CODE_SENT),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn complete_sign_in(
    State(state): State<AppState>,
    Validated(body): Validated<CompleteSignInRequest>,
) -> Response {
    match state.iam.complete_sign_in().execute(body.into()).await {
        Ok(session) => session_response(MESSAGE_SIGNED_IN, session.into()),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn forgot_password(
    State(state): State<AppState>,
    Validated(body): Validated<ForgotPasswordRequest>,
) -> Response {
    match state.iam.start_password_reset().execute(body.into()).await {
        Ok(()) => challenge(MESSAGE_RESET_SENT),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn reset_password(
    State(state): State<AppState>,
    Validated(body): Validated<ResetPasswordRequest>,
) -> Response {
    match state.iam.reset_password().execute(body.into()).await {
        Ok(()) => (
            StatusCode::OK,
            Json(Envelope::ok(MESSAGE_PASSWORD_CHANGED, ())),
        )
            .into_response(),
        Err(error) => ApiError::from(error).into_response(),
    }
}

pub async fn refresh(
    State(state): State<AppState>,
    Validated(body): Validated<RefreshRequest>,
) -> Response {
    match state.iam.refresh().execute(body.into()).await {
        Ok(session) => session_response(MESSAGE_REFRESHED, session.into()),
        Err(error) => ApiError::from(error).into_response(),
    }
}

fn challenge(message: &str) -> Response {
    (
        StatusCode::ACCEPTED,
        Json(Envelope::ok(message, ChallengeResponse::issued())),
    )
        .into_response()
}

fn session_response(message: &str, session: SessionResponse) -> Response {
    (StatusCode::OK, Json(Envelope::ok(message, session))).into_response()
}
