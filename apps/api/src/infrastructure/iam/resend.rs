use std::time::Duration;

use async_trait::async_trait;

use reqwest::Client;
use serde_json::json;

use crate::domain::iam::{EmailSender, IamError, OtpPurpose};

use super::mail_body;

const RESEND_ENDPOINT: &str = "https://api.resend.com/emails";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

pub struct ResendEmailSender {
    client: Client,
    api_key: String,
    from: String,
}

impl ResendEmailSender {
    pub fn new(api_key: String, from: String) -> Result<Self, IamError> {
        let client = Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .build()
            .map_err(|error| IamError::Mail(error.without_url().to_string()))?;
        Ok(Self {
            client,
            api_key,
            from,
        })
    }
}

#[async_trait]
impl EmailSender for ResendEmailSender {
    async fn send_code(&self, to: &str, code: &str, purpose: OtpPurpose) -> Result<(), IamError> {
        let body = json!({
            "from": self.from,
            "to": [to],
            "subject": mail_body::subject(purpose),
            "text": mail_body::text(code, purpose),
            "html": mail_body::html(code, purpose),
        });

        let response = self
            .client
            .post(RESEND_ENDPOINT)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|error| IamError::Mail(error.without_url().to_string()))?;

        let status = response.status();
        if status.is_success() {
            return Ok(());
        }
        Err(IamError::Mail(format!("Resend answered {status}")))
    }
}
