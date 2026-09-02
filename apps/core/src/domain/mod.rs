pub mod auth;
pub mod crop;
pub mod crypto;
pub mod entry;
pub mod error;
pub mod mood;
pub mod photo;
pub mod ports;
pub mod profile;
pub mod stats;
pub mod sticker;
pub mod sync;
pub mod weather;

pub use auth::{Challenge, Rejection, Session};
pub use crop::{COVER_MAX_HEIGHT, COVER_MAX_WIDTH, CropBox, cover_size, top_anchored_cover};
pub use entry::{Entry, EntryId};
pub use error::CoreError;
pub use mood::{Mood, MoodGroup};
pub use photo::{COVER_ORDINAL, PhotoKind, PhotoRef};
pub use ports::{
    Clock, ContentSealer, EntryRepository, ExifReader, FieldSealer, PhotoStore, ThumbnailMaker,
};
pub use profile::Profile;
pub use stats::{
    GroupCount, MoodCount, SPREAD_LIMIT, StatsRange, StatsSummary, TAG_LIMIT, TagCount,
    WeekdayCount, summarise,
};
pub use sticker::{PlacedSticker, STICKER_MAX_SIZE, STICKER_MIN_SIZE, Sticker};
pub use sync::{OutboundEntry, SyncOutcome};
pub use weather::{Location, Weather};
