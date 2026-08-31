package com.leafypuff.ui.stats

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.size
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

private val DonutGap = 18.dp
private val LegendGap = 12.dp
private val LegendRowGap = 8.dp
private val LegendDotSize = 10.dp
private val LegendTextSize = 13.sp

@Composable
fun MoodBalanceCard(summary: StatsSummary, modifier: Modifier = Modifier) {
    StatsCard(modifier = modifier) {
        StatsCardLabel("Mood balance")
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(DonutGap),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            MoodDonut(slices = summary.moodBalance, total = summary.balanceTotal)
            Column(
                modifier = Modifier.weight(1f),
                verticalArrangement = Arrangement.spacedBy(LegendGap),
            ) {
                summary.moodBalance.forEach { slice -> LegendRow(slice) }
            }
        }
    }
}

@Composable
private fun LegendRow(slice: GroupCount) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.spacedBy(LegendRowGap),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Box(
            modifier = Modifier
                .size(LegendDotSize)
                .clip(LeafyShapes.pill)
                .background(groupColor(slice.group)),
        )
        Text(
            text = groupLabel(slice.group),
            style = typography.chipLabel.copy(
                fontSize = LegendTextSize,
                fontWeight = FontWeight.W400,
            ),
            color = colors.ink,
            modifier = Modifier.weight(1f),
        )
        Text(
            text = slice.count.toString(),
            style = typography.monthLabel.copy(
                fontSize = LegendTextSize,
                fontWeight = FontWeight.W500,
            ),
            color = colors.ink2,
        )
    }
}
