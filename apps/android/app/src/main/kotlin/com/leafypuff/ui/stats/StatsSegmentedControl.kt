package com.leafypuff.ui.stats

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

private val TrackPadding = 4.dp
private val SegmentGap = 4.dp
private val SegmentPadding = 8.dp

fun rangeLabel(range: StatRange): String = when (range) {
    StatRange.SevenDays -> "7 days"
    StatRange.ThirtyDays -> "30 days"
    StatRange.AllTime -> "All time"
}

@Composable
fun StatsSegmentedControl(
    range: StatRange,
    onRangeChange: (StatRange) -> Unit,
    modifier: Modifier = Modifier,
) {
    Row(
        modifier = modifier
            .fillMaxWidth()
            .clip(LeafyShapes.pill)
            .background(LocalLeafyColors.current.soft2)
            .padding(TrackPadding),
        horizontalArrangement = Arrangement.spacedBy(SegmentGap),
    ) {
        StatRange.entries.forEach { candidate ->
            Segment(
                range = candidate,
                selected = candidate == range,
                onRangeChange = onRangeChange,
            )
        }
    }
}

@Composable
private fun RowScope.Segment(
    range: StatRange,
    selected: Boolean,
    onRangeChange: (StatRange) -> Unit,
) {
    val colors = LocalLeafyColors.current

    Text(
        text = rangeLabel(range),
        style = LocalLeafyTypography.current.chipLabel.copy(
            fontWeight = if (selected) FontWeight.W600 else FontWeight.W500,
        ),
        color = if (selected) colors.onAccent else colors.ink2,
        textAlign = TextAlign.Center,
        modifier = Modifier
            .weight(1f)
            .clip(LeafyShapes.pill)
            .background(if (selected) colors.accent else colors.soft2)
            .clickable { onRangeChange(range) }
            .padding(vertical = SegmentPadding),
    )
}
