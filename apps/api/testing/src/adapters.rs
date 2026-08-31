use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use leafypuff_api::domain::iam::{
    Clock, EmailSender, IamError, OtpGenerator, OtpPurpose, PasswordHasher, TokenIssuer,
    TokenVerifier,
};
use uuid::Uuid;

const DECOY_HASH: &str = "hashed:\u{0}decoy";

#[derive(Clone)]
pub struct FixedClock {
    now: Arc<Mutex<DateTime<Utc>>>,
}

impl FixedClock {
    pub fn new(now: DateTime<Utc>) -> Self {
        Self {
            now: Arc::new(Mutex::new(now)),
        }
    }

    pub fn advance(&self, by: Duration) {
        let mut now = self.now.lock().expect("the clock lock holds");
        *now += by;
    }
}

impl Clock for FixedClock {
    fn now(&self) -> DateTime<Utc> {
        *self.now.lock().expect("the clock lock holds")
    }
}

#[derive(Clone, Default)]
pub struct RecordingMailer {
    sent: Arc<Mutex<Vec<(String, OtpPurpose, String)>>>,
}

impl RecordingMailer {
    pub fn sent(&self) -> Vec<(String, OtpPurpose, String)> {
        self.sent.lock().expect("the mailer lock holds").clone()
    }

    pub fn last_code(&self) -> String {
        self.sent()
            .last()
            .map(|(_, _, code)| code.clone())
            .expect("a code must have been mailed")
    }
}

#[async_trait]
impl EmailSender for RecordingMailer {
    async fn send_code(&self, to: &str, code: &str, purpose: OtpPurpose) -> Result<(), IamError> {
        let mut sent = self.sent.lock().expect("the mailer lock holds");
        sent.push((to.to_owned(), purpose, code.to_owned()));
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct CountingHasher {
    verifications: Arc<Mutex<u32>>,
}

impl CountingHasher {
    pub fn verifications(&self) -> u32 {
        *self.verifications.lock().expect("the hasher lock holds")
    }
}

impl PasswordHasher for CountingHasher {
    fn hash(&self, plain: &str) -> Result<String, IamError> {
        Ok(format!("hashed:{plain}"))
    }

    fn verify(&self, plain: &str, hash: &str) -> bool {
        let mut count = self.verifications.lock().expect("the hasher lock holds");
        *count += 1;
        format!("hashed:{plain}") == hash
    }

    fn decoy_verify(&self, plain: &str) {
        let _ = self.verify(plain, DECOY_HASH);
    }
}

#[derive(Clone, Default)]
pub struct ScriptedOtp {
    codes: Arc<Mutex<VecDeque<String>>>,
}

impl ScriptedOtp {
    pub fn queue(&self, code: &str) {
        self.codes
            .lock()
            .expect("the generator lock holds")
            .push_back(code.to_owned());
    }
}

impl OtpGenerator for ScriptedOtp {
    fn code(&self) -> Result<String, IamError> {
        self.codes
            .lock()
            .expect("the generator lock holds")
            .pop_front()
            .ok_or_else(|| IamError::Storage("the test queued no code".to_owned()))
    }

    fn digest(&self, code: &str) -> String {
        format!("digest:{code}")
    }
}

#[derive(Clone, Default)]
pub struct SequentialIssuer {
    issued: Arc<Mutex<u32>>,
}

impl TokenIssuer for SequentialIssuer {
    fn access_token(&self, account_id: Uuid) -> Result<String, IamError> {
        Ok(format!("access:{account_id}"))
    }

    fn refresh_secret(&self) -> Result<String, IamError> {
        let mut issued = self.issued.lock().expect("the issuer lock holds");
        *issued += 1;
        Ok(format!("refresh:{issued}"))
    }

    fn digest(&self, secret: &str) -> String {
        format!("digest:{secret}")
    }
}

impl TokenVerifier for SequentialIssuer {
    fn account_id(&self, access_token: &str) -> Result<Uuid, IamError> {
        access_token
            .strip_prefix("access:")
            .and_then(|raw| Uuid::parse_str(raw).ok())
            .ok_or(IamError::InvalidCredentials)
    }
}
