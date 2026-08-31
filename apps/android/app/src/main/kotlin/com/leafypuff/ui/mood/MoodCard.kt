package com.leafypuff.ui.mood

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.leafypuff.domain.Mood
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.common.BunnyFace

private val CardElevation = 8.dp
private val CardFaceSize = 52.dp
private val CardTopPadding = 14.dp
private val CardBottomPadding = 12.dp
private val CardSidePadding = 6.dp
private val FaceToLabelGap = 8.dp

@Composable
fun MoodCard(mood: Mood, onPick: (Mood) -> Unit, modifier: Modifier = Modifier) {
    val colors = LocalLeafyColors.current
    val labelStyle = LocalLeafyTypography.current.chipLabel.copy(fontWeight = FontWeight.W600)

    Column(
        modifier = modifier
            .shadow(CardElevation, LeafyShapes.card)
            .clip(LeafyShapes.card)
            .background(colors.sheet)
            .clickable { onPick(mood) }
            .padding(
                start = CardSidePadding,
                end = CardSidePadding,
                top = CardTopPadding,
                bottom = CardBottomPadding,
            ),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(FaceToLabelGap),
    ) {
        BunnyFace(mood = mood, modifier = Modifier.size(CardFaceSize))
        Text(text = mood.label, style = labelStyle, color = colors.ink, maxLines = 1)
    }
}
