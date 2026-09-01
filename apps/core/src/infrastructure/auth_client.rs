use std::time::Duration;

use reqwest::Client;
use serde_json::{Value, json};

use super::http_error::reached;
use crate::domain::{Challenge, CoreError, Rejection, Session};

/// A phone leaves wifi mid-request and the socket simply stops answering. Without a deadline the
/// call waits forever, the screen stays on its spinner, and the owner cannot tell a slow network
/// from a dead button. These are generous enough for argon2 on a small server and short enough
/// that a hang becomes an error someone can act on.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

const REGISTER_PATH: &str = "/v1/auth/register";
const SIGN_IN_PATH: &str = "/v1/auth/sign-in";
const VERIFY_SIGN_IN_PATH: &str = "/v1/auth/sign-in/verify";
const VERIFY_EMAIL_PATH: &str = "/v1/auth/verify-email";

const ERR_UNREACHABLE: &str = "The account service could not be reached";
const ERR_SHAPE: &str = "The account service answered an unexpected shape";

/// Every call the sign-in flow makes. It lives here rather than in Kotlin so the device keeps one
/// http stack, and so the error the UI shows is the API's own code rather than a status number.
pub struct AuthClient {
    client: Client,
    base_url: String,
}

impl AuthClient {
    pub fn new(base_url: String) -> Result<Self, CoreError> {
        let client = Client::builder()
            .connect_timeout(CONNECT_TIMEOUT)
            .timeout(REQUEST_TIMEOUT)
            .build()
            .map_err(|_| CoreError::Storage(ERR_UNREACHABLE.to_owned()))?;
        Ok(Self { client, base_url })
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

    async fn post(&self, path: &str, body: &Value) -> Result<Value, CoreError> {
        let response = self
            .client
            .post(format!("{}{path}", self.base_url))
            .json(body)
            .send()
            .await
            .map_err(|error| reached(&error, ERR_UNREACHABLE))?;

        let parsed: Value = response
            .json()
            .await
            .map_err(|_| CoreError::Storage(ERR_SHAPE.to_owned()))?;

        if parsed["success"].as_bool() == Some(true) {
            return Ok(parsed);
        }
        // The API's own code, not the status number: the UI can tell "wrong password" from
        // "already registered" without inventing its own vocabulary.
        let code = parsed["error"]["code"].as_str().unwrap_or_default();
        Err(CoreError::Rejected {
            rejection: Rejection::from_code(code),
            detail: parsed["error"]["detail"]
                .as_str()
                .unwrap_or_default()
                .to_owned(),
        })
    }
}

fn challenge(body: &Value) -> Result<Challenge, CoreError> {
    Ok(Challenge {
        expires_in_seconds: body["data"]["expires_in"]
            .as_i64()
            .ok_or_else(|| CoreError::Storage(ERR_SHAPE.to_owned()))?,
    })
}

fn session(body: &Value) -> Result<Session, CoreError> {
    let data = &body["data"];
    let read = |key: &str| {
        data[key]
            .as_str()
            .map(str::to_owned)
            .ok_or_else(|| CoreError::Storage(ERR_SHAPE.to_owned()))
    };
    Ok(Session {
        access_token: read("access_token")?,
        refresh_token: read("refresh_token")?,
        expires_in_seconds: data["expires_in"]
            .as_i64()
            .ok_or_else(|| CoreError::Storage(ERR_SHAPE.to_owned()))?,
    })
}
