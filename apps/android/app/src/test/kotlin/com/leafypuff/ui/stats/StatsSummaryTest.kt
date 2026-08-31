package com.leafypuff.ui.stats

import com.leafypuff.domain.Entry
import com.leafypuff.domain.Mood
import com.leafypuff.domain.MoodGroup
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue
import kotlinx.datetime.LocalDate

private val Today = LocalDate(2026, 9, 1)

private fun entry(
    day: Int,
    mood: Mood = Mood.Calm,
    tags: List<String> = emptyList(),
    id: String = "e$day-${mood.name}-${tags.size}",
): Entry = Entry(
    id = id,
    date = LocalDate(2026, 8, day),
    mood = mood,
    title = "t$day",
    body = "b$day",
    tags = tags,
)

class StatsSummaryTest {

    @Test
    fun `a gap of one day ends the streak`() {
        val entries = listOf(entry(26), entry(27), entry(28), entry(30), entry(31))

        val summary = statsSummary(entries, StatRange.AllTime, Today)

        assertEquals(3, summary.longestStreak)
        assertEquals(5, summary.daysWritten)
    }

    @Test
    fun `two entries on one day count as one day of the streak`() {
        val entries = listOf(
            entry(30, id = "morning"),
            entry(30, mood = Mood.Excited, id = "evening"),
            entry(31),
        )

        val summary = statsSummary(entries, StatRange.AllTime, Today)

        assertEquals(2, summary.longestStreak)
        assertEquals(2, summary.daysWritten)
    }

    @Test
    fun `a single day is a streak of one`() {
        val summary = statsSummary(listOf(entry(31)), StatRange.AllTime, Today)

        assertEquals(1, summary.longestStreak)
        assertEquals(1, summary.daysWritten)
    }

    @Test
    fun `an empty list yields zeroes and every bucket still present`() {
        val summary = statsSummary(emptyList(), StatRange.AllTime, Today)

        assertEquals(0, summary.daysWritten)
        assertEquals(0, summary.longestStreak)
        assertEquals(0, summary.balanceTotal)
        assertTrue(summary.moodSpread.isEmpty())
        assertTrue(summary.tags.isEmpty())
        assertEquals(listOf(0, 0, 0), summary.moodBalance.map { it.count })
        assertEquals(MoodGroup.entries.toList(), summary.moodBalance.map { it.group })
        assertEquals(7, summary.weekdays.size)
        assertTrue(summary.weekdays.all { it.count == 0 })
    }

    @Test
    fun `moods fall into the three handoff groups`() {
        val entries = listOf(
            entry(24, Mood.Happy),
            entry(25, Mood.Calm),
            entry(26, Mood.Okay),
            entry(27, Mood.Sad),
            entry(28, Mood.Angry),
        )

        val summary = statsSummary(entries, StatRange.AllTime, Today)

        assertEquals(listOf(2, 1, 2), summary.moodBalance.map { it.count })
        assertEquals(5, summary.balanceTotal)
    }

    @Test
    fun `the seven day range keeps day six and drops day seven`() {
        val entries = listOf(entry(26), entry(25))

        val summary = statsSummary(entries, StatRange.SevenDays, Today)

        assertEquals(1, summary.daysWritten)
        assertEquals(listOf(entry(26).date), entriesInRange(entries, StatRange.SevenDays, Today).map { it.date })
    }

    @Test
    fun `the spread keeps at most six moods ordered by count`() {
        val entries = listOf(
            entry(20, Mood.Happy), entry(21, Mood.Happy), entry(22, Mood.Happy),
            entry(23, Mood.Calm), entry(24, Mood.Calm),
            entry(25, Mood.Okay), entry(26, Mood.Sad), entry(27, Mood.Angry),
            entry(28, Mood.Tired), entry(29, Mood.Loved),
        )

        val summary = statsSummary(entries, StatRange.AllTime, Today)

        assertEquals(SpreadLimit, summary.moodSpread.size)
        assertEquals(Mood.Happy, summary.moodSpread.first().mood)
        assertEquals(3, summary.spreadMax)
        assertEquals(summary.moodSpread.map { it.count }.sortedDescending(), summary.moodSpread.map { it.count })
    }

    @Test
    fun `weekday buckets are sunday first`() {
        val summary = statsSummary(listOf(entry(30), entry(31), entry(26)), StatRange.AllTime, Today)

        assertEquals(1, summary.weekdays[0].count)
        assertEquals(1, summary.weekdays[1].count)
        assertEquals(1, summary.weekdays[3].count)
        assertEquals(0, summary.weekdays[6].count)
    }

    @Test
    fun `hashtags come back ranked and capped at six`() {
        val entries = listOf(
            entry(28, tags = listOf("#home", "#work")),
            entry(29, tags = listOf("#home")),
            entry(30, tags = listOf("#home", "#rain")),
        )

        val summary = statsSummary(entries, StatRange.AllTime, Today)

        assertEquals(TagCount("#home", 3), summary.tags.first())
        assertEquals(3, summary.tags.size)
    }
}
