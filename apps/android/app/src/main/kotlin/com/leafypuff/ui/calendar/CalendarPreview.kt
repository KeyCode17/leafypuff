package com.leafypuff.ui.calendar

import androidx.compose.runtime.Composable
import androidx.compose.ui.tooling.preview.Preview
import com.leafypuff.data.SampleEntries
import com.leafypuff.domain.Entry
import com.leafypuff.domain.Mood
import com.leafypuff.theme.LeafyTheme
import kotlinx.datetime.LocalDate

private const val FrameWidth = 375
private const val FrameHeight = 812
private val PreviewMonth = LocalDate(2026, 8, 1)
private val PreviewSelected = LocalDate(2026, 8, 31)

private val PreviewEntries: List<Entry> = SampleEntries + Entry(
    id = "preview-second",
    date = PreviewSelected,
    mood = Mood.Excited,
    title = "Late night idea",
    body = "Second entry on the same day, so the block below the grid has to hold more than one card.",
    tags = listOf("#night"),
)

private val PhotoDayEntries: List<Entry> = PreviewEntries.map { entry ->
    when (entry.date) {
        PreviewSelected -> entry.copy(coverPhotoId = "preview-cover")
        else -> entry
    }
}

@Preview(name = "Calendar on light", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun CalendarLightPreview() {
    CalendarFrame(dark = false)
}

@Preview(name = "Calendar on dark", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun CalendarDarkPreview() {
    CalendarFrame(dark = true)
}

@Preview(name = "Calendar with two entries on one day", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun CalendarBusyDayPreview() {
    CalendarFrame(dark = false, selected = LocalDate(2026, 8, 30))
}

@Preview(name = "Calendar with a photo day", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun CalendarPhotoDayPreview() {
    CalendarFrame(dark = false, entries = PhotoDayEntries)
}

@Composable
private fun CalendarFrame(
    dark: Boolean,
    entries: List<Entry> = PreviewEntries,
    selected: LocalDate = PreviewSelected,
) {
    LeafyTheme(darkOverride = dark) {
        CalendarScreen(
            entries = entries,
            visibleMonth = PreviewMonth,
            selected = selected,
            onMonthChange = { },
            onSelect = { },
            onToday = { },
            onCreateForSelected = { },
        )
    }
}
