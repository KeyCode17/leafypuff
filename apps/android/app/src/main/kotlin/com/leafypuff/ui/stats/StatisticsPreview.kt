package com.leafypuff.ui.stats

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.leafypuff.core.Location
import com.leafypuff.core.Weather
import com.leafypuff.domain.Mood
import com.leafypuff.domain.MoodGroup
import com.leafypuff.theme.LeafyTheme
import com.leafypuff.theme.LocalLeafyColors

private const val FrameWidth = 375
private const val FrameHeight = 812

private val PreviewSummary = StatsSummary(
    daysWritten = 12,
    longestStreak = 4,
    moodSpread = listOf(
        MoodCount(Mood.Happy, 5),
        MoodCount(Mood.Calm, 3),
        MoodCount(Mood.Tired, 2),
        MoodCount(Mood.Sad, 1),
    ),
    moodBalance = listOf(
        GroupCount(MoodGroup.Positive, 8),
        GroupCount(MoodGroup.Neutral, 2),
        GroupCount(MoodGroup.Negative, 1),
    ),
    weekdays = WeekdayLabels.mapIndexed { index, label -> WeekdayCount(label, index % 3) },
    tags = listOf(TagCount("#home", 4), TagCount("#coffee", 2)),
    weather = listOf(
        WeatherCount(Weather.SUNNY, 6),
        WeatherCount(Weather.RAINY, 3),
        WeatherCount(Weather.CLOUDY, 1),
    ),
    places = listOf(
        PlaceCount(Location.HOME, 7),
        PlaceCount(Location.CAFE, 2),
    ),
)

@Preview(name = "Statistics on light", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun StatisticsLightPreview() {
    StatisticsFrame(dark = false, range = StatRange.AllTime, summary = PreviewSummary)
}

@Preview(name = "Statistics on dark", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun StatisticsDarkPreview() {
    StatisticsFrame(dark = true, range = StatRange.AllTime, summary = PreviewSummary)
}

@Preview(name = "Statistics with nothing in range", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun StatisticsEmptyPreview() {
    StatisticsFrame(dark = false, range = StatRange.SevenDays, summary = StatsSummary())
}

@Composable
private fun StatisticsFrame(dark: Boolean, range: StatRange, summary: StatsSummary) {
    LeafyTheme(darkOverride = dark) {
        Box(modifier = Modifier.fillMaxSize().background(LocalLeafyColors.current.bg)) {
            StatisticsScreen(summary = summary, range = range, onRangeChange = { })
        }
    }
}
