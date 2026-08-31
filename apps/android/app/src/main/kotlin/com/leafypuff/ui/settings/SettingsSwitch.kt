package com.leafypuff.ui.settings

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors

private val TrackWidth = 44.dp
private val TrackHeight = 26.dp
private val KnobSize = 20.dp
private val KnobInset = 3.dp
private val KnobColor = Color(0xFFFFFFFF)

@Composable
internal fun SettingsSwitch(checked: Boolean, modifier: Modifier = Modifier) {
    val colors = LocalLeafyColors.current

    Box(
        modifier = modifier
            .size(width = TrackWidth, height = TrackHeight)
            .clip(LeafyShapes.pill)
            .background(if (checked) colors.accent else colors.line),
        contentAlignment = if (checked) Alignment.CenterEnd else Alignment.CenterStart,
    ) {
        Box(
            modifier = Modifier
                .padding(horizontal = KnobInset)
                .size(KnobSize)
                .clip(LeafyShapes.pill)
                .background(KnobColor),
        )
    }
}
