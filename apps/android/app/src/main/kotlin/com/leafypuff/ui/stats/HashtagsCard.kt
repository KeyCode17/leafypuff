package com.leafypuff.ui.stats

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
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

private val ChipGap = 8.dp
private val ChipInnerGap = 6.dp
private val ChipPaddingX = 12.dp
private val ChipPaddingY = 6.dp
private val CountTextSize = 11.sp

@OptIn(ExperimentalLayoutApi::class)
@Composable
fun HashtagsCard(summary: StatsSummary, modifier: Modifier = Modifier) {
    StatsCard(modifier = modifier) {
        StatsCardLabel("Most used hashtags")
        FlowRow(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(ChipGap),
            verticalArrangement = Arrangement.spacedBy(ChipGap),
        ) {
            summary.tags.forEach { tag -> TagCountChip(tag) }
        }
    }
}

@Composable
private fun TagCountChip(tag: TagCount) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Row(
        modifier = Modifier
            .clip(LeafyShapes.chip)
            .background(colors.soft)
            .padding(horizontal = ChipPaddingX, vertical = ChipPaddingY),
        horizontalArrangement = Arrangement.spacedBy(ChipInnerGap),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Text(text = tag.tag, style = typography.chipLabel, color = colors.accentDeep)
        Text(
            text = tag.count.toString(),
            style = typography.monthLabel.copy(
                fontSize = CountTextSize,
                fontWeight = FontWeight.W500,
            ),
            color = colors.ink2,
        )
    }
}
