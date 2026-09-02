use crate::domain::{CoreError, Rejection};

const ERR_TIMEOUT: &str = "The service did not answer in time";

pub const ERR_SESSION_EXPIRED: &str = "This session is no longer accepted";

pub fn reached(error: &reqwest::Error, unreachable: &str) -> CoreError {
    if error.is_timeout() {
        return CoreError::Timeout(ERR_TIMEOUT.to_owned());
    }
    CoreError::Storage(format!("{unreachable}: {error}"))
}

pub fn refused(status: reqwest::StatusCode, refused: &str) -> CoreError {
    if status == reqwest::StatusCode::UNAUTHORIZED {
        return CoreError::Rejected {
            rejection: Rejection::InvalidCredentials,
            detail: ERR_SESSION_EXPIRED.to_owned(),
        };
    }
    CoreError::Storage(format!("{refused}: {status}"))
}
