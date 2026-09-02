use super::dto::{
    ChangeEmailRequest, CompleteSignInRequest, ConfirmEmailRequest, ForgotPasswordRequest,
    RefreshRequest, RegisterRequest, ResetPasswordRequest, SignInRequest, VerifyEmailRequest,
};
use crate::http::validated::ValidatedBody;

const MINIMUM_PASSWORD_LENGTH: usize = 12;
const CODE_LENGTH: usize = 6;
const MAXIMUM_DEVICE_ID_LENGTH: usize = 128;

impl ValidatedBody for RegisterRequest {
    fn validate(&self) -> Result<(), &'static str> {
        email(&self.email)?;
        password(&self.password)
    }
}

impl ValidatedBody for VerifyEmailRequest {
    fn validate(&self) -> Result<(), &'static str> {
        email(&self.email)?;
        code(&self.code)
    }
}

impl ValidatedBody for SignInRequest {
    fn validate(&self) -> Result<(), &'static str> {
        email(&self.email)?;
        password(&self.password)
    }
}

impl ValidatedBody for CompleteSignInRequest {
    fn validate(&self) -> Result<(), &'static str> {
        email(&self.email)?;
        code(&self.code)?;
        device_id(&self.device_id)
    }
}

impl ValidatedBody for ForgotPasswordRequest {
    fn validate(&self) -> Result<(), &'static str> {
        email(&self.email)
    }
}

impl ValidatedBody for ResetPasswordRequest {
    fn validate(&self) -> Result<(), &'static str> {
        email(&self.email)?;
        code(&self.code)?;
        password(&self.password)
    }
}

impl ValidatedBody for ChangeEmailRequest {
    fn validate(&self) -> Result<(), &'static str> {
        email(&self.email)
    }
}

impl ValidatedBody for ConfirmEmailRequest {
    fn validate(&self) -> Result<(), &'static str> {
        code(&self.code)
    }
}

impl ValidatedBody for RefreshRequest {
    fn validate(&self) -> Result<(), &'static str> {
        if self.refresh_token.trim().is_empty() {
            return Err("refresh_token is empty");
        }
        device_id(&self.device_id)
    }
}

fn email(value: &str) -> Result<(), &'static str> {
    let trimmed = value.trim();
    match trimmed.split_once('@') {
        Some((local, domain)) if !local.is_empty() && domain.contains('.') => Ok(()),
        _ => Err("email is not an address"),
    }
}

fn password(value: &str) -> Result<(), &'static str> {
    if value.chars().count() < MINIMUM_PASSWORD_LENGTH {
        return Err("password is too short");
    }
    Ok(())
}

fn code(value: &str) -> Result<(), &'static str> {
    if value.len() != CODE_LENGTH || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err("code is not six digits");
    }
    Ok(())
}

fn device_id(value: &str) -> Result<(), &'static str> {
    if value.trim().is_empty() || value.len() > MAXIMUM_DEVICE_ID_LENGTH {
        return Err("device_id is empty or too long");
    }
    Ok(())
}
