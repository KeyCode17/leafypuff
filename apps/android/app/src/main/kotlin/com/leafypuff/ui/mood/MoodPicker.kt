package com.leafypuff.ui.mood

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Close
import androidx.compose.material.icons.filled.KeyboardArrowDown
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import com.leafypuff.domain.Mood
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.diary.formatEntryDate
import kotlinx.datetime.LocalDate

private const val GridColumns = 3
private val ScreenGutter = 24.dp
private val DeckBottomPadding = 40.dp
private val HeaderTopPadding = 16.dp
private val HeaderBottomPadding = 24.dp
private val HeadingGap = 6.dp
private val HeadingBottomPadding = 24.dp
private val GridGap = 12.dp
private val CloseButtonSize = 36.dp
private val CloseGlyphSize = 18.dp
private val ChevronSize = 14.dp
private val Hairline = 0.5.dp
private val DateGlyphGap = 6.dp

@Composable
fun MoodPicker(
    entryDate: LocalDate,
    onPick: (Mood) -> Unit,
    onClose: () -> Unit,
    onDateClick: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Column(
        modifier = modifier
            .fillMaxSize()
            .background(colors.bg)
            .verticalScroll(rememberScrollState())
            .padding(horizontal = ScreenGutter)
            .padding(bottom = DeckBottomPadding),
    ) {
        DeckHeader(entryDate = entryDate, onClose = onClose, onDateClick = onDateClick)

        Column(
            modifier = Modifier.padding(bottom = HeadingBottomPadding),
            verticalArrangement = Arrangement.spacedBy(HeadingGap),
        ) {
            Text(text = "How are you?", style = typography.screenTitle, color = colors.ink)
            Text(
                text = "Pick one and start writing.",
                style = typography.body,
                color = colors.ink2,
            )
        }

        MoodGrid(onPick = onPick)
    }
}

@Composable
private fun DeckHeader(entryDate: LocalDate, onClose: () -> Unit, onDateClick: () -> Unit) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(top = HeaderTopPadding, bottom = HeaderBottomPadding),
        horizontalArrangement = Arrangement.SpaceBetween,
        verticalAlignment = Alignment.CenterVertically,
    ) {
        CloseButton(onClose = onClose)
        EntryDateButton(entryDate = entryDate, onDateClick = onDateClick)
    }
}

@Composable
private fun CloseButton(onClose: () -> Unit) {
    val colors = LocalLeafyColors.current

    Box(
        modifier = Modifier
            .size(CloseButtonSize)
            .clip(LeafyShapes.pill)
            .background(colors.surface)
            .border(Hairline, colors.line, LeafyShapes.pill)
            .clickable(onClick = onClose),
        contentAlignment = Alignment.Center,
    ) {
        Icon(
            imageVector = Icons.Filled.Close,
            contentDescription = "Close mood picker",
            tint = colors.ink,
            modifier = Modifier.size(CloseGlyphSize),
        )
    }
}

@Composable
private fun EntryDateButton(entryDate: LocalDate, onDateClick: () -> Unit) {
    val colors = LocalLeafyColors.current

    Row(
        modifier = Modifier
            .clip(LeafyShapes.pill)
            .clickable(onClick = onDateClick),
        horizontalArrangement = Arrangement.spacedBy(DateGlyphGap),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Text(
            text = formatEntryDate(entryDate).uppercase(),
            style = LocalLeafyTypography.current.metaLabel,
            color = colors.ink3,
        )
        Icon(
            imageVector = Icons.Filled.KeyboardArrowDown,
            contentDescription = null,
            tint = colors.ink3,
            modifier = Modifier.size(ChevronSize),
        )
    }
}

@Composable
private fun MoodGrid(onPick: (Mood) -> Unit) {
    Column(verticalArrangement = Arrangement.spacedBy(GridGap)) {
        Mood.entries.chunked(GridColumns).forEach { row ->
            Row(horizontalArrangement = Arrangement.spacedBy(GridGap)) {
                row.forEach { mood ->
                    MoodCard(mood = mood, onPick = onPick, modifier = Modifier.weight(1f))
                }
            }
        }
    }
}
