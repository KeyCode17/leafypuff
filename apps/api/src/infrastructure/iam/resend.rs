use std::time::Duration;

use async_trait::async_trait;

use reqwest::Client;
use serde_json::json;

use crate::domain::iam::{EmailSender, IamError, OtpPurpose};

use super::mail_body;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

pub struct ResendEmailSender {
    client: Client,
    api_key: String,
    from: String,
    endpoint: String,
}

impl ResendEmailSender {
    pub fn new(api_key: String, from: String, endpoint: String) -> Result<Self, IamError> {
        let client = Client::builder()
            .timeout(REQUEST_TIMEOUT)
            .build()
            .map_err(|error| IamError::Mail(error.without_url().to_string()))?;
        Ok(Self {
            client,
            api_key,
            from,
            endpoint,
        })
    }
}

impl ResendEmailSender {
    async fn deliver(&self, to: &str, body: mail_body::Body) -> Result<(), IamError> {
        let payload = json!({
            "from": self.from,
            "to": [to],
            "subject": body.subject,
            "text": body.text,
            "html": body.html,
        });

        let response = self
            .client
            .post(&self.endpoint)
            .bearer_auth(&self.api_key)
            .json(&payload)
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

#[async_trait]
impl EmailSender for ResendEmailSender {
    async fn send_code(&self, to: &str, code: &str, purpose: OtpPurpose) -> Result<(), IamError> {
        self.deliver(to, mail_body::code(code, purpose)).await
    }

    async fn send_existing_account_notice(&self, to: &str) -> Result<(), IamError> {
        self.deliver(to, mail_body::existing_account()).await
    }
}
