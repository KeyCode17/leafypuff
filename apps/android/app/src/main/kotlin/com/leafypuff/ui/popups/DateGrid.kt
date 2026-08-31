package com.leafypuff.ui.popups

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import kotlinx.datetime.DateTimeUnit
import kotlinx.datetime.LocalDate
import kotlinx.datetime.daysUntil
import kotlinx.datetime.plus

private val GridGap = 2.dp
private val WeekdayLabelSize = 9.sp
private val WeekdayBottomPadding = 4.dp
private val NumeralSize = 13.sp
private val SelectedInset = 1.dp
private const val DaysInWeek = 7
private val Weekdays = listOf("S", "M", "T", "W", "T", "F", "S")

@Composable
internal fun DateWeekdayRow() {
    val colors = LocalLeafyColors.current
    val style = LocalLeafyTypography.current.metaLabel.copy(fontSize = WeekdayLabelSize)

    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(bottom = WeekdayBottomPadding),
        horizontalArrangement = Arrangement.spacedBy(GridGap),
    ) {
        Weekdays.forEach { label ->
            Text(
                text = label,
                style = style,
                color = colors.ink3,
                textAlign = TextAlign.Center,
                modifier = Modifier.weight(1f),
            )
        }
    }
}

@Composable
internal fun DateGrid(
    visibleMonth: LocalDate,
    selected: LocalDate,
    onSelect: (LocalDate) -> Unit,
) {
    val weeks = monthCells(visibleMonth).chunked(DaysInWeek)

    Column(
        modifier = Modifier.fillMaxWidth(),
        verticalArrangement = Arrangement.spacedBy(GridGap),
    ) {
        weeks.forEach { week ->
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(GridGap),
            ) {
                week.forEach { date ->
                    if (date == null) {
                        Spacer(modifier = Modifier.weight(1f).aspectRatio(1f))
                    } else {
                        DateCell(
                            date = date,
                            isSelected = date == selected,
                            onSelect = onSelect,
                            modifier = Modifier.weight(1f),
                        )
                    }
                }
            }
        }
    }
}

@Composable
private fun DateCell(
    date: LocalDate,
    isSelected: Boolean,
    onSelect: (LocalDate) -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current

    Box(
        modifier = modifier
            .aspectRatio(1f)
            .clip(LeafyShapes.pill)
            .clickable { onSelect(date) },
        contentAlignment = Alignment.Center,
    ) {
        if (isSelected) {
            Box(
                modifier = Modifier
                    .matchParentSize()
                    .padding(SelectedInset)
                    .clip(LeafyShapes.pill)
                    .background(colors.accent),
            )
        }
        Text(
            text = date.dayOfMonth.toString(),
            style = LocalLeafyTypography.current.monthLabel.copy(
                fontSize = NumeralSize,
                fontWeight = if (isSelected) FontWeight.W600 else FontWeight.W400,
            ),
            color = if (isSelected) colors.onAccent else colors.ink,
        )
    }
}

private fun monthCells(visibleMonth: LocalDate): List<LocalDate?> {
    val first = LocalDate(visibleMonth.year, visibleMonth.monthNumber, 1)
    val lead = (first.dayOfWeek.ordinal + 1) % DaysInWeek
    val length = first.daysUntil(first.plus(1, DateTimeUnit.MONTH))
    val cells = ArrayList<LocalDate?>(lead + length)

    repeat(lead) { cells.add(null) }
    for (dayOfMonth in 1..length) {
        cells.add(LocalDate(first.year, first.monthNumber, dayOfMonth))
    }
    while (cells.size % DaysInWeek != 0) cells.add(null)

    return cells
}
