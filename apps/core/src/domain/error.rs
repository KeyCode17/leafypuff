pub const ERR_ENTRY_NOT_FOUND: &str = "Entry not found";
pub const ERR_ENTRY_ID_INVALID: &str = "Entry id is not a uuid";
pub const ERR_ENTRY_EMPTY: &str = "An entry needs a title or a body";
pub const ERR_MOOD_UNKNOWN: &str = "Unknown mood id";
pub const ERR_STICKER_UNKNOWN: &str = "Unknown sticker id";
pub const ERR_WEATHER_UNKNOWN: &str = "Unknown weather id";
pub const ERR_LOCATION_UNKNOWN: &str = "Unknown location id";
pub const ERR_DATE_UNREADABLE: &str = "Stored date is not ISO-8601";
pub const ERR_TIMESTAMP_UNREADABLE: &str = "Stored timestamp is not RFC3339";
pub const ERR_TEXT_UNREADABLE: &str = "Stored text is not valid UTF-8";
pub const ERR_TOO_MANY_PHOTOS: &str = "An entry holds more photos than an ordinal can number";
pub const ERR_STORE_LOCK_POISONED: &str = "Entry store lock poisoned";
pub const ERR_EXIF_UNREADABLE: &str = "Exif block could not be read";
pub const ERR_PHOTO_TOO_SMALL: &str = "Photo is too small to hold a 3:2 cover";
pub const ERR_PHOTO_UNDECODABLE: &str = "Photo bytes are not a supported image";
pub const ERR_PHOTO_UNENCODABLE: &str = "Cover thumbnail could not be encoded";

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("Entry not found: {id}")]
    NotFound { id: String },
    #[error("Storage failure: {0}")]
    Storage(String),
    #[error("Photo failure: {0}")]
    Photo(String),
    #[error("Exif failure: {0}")]
    Exif(String),
    #[error("Crypto failure: {0}")]
    Crypto(String),
    #[error("Invalid input: {0}")]
    Invalid(String),
}
