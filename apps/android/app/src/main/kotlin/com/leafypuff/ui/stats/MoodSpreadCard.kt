package com.leafypuff.ui.stats

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
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
import com.leafypuff.ui.common.BunnyFace

private val RowGap = 10.dp
private val FaceSize = 28.dp
private val LabelWidth = 62.dp
private val BarHeight = 8.dp
private val CountWidth = 18.dp
private val CountSize = 12.sp

@Composable
fun MoodSpreadCard(summary: StatsSummary, modifier: Modifier = Modifier) {
    StatsCard(modifier = modifier) {
        StatsCardLabel("Mood spread")
        summary.moodSpread.forEach { slice ->
            SpreadRow(slice = slice, max = summary.spreadMax)
        }
    }
}

@Composable
private fun SpreadRow(slice: MoodCount, max: Int) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.spacedBy(RowGap),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        BunnyFace(mood = slice.mood, modifier = Modifier.size(FaceSize))
        Text(
            text = slice.mood.label,
            style = typography.chipLabel,
            color = colors.ink,
            modifier = Modifier.width(LabelWidth),
        )
        Box(
            modifier = Modifier
                .weight(1f)
                .height(BarHeight)
                .clip(LeafyShapes.pill)
                .background(colors.soft2),
        ) {
            Box(
                modifier = Modifier
                    .fillMaxWidth(slice.count.toFloat() / max.toFloat())
                    .fillMaxHeight()
                    .clip(LeafyShapes.pill)
                    .background(colors.accent),
            )
        }
        Text(
            text = slice.count.toString(),
            style = typography.monthLabel.copy(fontSize = CountSize, fontWeight = FontWeight.W500),
            color = colors.ink2,
            textAlign = TextAlign.End,
            modifier = Modifier.width(CountWidth),
        )
    }
}
