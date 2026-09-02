use reqwest::Client;
use serde_json::{Value, json};

use super::http_client;
use super::http_error::reached;
use crate::domain::{Challenge, CoreError, Rejection, Session};

const REGISTER_PATH: &str = "/v1/auth/register";
const SIGN_IN_PATH: &str = "/v1/auth/sign-in";
const VERIFY_SIGN_IN_PATH: &str = "/v1/auth/sign-in/verify";
const VERIFY_EMAIL_PATH: &str = "/v1/auth/verify-email";
const REFRESH_PATH: &str = "/v1/auth/refresh";
const CHANGE_EMAIL_PATH: &str = "/v1/auth/email/change";
const CONFIRM_EMAIL_PATH: &str = "/v1/auth/email/confirm";
const FORGOT_PASSWORD_PATH: &str = "/v1/auth/password/forgot";
const RESET_PASSWORD_PATH: &str = "/v1/auth/password/reset";

const ERR_UNREACHABLE: &str = "The account service could not be reached";
const ERR_SHAPE: &str = "The account service answered an unexpected shape";

pub struct AuthClient {
    client: Client,
    base_url: String,
}

impl AuthClient {
    pub fn new(base_url: String) -> Result<Self, CoreError> {
        Ok(Self {
            client: http_client::plain(ERR_UNREACHABLE)?,
            base_url,
        })
    }

    pub fn for_device(base_url: String, device_id: &str) -> Result<Self, CoreError> {
        Ok(Self {
            client: http_client::for_device(device_id, ERR_UNREACHABLE)?,
            base_url,
        })
    }

    pub async fn register(
        &self,
        email: String,
        password: String,
        display_name: Option<String>,
    ) -> Result<Challenge, CoreError> {
        let body = self
            .post(
                REGISTER_PATH,
                &json!({
                    "email": email,
                    "password": password,
                    "display_name": display_name,
                }),
            )
            .await?;
        challenge(&body)
    }

    pub async fn verify_email(&self, email: String, code: String) -> Result<(), CoreError> {
        self.post(VERIFY_EMAIL_PATH, &json!({ "email": email, "code": code }))
            .await?;
        Ok(())
    }

    pub async fn forgot_password(&self, email: String) -> Result<Challenge, CoreError> {
        let body = self
            .post(FORGOT_PASSWORD_PATH, &json!({ "email": email }))
            .await?;
        challenge(&body)
    }

    pub async fn reset_password(
        &self,
        email: String,
        code: String,
        password: String,
    ) -> Result<(), CoreError> {
        self.post(
            RESET_PASSWORD_PATH,
            &json!({ "email": email, "code": code, "password": password }),
        )
        .await?;
        Ok(())
    }

    pub async fn sign_in(&self, email: String, password: String) -> Result<Challenge, CoreError> {
        let body = self
            .post(
                SIGN_IN_PATH,
                &json!({ "email": email, "password": password }),
            )
            .await?;
        challenge(&body)
    }

    pub async fn verify_sign_in(
        &self,
        email: String,
        code: String,
        device_id: String,
    ) -> Result<Session, CoreError> {
        let body = self
            .post(
                VERIFY_SIGN_IN_PATH,
                &json!({ "email": email, "code": code, "device_id": device_id }),
            )
            .await?;
        session(&body)
    }

    pub async fn refresh(
        &self,
        refresh_token: String,
        device_id: String,
    ) -> Result<Session, CoreError> {
        let body = self
            .post(
                REFRESH_PATH,
                &json!({ "refresh_token": refresh_token, "device_id": device_id }),
            )
            .await?;
        session(&body)
    }

    pub async fn change_email(
        &self,
        access_token: &str,
        email: String,
    ) -> Result<Challenge, CoreError> {
        let body = self
            .bearer(CHANGE_EMAIL_PATH, access_token, &json!({ "email": email }))
            .await?;
        challenge(&body)
    }

    pub async fn confirm_email(
        &self,
        access_token: &str,
        code: String,
    ) -> Result<String, CoreError> {
        let body = self
            .bearer(CONFIRM_EMAIL_PATH, access_token, &json!({ "code": code }))
            .await?;
        body["data"]["email"]
            .as_str()
            .map(str::to_owned)
            .ok_or_else(|| CoreError::Unreadable(ERR_SHAPE.to_owned()))
    }

    async fn bearer(
        &self,
        path: &str,
        access_token: &str,
        body: &Value,
    ) -> Result<Value, CoreError> {
        let response = self
            .client
            .post(format!("{}{path}", self.base_url))
            .bearer_auth(access_token)
            .json(body)
            .send()
            .await
            .map_err(|error| reached(&error, ERR_UNREACHABLE))?;
        read(response).await
    }

    async fn post(&self, path: &str, body: &Value) -> Result<Value, CoreError> {
        let response = self
            .client
            .post(format!("{}{path}", self.base_url))
            .json(body)
            .send()
            .await
            .map_err(|error| reached(&error, ERR_UNREACHABLE))?;
        read(response).await
    }
}

async fn read(response: reqwest::Response) -> Result<Value, CoreError> {
    let parsed: Value = response
        .json()
        .await
        .map_err(|_| CoreError::Unreadable(ERR_SHAPE.to_owned()))?;

    if parsed["success"].as_bool() == Some(true) {
        return Ok(parsed);
    }
    let code = parsed["error"]["code"].as_str().unwrap_or_default();
    Err(CoreError::Rejected {
        rejection: Rejection::from_code(code),
        detail: parsed["error"]["detail"]
            .as_str()
            .unwrap_or_default()
            .to_owned(),
    })
}

fn challenge(body: &Value) -> Result<Challenge, CoreError> {
    Ok(Challenge {
        expires_in_seconds: body["data"]["expires_in"]
            .as_i64()
            .ok_or_else(|| CoreError::Unreadable(ERR_SHAPE.to_owned()))?,
    })
}

fn session(body: &Value) -> Result<Session, CoreError> {
    let data = &body["data"];
    let read = |key: &str| {
        data[key]
            .as_str()
            .map(str::to_owned)
            .ok_or_else(|| CoreError::Unreadable(ERR_SHAPE.to_owned()))
    };
    Ok(Session {
        access_token: read("access_token")?,
        refresh_token: read("refresh_token")?,
        expires_in_seconds: data["expires_in"]
            .as_i64()
            .ok_or_else(|| CoreError::Unreadable(ERR_SHAPE.to_owned()))?,
    })
}
