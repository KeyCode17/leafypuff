use crate::domain::{
    GroupCount, MoodCount, MoodGroup, StatsRange, StatsSummary, TagCount, WeekdayCount,
};

#[derive(uniffi::Enum)]
pub enum FfiStatsRange {
    SevenDays,
    ThirtyDays,
    AllTime,
}

impl From<FfiStatsRange> for StatsRange {
    fn from(range: FfiStatsRange) -> Self {
        match range {
            FfiStatsRange::SevenDays => Self::SevenDays,
            FfiStatsRange::ThirtyDays => Self::ThirtyDays,
            FfiStatsRange::AllTime => Self::AllTime,
        }
    }
}

#[derive(uniffi::Enum)]
pub enum FfiMoodGroup {
    Positive,
    Neutral,
    Negative,
}

impl From<MoodGroup> for FfiMoodGroup {
    fn from(group: MoodGroup) -> Self {
        match group {
            MoodGroup::Positive => Self::Positive,
            MoodGroup::Neutral => Self::Neutral,
            MoodGroup::Negative => Self::Negative,
        }
    }
}

#[derive(uniffi::Record)]
pub struct FfiMoodCount {
    pub mood: crate::domain::Mood,
    pub count: u32,
}

#[derive(uniffi::Record)]
pub struct FfiGroupCount {
    pub group: FfiMoodGroup,
    pub count: u32,
}

#[derive(uniffi::Record)]
pub struct FfiWeekdayCount {
    pub label: String,
    pub count: u32,
}

#[derive(uniffi::Record)]
pub struct FfiTagCount {
    pub tag: String,
    pub count: u32,
}

#[derive(uniffi::Record)]
pub struct FfiStats {
    pub days_written: u32,
    pub longest_streak: u32,
    pub mood_spread: Vec<FfiMoodCount>,
    pub mood_balance: Vec<FfiGroupCount>,
    pub weekdays: Vec<FfiWeekdayCount>,
    pub tags: Vec<FfiTagCount>,
}

impl From<StatsSummary> for FfiStats {
    fn from(summary: StatsSummary) -> Self {
        Self {
            days_written: summary.days_written,
            longest_streak: summary.longest_streak,
            mood_spread: summary
                .mood_spread
                .into_iter()
                .map(FfiMoodCount::from)
                .collect(),
            mood_balance: summary
                .mood_balance
                .into_iter()
                .map(FfiGroupCount::from)
                .collect(),
            weekdays: summary
                .weekdays
                .into_iter()
                .map(FfiWeekdayCount::from)
                .collect(),
            tags: summary.tags.into_iter().map(FfiTagCount::from).collect(),
        }
    }
}

impl From<MoodCount> for FfiMoodCount {
    fn from(row: MoodCount) -> Self {
        Self {
            mood: row.mood,
            count: row.count,
        }
    }
}

impl From<GroupCount> for FfiGroupCount {
    fn from(row: GroupCount) -> Self {
        Self {
            group: FfiMoodGroup::from(row.group),
            count: row.count,
        }
    }
}

impl From<WeekdayCount> for FfiWeekdayCount {
    fn from(row: WeekdayCount) -> Self {
        Self {
            label: row.label.to_owned(),
            count: row.count,
        }
    }
}

impl From<TagCount> for FfiTagCount {
    fn from(row: TagCount) -> Self {
        Self {
            tag: row.tag,
            count: row.count,
        }
    }
}
