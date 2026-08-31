use serde::{Deserialize, Serialize};

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
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Happy => "happy",
            Self::Calm => "calm",
            Self::Grateful => "grateful",
            Self::Excited => "excited",
            Self::Okay => "okay",
            Self::Tired => "tired",
            Self::Anxious => "anxious",
            Self::Sad => "sad",
            Self::Angry => "angry",
            Self::Sick => "sick",
            Self::Lonely => "lonely",
            Self::Loved => "loved",
        }
    }

    pub const fn group(self) -> MoodGroup {
        match self {
            Self::Happy | Self::Calm | Self::Grateful | Self::Excited | Self::Loved => {
                MoodGroup::Positive
            }
            Self::Okay | Self::Tired => MoodGroup::Neutral,
            Self::Anxious | Self::Sad | Self::Angry | Self::Sick | Self::Lonely => {
                MoodGroup::Negative
            }
        }
    }
}
