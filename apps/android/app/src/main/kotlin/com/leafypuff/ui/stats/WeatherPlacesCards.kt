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
import com.leafypuff.data.label
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.popups.glyph

private val RowGap = 10.dp
private val GlyphSize = 20.dp
private val LabelWidth = 82.dp
private val BarHeight = 8.dp
private val CountWidth = 18.dp
private val CountSize = 12.sp

@Composable
fun WeatherCard(summary: StatsSummary, modifier: Modifier = Modifier) {
    if (summary.weather.isEmpty()) {
        return
    }

    StatsCard(modifier = modifier) {
        StatsCardLabel("Weather you wrote in")
        summary.weather.forEach { slice ->
            CountRow(
                glyph = slice.weather.glyph(),
                label = slice.weather.label(),
                count = slice.count,
                max = summary.weatherMax,
            )
        }
    }
}

@Composable
fun PlacesCard(summary: StatsSummary, modifier: Modifier = Modifier) {
    if (summary.places.isEmpty()) {
        return
    }

    StatsCard(modifier = modifier) {
        StatsCardLabel("Where you wrote")
        summary.places.forEach { slice ->
            CountRow(
                glyph = slice.location.glyph(),
                label = slice.location.label(),
                count = slice.count,
                max = summary.placeMax,
            )
        }
    }
}

@Composable
private fun CountRow(glyph: ImageVector, label: String, count: Int, max: Int) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.spacedBy(RowGap),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Icon(
            imageVector = glyph,
            contentDescription = null,
            tint = colors.accentDeep,
            modifier = Modifier.size(GlyphSize),
        )
        Text(
            text = label,
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
                    .fillMaxWidth(count.toFloat() / max.toFloat())
                    .fillMaxHeight()
                    .clip(LeafyShapes.pill)
                    .background(colors.accent),
            )
        }
        Text(
            text = count.toString(),
            style = typography.monthLabel.copy(fontSize = CountSize, fontWeight = FontWeight.W500),
            color = colors.ink2,
            textAlign = TextAlign.End,
            modifier = Modifier.width(CountWidth),
        )
    }
}
