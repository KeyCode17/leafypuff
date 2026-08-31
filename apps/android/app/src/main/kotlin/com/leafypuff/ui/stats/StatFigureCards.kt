package com.leafypuff.ui.stats

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

private val FigureCardGap = 12.dp
private val FigureLabelGap = 4.dp
private val FigureLineHeight = 35.2.sp

@Composable
fun StatFigureRow(summary: StatsSummary, modifier: Modifier = Modifier) {
    Row(
        modifier = modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.spacedBy(FigureCardGap),
    ) {
        FigureCard(figure = summary.daysWritten, label = "Days written")
        FigureCard(figure = summary.longestStreak, label = "Longest streak")
    }
}

@Composable
private fun RowScope.FigureCard(figure: Int, label: String) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    StatsCard(modifier = Modifier.weight(1f), gap = FigureLabelGap) {
        Text(
            text = figure.toString(),
            style = typography.statFigure.copy(lineHeight = FigureLineHeight),
            color = colors.accentDeep,
        )
        Text(
            text = label,
            style = typography.chipLabel.copy(fontWeight = FontWeight.W400),
            color = colors.ink2,
        )
    }
}
