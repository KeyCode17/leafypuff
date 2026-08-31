use chrono::{NaiveDate, Utc};
use leafypuff_core::domain::{
    Entry, EntryId, Location, Mood, MoodGroup, PhotoRef, PlacedSticker, Sticker, Weather,
};

#[test]
fn mood_groups_partition_all_twelve() {
    let positive: Vec<Mood> = Mood::ALL
        .into_iter()
        .filter(|mood| mood.group() == MoodGroup::Positive)
        .collect();
    let neutral: Vec<Mood> = Mood::ALL
        .into_iter()
        .filter(|mood| mood.group() == MoodGroup::Neutral)
        .collect();
    let negative: Vec<Mood> = Mood::ALL
        .into_iter()
        .filter(|mood| mood.group() == MoodGroup::Negative)
        .collect();

    assert_eq!(Mood::ALL.len(), 12);
    assert_eq!(positive.len(), 5);
    assert_eq!(neutral.len(), 2);
    assert_eq!(negative.len(), 5);
    assert_eq!(
        positive.len() + neutral.len() + negative.len(),
        Mood::ALL.len()
    );
}

#[test]
fn every_mood_carries_a_label_and_a_dot_colour() {
    for mood in Mood::ALL {
        assert!(!mood.label().is_empty());
        assert_eq!(mood.dot_colour().len(), 7);
        assert!(mood.dot_colour().starts_with('#'));
    }
}

#[test]
fn ids_round_trip_through_their_lowercase_strings() {
    for mood in Mood::ALL {
        assert_eq!(
            Mood::parse(mood.as_str()).expect("a known mood id parses"),
            mood
        );
    }
    for sticker in Sticker::ALL {
        assert_eq!(
            Sticker::parse(sticker.as_str()).expect("a known sticker id parses"),
            sticker
        );
    }
    for weather in Weather::ALL {
        assert_eq!(
            Weather::parse(weather.as_str()).expect("a known weather id parses"),
            weather
        );
    }
    for location in Location::ALL {
        assert_eq!(
            Location::parse(location.as_str()).expect("a known location id parses"),
            location
        );
    }
    assert_eq!(Sticker::BunSleep.as_str(), "bunSleep");
    assert_eq!(Location::OnTheRoad.as_str(), "on-the-road");
    assert!(Mood::parse("euphoric").is_err());
}

#[test]
fn placed_sticker_clamps_size() {
    let small = PlacedSticker::new("a".to_owned(), Sticker::Heart, 0.0, 0.0, 35.0, 0.0);
    let large = PlacedSticker::new("b".to_owned(), Sticker::Heart, 0.0, 0.0, 200.0, 0.0);
    assert!((small.size - 36.0).abs() < f32::EPSILON);
    assert!((large.size - 180.0).abs() < f32::EPSILON);
}

#[test]
fn cover_is_the_ordinal_zero_photo_and_nothing_else() {
    let photo = |id: &str, ordinal: i32| PhotoRef {
        id: id.to_owned(),
        path: format!("/photos/{id}.jpg"),
        ordinal,
        taken_at: None,
    };
    let now = Utc::now();
    let entry = Entry {
        id: EntryId::new(),
        date: NaiveDate::from_ymd_opt(2026, 8, 31).expect("a real date"),
        mood: Mood::Loved,
        title: "Quiet morning".to_owned(),
        body: "Tea on the balcony.".to_owned(),
        tags: vec!["#slowday".to_owned()],
        weather: Some(Weather::Sunny),
        location: Some(Location::Home),
        photos: vec![photo("second", 1), photo("first", 0)],
        stickers: Vec::new(),
        created_at: now,
        updated_at: now,
    };

    assert_eq!(entry.cover().map(|found| found.id.as_str()), Some("first"));

    let coverless = Entry {
        photos: vec![photo("second", 1)],
        ..entry
    };
    assert!(coverless.cover().is_none());
}
