use serde::{Deserialize, Serialize};

use super::error::{CoreError, ERR_LOCATION_UNKNOWN, ERR_WEATHER_UNKNOWN};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Weather {
    Sunny,
    Cloudy,
    Rainy,
    Windy,
}

impl Weather {
    pub const ALL: [Self; 4] = [Self::Sunny, Self::Cloudy, Self::Rainy, Self::Windy];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sunny => "sunny",
            Self::Cloudy => "cloudy",
            Self::Rainy => "rainy",
            Self::Windy => "windy",
        }
    }

    pub fn parse(raw: &str) -> Result<Self, CoreError> {
        Self::ALL
            .into_iter()
            .find(|weather| weather.as_str() == raw)
            .ok_or_else(|| CoreError::Invalid(format!("{ERR_WEATHER_UNKNOWN}: {raw}")))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Location {
    Home,
    Cafe,
    Office,
    Park,
    OnTheRoad,
}

impl Location {
    pub const ALL: [Self; 5] = [
        Self::Home,
        Self::Cafe,
        Self::Office,
        Self::Park,
        Self::OnTheRoad,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Home => "home",
            Self::Cafe => "cafe",
            Self::Office => "office",
            Self::Park => "park",
            Self::OnTheRoad => "on-the-road",
        }
    }

    pub fn parse(raw: &str) -> Result<Self, CoreError> {
        Self::ALL
            .into_iter()
            .find(|location| location.as_str() == raw)
            .ok_or_else(|| CoreError::Invalid(format!("{ERR_LOCATION_UNKNOWN}: {raw}")))
    }
}
