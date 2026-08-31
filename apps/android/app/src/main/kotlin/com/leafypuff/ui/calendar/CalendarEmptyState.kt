package com.leafypuff.ui.calendar

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.widthIn
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import com.leafypuff.domain.Mood
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.common.BunnyFace

private val CardElevation = 8.dp
private val CardPaddingH = 24.dp
private val CardPaddingV = 32.dp
private val ContentGap = 16.dp
private val BunnySize = 84.dp
private val BodyMaxWidth = 220.dp

@Composable
fun CalendarEmptyState(onCreate: () -> Unit, modifier: Modifier = Modifier) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Column(
        modifier = modifier
            .fillMaxWidth()
            .shadow(CardElevation, LeafyShapes.card)
            .background(colors.sheet, LeafyShapes.card)
            .clickable(onClick = onCreate)
            .padding(horizontal = CardPaddingH, vertical = CardPaddingV),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(ContentGap),
    ) {
        BunnyFace(mood = Mood.Grateful, modifier = Modifier.size(BunnySize))
        Text(
            text = "Let's tell your today's story!",
            style = typography.cardTitle,
            color = colors.ink,
            textAlign = TextAlign.Center,
        )
        Text(
            text = "Nothing written for this day yet. Tap here to start one.",
            style = typography.body,
            color = colors.ink2,
            textAlign = TextAlign.Center,
            modifier = Modifier.widthIn(max = BodyMaxWidth),
        )
    }
}
