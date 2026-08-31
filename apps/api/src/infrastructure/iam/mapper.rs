use chrono::{DateTime, Utc};
use sea_orm::DbErr;

use crate::domain::iam::error::ERR_UNKNOWN_OTP_PURPOSE;
use crate::domain::iam::{Account, IamError, OtpCode, OtpPurpose, RefreshToken};

use super::entity::{accounts, otp_codes, refresh_tokens};

const UNIQUE_VIOLATION: &str = "unique constraint";

pub fn storage(error: DbErr) -> IamError {
    IamError::Storage(error.to_string())
}

pub fn insert_conflict(error: DbErr, conflict: IamError) -> IamError {
    if error.to_string().contains(UNIQUE_VIOLATION) {
        return conflict;
    }
    storage(error)
}

pub fn account(row: accounts::Model) -> Account {
    Account {
        id: row.id,
        email: row.email,
        password_hash: row.password_hash,
        display_name: row.display_name,
        email_verified_at: row.email_verified_at.map(utc),
    }
}

pub fn refresh_token(row: refresh_tokens::Model) -> RefreshToken {
    RefreshToken {
        id: row.id,
        account_id: row.account_id,
        device_id: row.device_id,
        token_hash: row.token_hash,
        expires_at: utc(row.expires_at),
        revoked_at: row.revoked_at.map(utc),
    }
}

pub fn otp_code(row: otp_codes::Model) -> Result<OtpCode, IamError> {
    let purpose = OtpPurpose::from_stored(&row.purpose)
        .ok_or_else(|| IamError::Storage(ERR_UNKNOWN_OTP_PURPOSE.to_owned()))?;
    Ok(OtpCode {
        id: row.id,
        account_id: row.account_id,
        code_hash: row.code_hash,
        purpose,
        attempts: row.attempts,
        expires_at: utc(row.expires_at),
        consumed_at: row.consumed_at.map(utc),
    })
}

fn utc(value: DateTime<chrono::FixedOffset>) -> DateTime<Utc> {
    value.with_timezone(&Utc)
}
