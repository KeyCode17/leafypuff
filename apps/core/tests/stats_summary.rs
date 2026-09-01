use chrono::NaiveDate;
use leafypuff_core::domain::{
    Entry, EntryId, Mood, MoodGroup, SPREAD_LIMIT, StatsRange, StatsSummary, summarise,
};

fn today() -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 9, 1).expect("2026-09-01 is a real date")
}

fn entry(day: u32, mood: Mood, tags: &[&str]) -> Entry {
    let stamp = chrono::Utc::now();
    Entry {
        id: EntryId::new(),
        date: NaiveDate::from_ymd_opt(2026, 8, day).expect("august day"),
        mood,
        title: format!("t{day}"),
        body: format!("b{day}"),
        tags: tags.iter().map(|tag| (*tag).to_owned()).collect(),
        weather: None,
        location: None,
        photos: Vec::new(),
        stickers: Vec::new(),
        created_at: stamp,
        updated_at: stamp,
    }
}

fn plain(day: u32) -> Entry {
    entry(day, Mood::Calm, &[])
}

fn all_time(entries: &[Entry]) -> StatsSummary {
    summarise(entries, StatsRange::AllTime, today())
}

#[test]
fn a_gap_of_one_day_ends_the_streak() {
    let entries = [plain(26), plain(27), plain(28), plain(30), plain(31)];

    let summary = all_time(&entries);

    assert_eq!(summary.longest_streak, 3);
    assert_eq!(summary.days_written, 5);
}

#[test]
fn two_entries_on_one_day_count_as_one_day_of_the_streak() {
    let entries = [plain(30), entry(30, Mood::Excited, &[]), plain(31)];

    let summary = all_time(&entries);

    assert_eq!(summary.longest_streak, 2);
    assert_eq!(summary.days_written, 2);
}

#[test]
fn a_single_day_is_a_streak_of_one() {
    let summary = all_time(&[plain(31)]);

    assert_eq!(summary.longest_streak, 1);
    assert_eq!(summary.days_written, 1);
}

#[test]
fn an_empty_diary_yields_zeroes_and_every_bucket_still_present() {
    let summary = all_time(&[]);

    assert_eq!(summary.days_written, 0);
    assert_eq!(summary.longest_streak, 0);
    assert!(summary.mood_spread.is_empty());
    assert!(summary.tags.is_empty());
    assert_eq!(
        summary
            .mood_balance
            .iter()
            .map(|row| row.group)
            .collect::<Vec<MoodGroup>>(),
        MoodGroup::ALL.to_vec()
    );
    assert!(summary.mood_balance.iter().all(|row| row.count == 0));
    assert_eq!(summary.weekdays.len(), 7);
    assert!(summary.weekdays.iter().all(|row| row.count == 0));
}

#[test]
fn moods_fall_into_the_three_handoff_groups() {
    let entries = [
        entry(24, Mood::Happy, &[]),
        entry(25, Mood::Calm, &[]),
        entry(26, Mood::Okay, &[]),
        entry(27, Mood::Sad, &[]),
        entry(28, Mood::Angry, &[]),
    ];

    let summary = all_time(&entries);

    assert_eq!(
        summary
            .mood_balance
            .iter()
            .map(|row| row.count)
            .collect::<Vec<u32>>(),
        vec![2, 1, 2]
    );
}

#[test]
fn the_seven_day_range_keeps_day_six_and_drops_day_seven() {
    let entries = [plain(26), plain(25)];

    let summary = summarise(&entries, StatsRange::SevenDays, today());

    assert_eq!(summary.days_written, 1);
}

#[test]
fn the_spread_keeps_at_most_six_moods_ordered_by_count() {
    let entries = [
        entry(20, Mood::Happy, &[]),
        entry(21, Mood::Happy, &[]),
        entry(22, Mood::Happy, &[]),
        entry(23, Mood::Calm, &[]),
        entry(24, Mood::Calm, &[]),
        entry(25, Mood::Okay, &[]),
        entry(26, Mood::Sad, &[]),
        entry(27, Mood::Angry, &[]),
        entry(28, Mood::Tired, &[]),
        entry(29, Mood::Loved, &[]),
    ];

    let summary = all_time(&entries);
    let counts: Vec<u32> = summary.mood_spread.iter().map(|row| row.count).collect();
    let mut ordered = counts.clone();
    ordered.sort_unstable_by(|left, right| right.cmp(left));

    assert_eq!(summary.mood_spread.len(), SPREAD_LIMIT);
    assert_eq!(summary.mood_spread[0].mood, Mood::Happy);
    assert_eq!(counts[0], 3);
    assert_eq!(counts, ordered);
}

#[test]
fn weekday_buckets_are_sunday_first() {
    let summary = all_time(&[plain(30), plain(31), plain(26)]);

    assert_eq!(summary.weekdays[0].count, 1);
    assert_eq!(summary.weekdays[1].count, 1);
    assert_eq!(summary.weekdays[3].count, 1);
    assert_eq!(summary.weekdays[6].count, 0);
}

#[test]
fn hashtags_come_back_ranked_and_capped_at_six() {
    let entries = [
        entry(28, Mood::Calm, &["#home", "#work"]),
        entry(29, Mood::Calm, &["#home"]),
        entry(30, Mood::Calm, &["#home", "#rain"]),
    ];

    let summary = all_time(&entries);

    assert_eq!(summary.tags[0].tag, "#home");
    assert_eq!(summary.tags[0].count, 3);
    assert_eq!(summary.tags.len(), 3);
}
