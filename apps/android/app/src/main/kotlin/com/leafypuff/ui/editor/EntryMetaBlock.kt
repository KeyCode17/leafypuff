package com.leafypuff.ui.editor

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.DateRange
import androidx.compose.material.icons.filled.KeyboardArrowDown
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.leafypuff.domain.Mood
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LeafyStroke
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.common.BunnyFace
import com.leafypuff.ui.common.formatEntryDate
import com.leafypuff.ui.popups.LocationOptions
import com.leafypuff.ui.popups.LocationTitle
import com.leafypuff.ui.popups.WeatherOptions
import com.leafypuff.ui.popups.WeatherTitle
import com.leafypuff.ui.popups.glyphOf
import kotlinx.datetime.LocalDate

private val BlockGap = 12.dp
private val BlockBottomPadding = 18.dp
private val GlyphGap = 6.dp
private val CalendarGlyphSize = 16.dp
private val ChevronSize = 14.dp
private val ChipGap = 8.dp
private val BunnySize = 24.dp
private val ChipGlyphSize = 16.dp
private val MoodChipPadding = PaddingValues(start = 6.dp, top = 5.dp, end = 12.dp, bottom = 5.dp)
private val OptionChipPadding = PaddingValues(horizontal = 14.dp, vertical = 8.dp)

@Composable
fun EntryMetaBlock(
    date: LocalDate,
    mood: Mood,
    weather: String?,
    location: String?,
    onDateClick: () -> Unit,
    onMoodClick: () -> Unit,
    onWeatherClick: () -> Unit,
    onLocationClick: () -> Unit,
    modifier: Modifier = Modifier,
) {
    Column(
        modifier = modifier.padding(bottom = BlockBottomPadding),
        verticalArrangement = Arrangement.spacedBy(BlockGap),
    ) {
        DateRow(date = date, onDateClick = onDateClick)

        Row(
            horizontalArrangement = Arrangement.spacedBy(ChipGap),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            MoodChip(mood = mood, onMoodClick = onMoodClick)
            OptionChip(
                label = weather ?: WeatherTitle,
                glyph = WeatherOptions.glyphOf(weather),
                onClick = onWeatherClick,
            )
            OptionChip(
                label = location ?: LocationTitle,
                glyph = LocationOptions.glyphOf(location),
                onClick = onLocationClick,
            )
        }
    }
}

@Composable
private fun DateRow(date: LocalDate, onDateClick: () -> Unit) {
    val colors = LocalLeafyColors.current

    Row(
        modifier = Modifier
            .clip(LeafyShapes.pill)
            .clickable(onClick = onDateClick),
        horizontalArrangement = Arrangement.spacedBy(GlyphGap),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Icon(
            imageVector = Icons.Filled.DateRange,
            contentDescription = null,
            tint = colors.ink3,
            modifier = Modifier.size(CalendarGlyphSize),
        )
        Text(
            text = formatEntryDate(date).uppercase(),
            style = LocalLeafyTypography.current.metaLabel,
            color = colors.ink3,
        )
        Icon(
            imageVector = Icons.Filled.KeyboardArrowDown,
            contentDescription = "Change entry date",
            tint = colors.ink3,
            modifier = Modifier.size(ChevronSize),
        )
    }
}

@Composable
private fun MoodChip(mood: Mood, onMoodClick: () -> Unit) {
    val colors = LocalLeafyColors.current

    Row(
        modifier = Modifier
            .clip(LeafyShapes.chip)
            .background(colors.soft2)
            .clickable(onClick = onMoodClick)
            .padding(MoodChipPadding),
        horizontalArrangement = Arrangement.spacedBy(GlyphGap),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        BunnyFace(mood = mood, modifier = Modifier.size(BunnySize))
        Text(
            text = mood.label,
            style = LocalLeafyTypography.current.chipLabel.copy(fontWeight = FontWeight.W600),
            color = colors.ink,
        )
    }
}

@Composable
private fun OptionChip(label: String, glyph: ImageVector?, onClick: () -> Unit) {
    val colors = LocalLeafyColors.current

    Row(
        modifier = Modifier
            .clip(LeafyShapes.chip)
            .background(colors.surface)
            .border(LeafyStroke.hairline, colors.line, LeafyShapes.chip)
            .clickable(onClick = onClick)
            .padding(OptionChipPadding),
        horizontalArrangement = Arrangement.spacedBy(GlyphGap),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        if (glyph != null) {
            Icon(
                imageVector = glyph,
                contentDescription = null,
                tint = colors.ink2,
                modifier = Modifier.size(ChipGlyphSize),
            )
        }
        Text(
            text = label,
            style = LocalLeafyTypography.current.chipLabel,
            color = colors.ink2,
        )
    }
}
