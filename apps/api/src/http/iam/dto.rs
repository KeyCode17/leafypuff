use serde::{Deserialize, Serialize};

use crate::application::iam::{
    CompleteSignInInput, RefreshInput, RegisterInput, ResetPasswordInput, Session,
    StartPasswordResetInput, StartSignInInput, VerifyEmailInput,
};
use crate::domain::iam::policy::{ACCESS_TOKEN_TTL_SECONDS, OTP_TTL_SECONDS};

const BEARER: &str = "Bearer";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerifyEmailRequest {
    pub email: String,
    pub code: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompleteSignInRequest {
    pub email: String,
    pub code: String,
    pub device_id: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RefreshRequest {
    pub refresh_token: String,
    pub device_id: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResetPasswordRequest {
    pub email: String,
    pub code: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct ChallengeResponse {
    pub expires_in: i64,
}

impl ChallengeResponse {
    pub const fn issued() -> Self {
        Self {
            expires_in: OTP_TTL_SECONDS,
        }
    }
}

#[derive(Serialize)]
pub struct SessionResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: &'static str,
    pub expires_in: i64,
}

impl From<Session> for SessionResponse {
    fn from(session: Session) -> Self {
        Self {
            access_token: session.access_token,
            refresh_token: session.refresh_secret,
            token_type: BEARER,
            expires_in: ACCESS_TOKEN_TTL_SECONDS,
        }
    }
}

impl From<RegisterRequest> for RegisterInput {
    fn from(request: RegisterRequest) -> Self {
        Self {
            email: request.email,
            password: request.password,
            display_name: request.display_name,
        }
    }
}

impl From<VerifyEmailRequest> for VerifyEmailInput {
    fn from(request: VerifyEmailRequest) -> Self {
        Self {
            email: request.email,
            code: request.code,
        }
    }
}

impl From<SignInRequest> for StartSignInInput {
    fn from(request: SignInRequest) -> Self {
        Self {
            email: request.email,
            password: request.password,
        }
    }
}

impl From<CompleteSignInRequest> for CompleteSignInInput {
    fn from(request: CompleteSignInRequest) -> Self {
        Self {
            email: request.email,
            code: request.code,
            device_id: request.device_id,
        }
    }
}

impl From<ForgotPasswordRequest> for StartPasswordResetInput {
    fn from(request: ForgotPasswordRequest) -> Self {
        Self {
            email: request.email,
        }
    }
}

impl From<ResetPasswordRequest> for ResetPasswordInput {
    fn from(request: ResetPasswordRequest) -> Self {
        Self {
            email: request.email,
            code: request.code,
            password: request.password,
        }
    }
}

impl From<RefreshRequest> for RefreshInput {
    fn from(request: RefreshRequest) -> Self {
        Self {
            refresh_secret: request.refresh_token,
            device_id: request.device_id,
        }
    }
}
