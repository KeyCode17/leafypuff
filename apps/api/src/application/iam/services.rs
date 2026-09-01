use std::sync::Arc;

use crate::domain::iam::{
    AccountRepository, Clock, EmailSender, OtpGenerator, OtpRepository, PasswordHasher,
    RefreshTokenRepository, TokenIssuer, TokenVerifier,
};

use super::consume_challenge::ConsumeChallenge;
use super::issue_challenge::IssueChallenge;
use super::mint_session::MintSession;
use super::{CompleteSignIn, RefreshSession, RegisterAccount, StartSignIn, VerifyEmail};

#[derive(Clone)]
pub struct IamServices {
    pub accounts: Arc<dyn AccountRepository>,
    pub credentials: Arc<dyn RefreshTokenRepository>,
    pub otps: Arc<dyn OtpRepository>,
    pub hasher: Arc<dyn PasswordHasher>,
    pub tokens: Arc<dyn TokenIssuer>,
    pub verifier: Arc<dyn TokenVerifier>,
    pub generator: Arc<dyn OtpGenerator>,
    pub mail: Arc<dyn EmailSender>,
    pub clock: Arc<dyn Clock>,
}

impl IamServices {
    pub fn register(&self) -> RegisterAccount {
        RegisterAccount::new(
            Arc::clone(&self.accounts),
            Arc::clone(&self.hasher),
            Arc::clone(&self.mail),
            self.issue_challenge(),
        )
    }

    pub fn verify_email(&self) -> VerifyEmail {
        VerifyEmail::new(
            Arc::clone(&self.accounts),
            self.consume_challenge(),
            Arc::clone(&self.clock),
        )
    }

    pub fn start_sign_in(&self) -> StartSignIn {
        StartSignIn::new(
            Arc::clone(&self.accounts),
            Arc::clone(&self.hasher),
            self.issue_challenge(),
        )
    }

    pub fn complete_sign_in(&self) -> CompleteSignIn {
        CompleteSignIn::new(
            Arc::clone(&self.accounts),
            self.consume_challenge(),
            self.mint_session(),
        )
    }

    pub fn refresh(&self) -> RefreshSession {
        RefreshSession::new(
            Arc::clone(&self.credentials),
            Arc::clone(&self.tokens),
            Arc::clone(&self.clock),
            self.mint_session(),
        )
    }

    fn issue_challenge(&self) -> IssueChallenge {
        IssueChallenge::new(
            Arc::clone(&self.otps),
            Arc::clone(&self.generator),
            Arc::clone(&self.mail),
            Arc::clone(&self.clock),
        )
    }

    fn consume_challenge(&self) -> ConsumeChallenge {
        ConsumeChallenge::new(
            Arc::clone(&self.otps),
            Arc::clone(&self.generator),
            Arc::clone(&self.clock),
        )
    }

    fn mint_session(&self) -> MintSession {
        MintSession::new(
            Arc::clone(&self.tokens),
            Arc::clone(&self.credentials),
            Arc::clone(&self.clock),
        )
    }
}
