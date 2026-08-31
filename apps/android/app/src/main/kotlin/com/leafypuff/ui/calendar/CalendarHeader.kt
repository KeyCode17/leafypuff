package com.leafypuff.ui.calendar

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.widthIn
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.KeyboardArrowLeft
import androidx.compose.material.icons.filled.KeyboardArrowRight
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.common.formatMonthYear
import kotlinx.datetime.DateTimeUnit
import kotlinx.datetime.LocalDate
import kotlinx.datetime.minus
import kotlinx.datetime.plus

private val HeaderPadding = 20.dp
private val NavGap = 10.dp
private val NavButtonSize = 32.dp
private val NavGlyphSize = 16.dp
private val MonthLabelMinWidth = 132.dp
private val Hairline = 0.5.dp
private val PillPaddingH = 14.dp
private val PillPaddingV = 8.dp
private val TodayLabelSize = 12.sp
private val WeekdayLabelSize = 10.sp
private val WeekdayGap = 6.dp
private val WeekdayBottomPadding = 8.dp
private val Weekdays = listOf("S", "M", "T", "W", "T", "F", "S")

@Composable
fun CalendarHeader(
    visibleMonth: LocalDate,
    onMonthChange: (LocalDate) -> Unit,
    onToday: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current

    Row(
        modifier = modifier
            .fillMaxWidth()
            .padding(vertical = HeaderPadding),
        horizontalArrangement = Arrangement.SpaceBetween,
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Row(
            horizontalArrangement = Arrangement.spacedBy(NavGap),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            NavButton(
                glyph = Icons.Filled.KeyboardArrowLeft,
                label = "Previous month",
                onClick = { onMonthChange(visibleMonth.minus(1, DateTimeUnit.MONTH)) },
            )
            Text(
                text = formatMonthYear(visibleMonth),
                style = LocalLeafyTypography.current.monthLabel,
                color = colors.ink,
                modifier = Modifier.widthIn(min = MonthLabelMinWidth),
            )
            NavButton(
                glyph = Icons.Filled.KeyboardArrowRight,
                label = "Next month",
                onClick = { onMonthChange(visibleMonth.plus(1, DateTimeUnit.MONTH)) },
            )
        }
        TodayPill(onToday)
    }
}

@Composable
fun WeekdayRow(modifier: Modifier = Modifier) {
    val colors = LocalLeafyColors.current
    val style = LocalLeafyTypography.current.metaLabel.copy(fontSize = WeekdayLabelSize)

    Row(
        modifier = modifier
            .fillMaxWidth()
            .padding(bottom = WeekdayBottomPadding),
        horizontalArrangement = Arrangement.spacedBy(WeekdayGap),
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
private fun NavButton(glyph: ImageVector, label: String, onClick: () -> Unit) {
    val colors = LocalLeafyColors.current

    Box(
        modifier = Modifier
            .size(NavButtonSize)
            .clip(LeafyShapes.pill)
            .background(colors.surface)
            .border(Hairline, colors.line, LeafyShapes.pill)
            .clickable(onClick = onClick),
        contentAlignment = Alignment.Center,
    ) {
        Icon(
            imageVector = glyph,
            contentDescription = label,
            tint = colors.ink,
            modifier = Modifier.size(NavGlyphSize),
        )
    }
}

@Composable
private fun TodayPill(onToday: () -> Unit) {
    val colors = LocalLeafyColors.current

    Text(
        text = "Today".uppercase(),
        style = LocalLeafyTypography.current.buttonLabel.copy(
            fontSize = TodayLabelSize,
            fontWeight = FontWeight.W500,
        ),
        color = colors.accentDeep,
        modifier = Modifier
            .clip(LeafyShapes.pill)
            .background(colors.soft2)
            .clickable(onClick = onToday)
            .padding(horizontal = PillPaddingH, vertical = PillPaddingV),
    )
}
