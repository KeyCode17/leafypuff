use crate::domain::{Location, Mood, Sticker, Weather};

#[uniffi::remote(Enum)]
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

#[uniffi::remote(Enum)]
pub enum Weather {
    Sunny,
    Cloudy,
    Rainy,
    Windy,
}

#[uniffi::remote(Enum)]
pub enum Location {
    Home,
    Cafe,
    Office,
    Park,
    OnTheRoad,
}

#[uniffi::remote(Enum)]
pub enum Sticker {
    BunSit,
    BunSleep,
    Carrot,
    Heart,
    Star,
    Cloud,
    Flower,
    Moon,
}
