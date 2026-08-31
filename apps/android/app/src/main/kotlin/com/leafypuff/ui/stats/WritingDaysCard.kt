package com.leafypuff.ui.stats

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.size
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.em
import androidx.compose.ui.unit.sp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

private val ColumnGap = 4.dp
private val CircleLabelGap = 7.dp
private val CircleSize = 34.dp
private val CircleTextSize = 13.sp
private val WeekdayTextSize = 10.sp

@Composable
fun WritingDaysCard(summary: StatsSummary, modifier: Modifier = Modifier) {
    StatsCard(modifier = modifier) {
        StatsCardLabel("Writing days")
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(ColumnGap),
        ) {
            summary.weekdays.forEach { weekday -> WeekdayColumn(weekday) }
        }
    }
}

@Composable
private fun RowScope.WeekdayColumn(weekday: WeekdayCount) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current
    val written = weekday.count > 0

    Column(
        modifier = Modifier.weight(1f),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(CircleLabelGap),
    ) {
        Box(
            modifier = Modifier
                .size(CircleSize)
                .clip(LeafyShapes.pill)
                .background(if (written) colors.accent else colors.soft2),
            contentAlignment = Alignment.Center,
        ) {
            Text(
                text = weekday.count.toString(),
                style = typography.monthLabel.copy(
                    fontSize = CircleTextSize,
                    fontWeight = if (written) FontWeight.W600 else FontWeight.W500,
                ),
                color = if (written) colors.onAccent else colors.ink3,
            )
        }
        Text(
            text = weekday.label.uppercase(),
            style = typography.monthLabel.copy(
                fontSize = WeekdayTextSize,
                fontWeight = FontWeight.W500,
                letterSpacing = 0.04.em,
            ),
            color = colors.ink3,
        )
    }
}
