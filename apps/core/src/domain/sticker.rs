use serde::{Deserialize, Serialize};

use super::error::{CoreError, ERR_STICKER_UNKNOWN};

pub const STICKER_MIN_SIZE: f32 = 36.0;
pub const STICKER_MAX_SIZE: f32 = 180.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sticker {
    BunSit,
    BunSleep,
    Carrot,
    Heart,
    Star,
    Cloud,
    Flower,
    Moon,
    MoodHappy,
    MoodCalm,
    MoodGrateful,
    MoodExcited,
    MoodOkay,
    MoodTired,
    MoodAnxious,
    MoodSad,
    MoodAngry,
    MoodSick,
    MoodLonely,
    MoodLoved,
}

impl Sticker {
    pub const ALL: [Self; 20] = [
        Self::BunSit,
        Self::BunSleep,
        Self::Carrot,
        Self::Heart,
        Self::Star,
        Self::Cloud,
        Self::Flower,
        Self::Moon,
        Self::MoodHappy,
        Self::MoodCalm,
        Self::MoodGrateful,
        Self::MoodExcited,
        Self::MoodOkay,
        Self::MoodTired,
        Self::MoodAnxious,
        Self::MoodSad,
        Self::MoodAngry,
        Self::MoodSick,
        Self::MoodLonely,
        Self::MoodLoved,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BunSit => "bunSit",
            Self::BunSleep => "bunSleep",
            Self::Carrot => "carrot",
            Self::Heart => "heart",
            Self::Star => "star",
            Self::Cloud => "cloud",
            Self::Flower => "flower",
            Self::Moon => "moon",
            Self::MoodHappy => "moodHappy",
            Self::MoodCalm => "moodCalm",
            Self::MoodGrateful => "moodGrateful",
            Self::MoodExcited => "moodExcited",
            Self::MoodOkay => "moodOkay",
            Self::MoodTired => "moodTired",
            Self::MoodAnxious => "moodAnxious",
            Self::MoodSad => "moodSad",
            Self::MoodAngry => "moodAngry",
            Self::MoodSick => "moodSick",
            Self::MoodLonely => "moodLonely",
            Self::MoodLoved => "moodLoved",
        }
    }

    pub fn parse(raw: &str) -> Result<Self, CoreError> {
        Self::ALL
            .into_iter()
            .find(|sticker| sticker.as_str() == raw)
            .ok_or_else(|| CoreError::Invalid(format!("{ERR_STICKER_UNKNOWN}: {raw}")))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlacedSticker {
    pub key: String,
    pub sticker: Sticker,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub rotation: f32,
}

impl PlacedSticker {
    pub fn new(key: String, sticker: Sticker, x: f32, y: f32, size: f32, rotation: f32) -> Self {
        Self {
            key,
            sticker,
            x,
            y,
            size: size.clamp(STICKER_MIN_SIZE, STICKER_MAX_SIZE),
            rotation,
        }
    }
}
