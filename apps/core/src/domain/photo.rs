use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub const COVER_ORDINAL: i32 = 0;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhotoRef {
    pub id: String,
    pub path: String,
    pub ordinal: i32,
    pub taken_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhotoKind {
    Original,
    Cover,
}

impl PhotoKind {
    pub const fn suffix(self) -> &'static str {
        match self {
            Self::Original => "",
            Self::Cover => ".cover",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Original => "original",
            Self::Cover => "cover",
        }
    }
}
