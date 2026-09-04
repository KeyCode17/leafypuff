package com.leafypuff.ui.stats

import com.leafypuff.core.Location
import com.leafypuff.core.Weather
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

data class WeatherCount(val weather: Weather, val count: Int)

data class PlaceCount(val location: Location, val count: Int)

data class StatsSummary(
    val daysWritten: Int = 0,
    val longestStreak: Int = 0,
    val moodSpread: List<MoodCount> = emptyList(),
    val moodBalance: List<GroupCount> = MoodGroup.entries.map { GroupCount(it, 0) },
    val weekdays: List<WeekdayCount> = WeekdayLabels.map { WeekdayCount(it, 0) },
    val tags: List<TagCount> = emptyList(),
    val weather: List<WeatherCount> = emptyList(),
    val places: List<PlaceCount> = emptyList(),
) {
    val spreadMax: Int get() = maxOf(1, moodSpread.maxOfOrNull { it.count } ?: 0)

    val balanceTotal: Int get() = moodBalance.sumOf { it.count }

    val weatherMax: Int get() = maxOf(1, weather.maxOfOrNull { it.count } ?: 0)

    val placeMax: Int get() = maxOf(1, places.maxOfOrNull { it.count } ?: 0)
}

val WeekdayLabels = listOf("Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat")
