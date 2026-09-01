use crate::domain::CoreError;

const ERR_TIMEOUT: &str = "The service did not answer in time";

pub fn reached(error: &reqwest::Error, unreachable: &str) -> CoreError {
    if error.is_timeout() {
        return CoreError::Timeout(ERR_TIMEOUT.to_owned());
    }
    CoreError::Storage(format!("{unreachable}: {error}"))
}
