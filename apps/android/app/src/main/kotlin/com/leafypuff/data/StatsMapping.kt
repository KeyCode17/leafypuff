package com.leafypuff.data

import com.leafypuff.core.FfiGroupCount
import com.leafypuff.core.FfiMoodCount
import com.leafypuff.core.FfiMoodGroup
import com.leafypuff.core.FfiStats
import com.leafypuff.core.FfiStatsRange
import com.leafypuff.core.FfiTagCount
import com.leafypuff.core.FfiWeekdayCount
import com.leafypuff.domain.MoodGroup
import com.leafypuff.ui.stats.GroupCount
import com.leafypuff.ui.stats.MoodCount
import com.leafypuff.ui.stats.StatRange
import com.leafypuff.ui.stats.StatsSummary
import com.leafypuff.ui.stats.TagCount
import com.leafypuff.ui.stats.WeekdayCount

fun StatRange.toCore(): FfiStatsRange = when (this) {
    StatRange.SevenDays -> FfiStatsRange.SEVEN_DAYS
    StatRange.ThirtyDays -> FfiStatsRange.THIRTY_DAYS
    StatRange.AllTime -> FfiStatsRange.ALL_TIME
}

fun FfiStats.toSummary(): StatsSummary = StatsSummary(
    daysWritten = daysWritten.toInt(),
    longestStreak = longestStreak.toInt(),
    moodSpread = moodSpread.map { it.toUi() },
    moodBalance = moodBalance.map { it.toUi() },
    weekdays = weekdays.map { it.toUi() },
    tags = tags.map { it.toUi() },
)

private fun FfiMoodCount.toUi(): MoodCount = MoodCount(mood.toDomain(), count.toInt())

private fun FfiGroupCount.toUi(): GroupCount = GroupCount(group.toDomain(), count.toInt())

private fun FfiWeekdayCount.toUi(): WeekdayCount = WeekdayCount(label, count.toInt())

private fun FfiTagCount.toUi(): TagCount = TagCount(tag, count.toInt())

private fun FfiMoodGroup.toDomain(): MoodGroup = when (this) {
    FfiMoodGroup.POSITIVE -> MoodGroup.Positive
    FfiMoodGroup.NEUTRAL -> MoodGroup.Neutral
    FfiMoodGroup.NEGATIVE -> MoodGroup.Negative
}
