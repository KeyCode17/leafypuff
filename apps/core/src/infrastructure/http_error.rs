use crate::domain::CoreError;

const ERR_TIMEOUT: &str = "The service did not answer in time";

/// One place that decides what a failed request means. A timeout and an unreachable host produce
/// the same reqwest error type but call for different words: one asks the owner to check their
/// network, the other asks them to wait. Telling them apart here keeps every caller from having to
/// match on a message string.
pub fn reached(error: &reqwest::Error, unreachable: &str) -> CoreError {
    if error.is_timeout() {
        return CoreError::Timeout(ERR_TIMEOUT.to_owned());
    }
    CoreError::Storage(format!("{unreachable}: {error}"))
}
