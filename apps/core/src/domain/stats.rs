use std::cmp::Reverse;

use chrono::{Datelike, NaiveDate};

use super::entry::Entry;
use super::mood::{Mood, MoodGroup};
use super::weather::{Location, Weather};

pub const SPREAD_LIMIT: usize = 6;
pub const TAG_LIMIT: usize = 6;
pub const WEEKDAY_LABELS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

const SEVEN_DAYS: i64 = 7;
const THIRTY_DAYS: i64 = 30;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatsRange {
    SevenDays,
    ThirtyDays,
    AllTime,
}

impl StatsRange {
    const fn span_days(self) -> Option<i64> {
        match self {
            Self::SevenDays => Some(SEVEN_DAYS),
            Self::ThirtyDays => Some(THIRTY_DAYS),
            Self::AllTime => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoodCount {
    pub mood: Mood,
    pub count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GroupCount {
    pub group: MoodGroup,
    pub count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeekdayCount {
    pub label: &'static str,
    pub count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeatherCount {
    pub weather: Weather,
    pub count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlaceCount {
    pub location: Location,
    pub count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagCount {
    pub tag: String,
    pub count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatsSummary {
    pub days_written: u32,
    pub longest_streak: u32,
    pub mood_spread: Vec<MoodCount>,
    pub mood_balance: Vec<GroupCount>,
    pub weekdays: Vec<WeekdayCount>,
    pub tags: Vec<TagCount>,
    pub weather: Vec<WeatherCount>,
    pub places: Vec<PlaceCount>,
}

pub fn summarise(entries: &[Entry], range: StatsRange, today: NaiveDate) -> StatsSummary {
    let in_range: Vec<&Entry> = entries
        .iter()
        .filter(|entry| within(entry.date, range, today))
        .collect();

    let mut written_days: Vec<i32> = in_range
        .iter()
        .map(|entry| entry.date.num_days_from_ce())
        .collect();
    written_days.sort_unstable();
    written_days.dedup();

    StatsSummary {
        days_written: written_days.len() as u32,
        longest_streak: longest_streak(&written_days),
        mood_spread: mood_spread(&in_range),
        mood_balance: mood_balance(&in_range),
        weekdays: weekday_counts(&in_range),
        tags: top_tags(&in_range),
        weather: weather_counts(&in_range),
        places: place_counts(&in_range),
    }
}

fn within(date: NaiveDate, range: StatsRange, today: NaiveDate) -> bool {
    let elapsed = (today - date).num_days();
    match range.span_days() {
        Some(span) => (0..span).contains(&elapsed),
        None => elapsed >= 0,
    }
}

fn longest_streak(sorted_days: &[i32]) -> u32 {
    let mut longest = 0;
    let mut run = 0;
    let mut previous: Option<i32> = None;
    for day in sorted_days {
        run = match previous {
            Some(before) if *day == before + 1 => run + 1,
            _ => 1,
        };
        longest = longest.max(run);
        previous = Some(*day);
    }
    longest
}

fn counted(entries: &[&Entry], mood: Mood) -> u32 {
    entries.iter().filter(|entry| entry.mood == mood).count() as u32
}

fn mood_spread(entries: &[&Entry]) -> Vec<MoodCount> {
    let mut counts: Vec<MoodCount> = Mood::ALL
        .into_iter()
        .map(|mood| MoodCount {
            mood,
            count: counted(entries, mood),
        })
        .filter(|row| row.count > 0)
        .collect();
    counts.sort_by_key(|row| Reverse(row.count));
    counts.truncate(SPREAD_LIMIT);
    counts
}

fn mood_balance(entries: &[&Entry]) -> Vec<GroupCount> {
    MoodGroup::ALL
        .into_iter()
        .map(|group| GroupCount {
            group,
            count: entries
                .iter()
                .filter(|entry| entry.mood.group() == group)
                .count() as u32,
        })
        .collect()
}

fn weekday_counts(entries: &[&Entry]) -> Vec<WeekdayCount> {
    let mut counts = [0_u32; WEEKDAY_LABELS.len()];
    for entry in entries {
        counts[entry.date.weekday().num_days_from_sunday() as usize] += 1;
    }
    WEEKDAY_LABELS
        .into_iter()
        .zip(counts)
        .map(|(label, count)| WeekdayCount { label, count })
        .collect()
}

fn weather_counts(entries: &[&Entry]) -> Vec<WeatherCount> {
    let mut counts: Vec<WeatherCount> = Weather::ALL
        .into_iter()
        .map(|weather| WeatherCount {
            weather,
            count: entries
                .iter()
                .filter(|entry| entry.weather == Some(weather))
                .count() as u32,
        })
        .filter(|row| row.count > 0)
        .collect();
    counts.sort_by_key(|row| Reverse(row.count));
    counts
}

fn place_counts(entries: &[&Entry]) -> Vec<PlaceCount> {
    let mut counts: Vec<PlaceCount> = Location::ALL
        .into_iter()
        .map(|location| PlaceCount {
            location,
            count: entries
                .iter()
                .filter(|entry| entry.location == Some(location))
                .count() as u32,
        })
        .filter(|row| row.count > 0)
        .collect();
    counts.sort_by_key(|row| Reverse(row.count));
    counts
}

fn top_tags(entries: &[&Entry]) -> Vec<TagCount> {
    let mut counts: Vec<TagCount> = Vec::new();
    for tag in entries.iter().flat_map(|entry| entry.tags.iter()) {
        match counts.iter_mut().find(|row| &row.tag == tag) {
            Some(row) => row.count += 1,
            None => counts.push(TagCount {
                tag: tag.clone(),
                count: 1,
            }),
        }
    }
    counts.sort_by_key(|row| Reverse(row.count));
    counts.truncate(TAG_LIMIT);
    counts
}
