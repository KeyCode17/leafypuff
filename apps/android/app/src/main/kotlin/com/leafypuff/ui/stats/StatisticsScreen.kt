package com.leafypuff.ui.stats

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

private val ScreenGutter = 24.dp
private val ScrollBottomPadding = 130.dp
private val HeaderTopPadding = 20.dp
private val HeaderBottomPadding = 24.dp
private val HeaderGap = 4.dp

fun rangeMetaLabel(range: StatRange): String = when (range) {
    StatRange.SevenDays -> "Last 7 days"
    StatRange.ThirtyDays -> "Last 30 days"
    StatRange.AllTime -> "All time"
}

@Composable
fun StatisticsScreen(
    summary: StatsSummary,
    range: StatRange,
    onRangeChange: (StatRange) -> Unit,
    modifier: Modifier = Modifier,
) {
    Column(
        modifier = modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState())
            .padding(horizontal = ScreenGutter)
            .padding(bottom = ScrollBottomPadding),
    ) {
        StatisticsHeader(range)
        Column(
            modifier = Modifier.fillMaxWidth(),
            verticalArrangement = Arrangement.spacedBy(BlockGap),
        ) {
            StatsSegmentedControl(range = range, onRangeChange = onRangeChange)
            StatFigureRow(summary)
            MoodSpreadCard(summary)
            MoodBalanceCard(summary)
            WritingDaysCard(summary)
            HashtagsCard(summary)
        }
    }
}

@Composable
private fun StatisticsHeader(range: StatRange) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Column(
        modifier = Modifier
            .fillMaxWidth()
            .padding(top = HeaderTopPadding, bottom = HeaderBottomPadding),
        verticalArrangement = Arrangement.spacedBy(HeaderGap),
    ) {
        Text(text = "Statistics", style = typography.screenTitle, color = colors.ink)
        Text(
            text = rangeMetaLabel(range).uppercase(),
            style = typography.metaLabel,
            color = colors.ink3,
        )
    }
}
