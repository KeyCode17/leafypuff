package com.leafypuff.ui.calendar

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxScope
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.drawBehind
import androidx.compose.ui.geometry.Rect
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.PathEffect
import androidx.compose.ui.graphics.drawscope.DrawScope
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import kotlinx.datetime.LocalDate

private val NumeralSize = 15.sp
private val PhotoInset = 1.dp
private val SelectedInset = 2.dp
private val TodayRingInset = 3.dp
private val TodayRingWidth = 1.5.dp
private val TodayDashOn = 4.dp
private val TodayDashOff = 3.dp
private val MoodDotSize = 6.dp
private val MoodDotBottom = 2.dp
private val CountBadgeSize = 13.dp
private val CountNumeralSize = 8.sp
private val PhotoScrim = Color(0x57242D35)
private val PhotoNumeral = Color(0xFFFFFFFF)

@Composable
fun CalendarDayCell(
    day: CalendarDay,
    onSelect: (LocalDate) -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current
    val accent = colors.accent

    Box(
        modifier = modifier
            .aspectRatio(1f)
            .clickable { onSelect(day.date) }
            .drawBehind { if (day.isToday) drawTodayRing(accent) },
        contentAlignment = Alignment.Center,
    ) {
        if (day.showsPhoto) {
            Box(
                modifier = Modifier
                    .matchParentSize()
                    .padding(PhotoInset)
                    .clip(CircleShape)
                    .background(colors.soft2)
                    .background(PhotoScrim),
            )
        }
        if (day.isSelected) {
            Box(
                modifier = Modifier
                    .matchParentSize()
                    .padding(SelectedInset)
                    .clip(CircleShape)
                    .background(accent),
            )
        }
        DayNumeral(day)
        if (day.showsDot) MoodDot(day.dotColor)
        if (day.showsCount) EntryCount(day.entries.size, day.isSelected)
    }
}

@Composable
private fun DayNumeral(day: CalendarDay) {
    val colors = LocalLeafyColors.current
    val strong = day.isSelected || day.showsPhoto
    val color = when {
        day.isSelected -> colors.onAccent
        day.showsPhoto -> PhotoNumeral
        day.entries.isNotEmpty() -> colors.ink
        else -> colors.ink3
    }

    Text(
        text = day.date.dayOfMonth.toString(),
        style = LocalLeafyTypography.current.monthLabel.copy(
            fontSize = NumeralSize,
            fontWeight = if (strong) FontWeight.W600 else FontWeight.W400,
        ),
        color = color,
    )
}

@Composable
private fun BoxScope.MoodDot(color: Color) {
    Box(
        modifier = Modifier
            .align(Alignment.BottomCenter)
            .padding(bottom = MoodDotBottom)
            .size(MoodDotSize)
            .clip(CircleShape)
            .background(color),
    )
}

@Composable
private fun BoxScope.EntryCount(count: Int, onAccentCell: Boolean) {
    val colors = LocalLeafyColors.current
    val fill = when {
        onAccentCell -> colors.sheet
        else -> colors.accent
    }
    val ink = when {
        onAccentCell -> colors.accentDeep
        else -> colors.onAccent
    }

    Box(
        modifier = Modifier
            .align(Alignment.TopEnd)
            .size(CountBadgeSize)
            .clip(CircleShape)
            .background(fill),
        contentAlignment = Alignment.Center,
    ) {
        Text(
            text = count.toString(),
            style = LocalLeafyTypography.current.metaLabel.copy(
                fontSize = CountNumeralSize,
                fontWeight = FontWeight.W600,
            ),
            color = ink,
        )
    }
}

private fun DrawScope.drawTodayRing(color: Color) {
    val stroke = TodayRingWidth.toPx()
    val radius = size.minDimension / 2f + TodayRingInset.toPx() - stroke / 2f
    val ring = Path().apply { addOval(Rect(center, radius)) }
    drawPath(
        path = ring,
        color = color,
        style = Stroke(
            width = stroke,
            pathEffect = PathEffect.dashPathEffect(
                floatArrayOf(TodayDashOn.toPx(), TodayDashOff.toPx()),
            ),
        ),
    )
}
