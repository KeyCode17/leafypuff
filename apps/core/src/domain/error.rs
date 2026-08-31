pub const ERR_ENTRY_ID_INVALID: &str = "Entry id is not a uuid";
pub const ERR_MOOD_UNKNOWN: &str = "Unknown mood id";
pub const ERR_STICKER_UNKNOWN: &str = "Unknown sticker id";
pub const ERR_WEATHER_UNKNOWN: &str = "Unknown weather id";
pub const ERR_LOCATION_UNKNOWN: &str = "Unknown location id";

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Entry not found")]
    NotFound,
    #[error("Storage failure: {0}")]
    Storage(String),
    #[error("Invalid entry: {0}")]
    Invalid(String),
}
