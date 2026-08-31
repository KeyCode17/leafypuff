use serde::{Deserialize, Serialize};

use super::error::{CoreError, ERR_MOOD_UNKNOWN};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mood {
    Happy,
    Calm,
    Grateful,
    Excited,
    Okay,
    Tired,
    Anxious,
    Sad,
    Angry,
    Sick,
    Lonely,
    Loved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoodGroup {
    Positive,
    Neutral,
    Negative,
}

impl Mood {
    pub const ALL: [Self; 12] = [
        Self::Happy,
        Self::Calm,
        Self::Grateful,
        Self::Excited,
        Self::Okay,
        Self::Tired,
        Self::Anxious,
        Self::Sad,
        Self::Angry,
        Self::Sick,
        Self::Lonely,
        Self::Loved,
    ];

    const fn facts(self) -> (&'static str, &'static str, &'static str, MoodGroup) {
        match self {
            Self::Happy => ("happy", "Happy", "#8B9A5F", MoodGroup::Positive),
            Self::Calm => ("calm", "Calm", "#A8B98B", MoodGroup::Positive),
            Self::Grateful => ("grateful", "Grateful", "#C3CE9E", MoodGroup::Positive),
            Self::Excited => ("excited", "Excited", "#E3C766", MoodGroup::Positive),
            Self::Okay => ("okay", "Okay", "#C3C8B4", MoodGroup::Neutral),
            Self::Tired => ("tired", "Tired", "#A3A896", MoodGroup::Neutral),
            Self::Anxious => ("anxious", "Anxious", "#C9A87A", MoodGroup::Negative),
            Self::Sad => ("sad", "Sad", "#8FA0A8", MoodGroup::Negative),
            Self::Angry => ("angry", "Angry", "#EF4E4E", MoodGroup::Negative),
            Self::Sick => ("sick", "Sick", "#9CB49A", MoodGroup::Negative),
            Self::Lonely => ("lonely", "Lonely", "#A6A2AE", MoodGroup::Negative),
            Self::Loved => ("loved", "Loved", "#E0909A", MoodGroup::Positive),
        }
    }

    pub const fn as_str(self) -> &'static str {
        self.facts().0
    }

    pub const fn label(self) -> &'static str {
        self.facts().1
    }

    pub const fn dot_colour(self) -> &'static str {
        self.facts().2
    }

    pub const fn group(self) -> MoodGroup {
        self.facts().3
    }

    pub fn parse(raw: &str) -> Result<Self, CoreError> {
        Self::ALL
            .into_iter()
            .find(|mood| mood.as_str() == raw)
            .ok_or_else(|| CoreError::Invalid(format!("{ERR_MOOD_UNKNOWN}: {raw}")))
    }
}
