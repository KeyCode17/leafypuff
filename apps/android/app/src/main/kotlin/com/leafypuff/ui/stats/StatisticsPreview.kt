package com.leafypuff.ui.stats

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.leafypuff.data.SampleEntries
import com.leafypuff.domain.Entry
import com.leafypuff.theme.LeafyTheme
import com.leafypuff.theme.LocalLeafyColors

private const val FrameWidth = 375
private const val FrameHeight = 812

@Preview(name = "Statistics on light", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun StatisticsLightPreview() {
    StatisticsFrame(dark = false, range = StatRange.AllTime, entries = SampleEntries)
}

@Preview(name = "Statistics on dark", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun StatisticsDarkPreview() {
    StatisticsFrame(dark = true, range = StatRange.AllTime, entries = SampleEntries)
}

@Preview(name = "Statistics with nothing in range", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun StatisticsEmptyPreview() {
    StatisticsFrame(dark = false, range = StatRange.SevenDays, entries = emptyList())
}

@Composable
private fun StatisticsFrame(dark: Boolean, range: StatRange, entries: List<Entry>) {
    LeafyTheme(darkOverride = dark) {
        Box(modifier = Modifier.fillMaxSize().background(LocalLeafyColors.current.bg)) {
            StatisticsScreen(entries = entries, range = range, onRangeChange = { })
        }
    }
}
