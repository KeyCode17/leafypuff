package com.leafypuff.ui.stats

import com.leafypuff.domain.Mood
import com.leafypuff.domain.MoodGroup

enum class StatRange {
    SevenDays,
    ThirtyDays,
    AllTime,
}

data class MoodCount(val mood: Mood, val count: Int)

data class GroupCount(val group: MoodGroup, val count: Int)

data class WeekdayCount(val label: String, val count: Int)

data class TagCount(val tag: String, val count: Int)

/**
 * What the core computed. Nothing here recomputes it: the streak rule walks calendar days, and one
 * definition of that living in two languages is one definition too many.
 */
data class StatsSummary(
    val daysWritten: Int = 0,
    val longestStreak: Int = 0,
    val moodSpread: List<MoodCount> = emptyList(),
    val moodBalance: List<GroupCount> = MoodGroup.entries.map { GroupCount(it, 0) },
    val weekdays: List<WeekdayCount> = WeekdayLabels.map { WeekdayCount(it, 0) },
    val tags: List<TagCount> = emptyList(),
) {
    val spreadMax: Int get() = maxOf(1, moodSpread.maxOfOrNull { it.count } ?: 0)

    val balanceTotal: Int get() = moodBalance.sumOf { it.count }
}

val WeekdayLabels = listOf("Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat")
