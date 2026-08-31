package com.leafypuff.ui.stats

import com.leafypuff.domain.Entry
import com.leafypuff.domain.Mood
import com.leafypuff.domain.MoodGroup
import kotlinx.datetime.LocalDate

enum class StatRange(val spanDays: Int) {
    SevenDays(7),
    ThirtyDays(30),
    AllTime(Int.MAX_VALUE),
}

data class MoodCount(val mood: Mood, val count: Int)

data class GroupCount(val group: MoodGroup, val count: Int)

data class WeekdayCount(val label: String, val count: Int)

data class TagCount(val tag: String, val count: Int)

data class StatsSummary(
    val daysWritten: Int,
    val longestStreak: Int,
    val moodSpread: List<MoodCount>,
    val moodBalance: List<GroupCount>,
    val weekdays: List<WeekdayCount>,
    val tags: List<TagCount>,
) {
    val spreadMax: Int get() = maxOf(1, moodSpread.maxOfOrNull { it.count } ?: 0)

    val balanceTotal: Int get() = moodBalance.sumOf { it.count }
}

const val SpreadLimit = 6
const val TagLimit = 6

val WeekdayLabels = listOf("Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat")

fun entriesInRange(entries: List<Entry>, range: StatRange, today: LocalDate): List<Entry> {
    val end = epochDay(today)
    return entries.filter { entry ->
        val diff = end - epochDay(entry.date)
        diff >= 0L && (range == StatRange.AllTime || diff < range.spanDays.toLong())
    }
}

fun statsSummary(entries: List<Entry>, range: StatRange, today: LocalDate): StatsSummary {
    val inRange = entriesInRange(entries, range, today)
    val writtenDays = inRange.map { epochDay(it.date) }.distinct().sorted()
    val moodCounts = inRange.groupingBy { it.mood }.eachCount()
    val tagCounts = inRange.flatMap { it.tags }.groupingBy { it }.eachCount()

    return StatsSummary(
        daysWritten = writtenDays.size,
        longestStreak = longestStreak(writtenDays),
        moodSpread = moodSpread(moodCounts),
        moodBalance = moodBalance(moodCounts),
        weekdays = weekdayCounts(inRange),
        tags = topTags(tagCounts),
    )
}

fun longestStreak(sortedDays: List<Long>): Int {
    var longest = 0
    var run = 0
    var previous: Long? = null
    for (day in sortedDays) {
        run = if (previous != null && day == previous + 1L) run + 1 else 1
        if (run > longest) longest = run
        previous = day
    }
    return longest
}

fun moodSpread(moodCounts: Map<Mood, Int>): List<MoodCount> = Mood.entries
    .mapNotNull { mood -> moodCounts[mood]?.let { MoodCount(mood, it) } }
    .sortedByDescending { it.count }
    .take(SpreadLimit)

fun moodBalance(moodCounts: Map<Mood, Int>): List<GroupCount> = MoodGroup.entries.map { group ->
    GroupCount(group, moodCounts.entries.filter { it.key.group == group }.sumOf { it.value })
}

fun weekdayCounts(entries: List<Entry>): List<WeekdayCount> {
    val counts = IntArray(WeekdayLabels.size)
    entries.forEach { counts[sundayIndex(it.date)] += 1 }
    return WeekdayLabels.mapIndexed { index, label -> WeekdayCount(label, counts[index]) }
}

fun topTags(tagCounts: Map<String, Int>): List<TagCount> = tagCounts.entries
    .map { TagCount(it.key, it.value) }
    .sortedByDescending { it.count }
    .take(TagLimit)

fun sundayIndex(date: LocalDate): Int = (date.dayOfWeek.ordinal + 1) % WeekdayLabels.size

private fun epochDay(date: LocalDate): Long = date.toEpochDays().toLong()
