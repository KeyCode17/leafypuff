package com.leafypuff.ui.calendar

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.unit.dp
import com.leafypuff.domain.Entry
import kotlinx.datetime.DateTimeUnit
import kotlinx.datetime.LocalDate
import kotlinx.datetime.daysUntil
import kotlinx.datetime.plus

private val GridGap = 6.dp
private const val DaysInWeek = 7

data class CalendarDay(
    val date: LocalDate,
    val entries: List<Entry>,
    val isToday: Boolean,
    val isSelected: Boolean,
    val cover: ImageBitmap?,
) {
    val showsPhoto: Boolean get() = cover != null && !isSelected

    val showsDot: Boolean get() = entries.isNotEmpty() && !isSelected && !showsPhoto

    val showsCount: Boolean get() = entries.size > 1

    val dotColor: Color
        get() = entries.firstOrNull()?.let { Color(it.mood.dotArgb) } ?: Color.Transparent
}

@Composable
fun CalendarGrid(
    visibleMonth: LocalDate,
    entriesByDate: Map<LocalDate, List<Entry>>,
    today: LocalDate,
    selected: LocalDate,
    covers: Map<String, ImageBitmap>,
    onSelect: (LocalDate) -> Unit,
    modifier: Modifier = Modifier,
) {
    val weeks = monthCells(visibleMonth, entriesByDate, today, selected, covers)
        .chunked(DaysInWeek)

    Column(
        modifier = modifier.fillMaxWidth(),
        verticalArrangement = Arrangement.spacedBy(GridGap),
    ) {
        weeks.forEach { week ->
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(GridGap),
            ) {
                week.forEach { day ->
                    if (day == null) {
                        Spacer(modifier = Modifier.weight(1f).aspectRatio(1f))
                    } else {
                        CalendarDayCell(
                            day = day,
                            onSelect = onSelect,
                            modifier = Modifier.weight(1f),
                        )
                    }
                }
            }
        }
    }
}

private fun monthCells(
    visibleMonth: LocalDate,
    entriesByDate: Map<LocalDate, List<Entry>>,
    today: LocalDate,
    selected: LocalDate,
    covers: Map<String, ImageBitmap>,
): List<CalendarDay?> {
    val first = LocalDate(visibleMonth.year, visibleMonth.monthNumber, 1)
    val lead = (first.dayOfWeek.ordinal + 1) % DaysInWeek
    val length = first.daysUntil(first.plus(1, DateTimeUnit.MONTH))
    val cells = ArrayList<CalendarDay?>(lead + length)

    repeat(lead) { cells.add(null) }
    for (dayOfMonth in 1..length) {
        val date = LocalDate(first.year, first.monthNumber, dayOfMonth)
        val dayEntries = entriesByDate[date].orEmpty()
        cells.add(
            CalendarDay(
                date = date,
                entries = dayEntries,
                isToday = date == today,
                isSelected = date == selected,
                cover = dayEntries.firstNotNullOfOrNull { covers[it.id] },
            ),
        )
    }
    while (cells.size % DaysInWeek != 0) cells.add(null)

    return cells
}
