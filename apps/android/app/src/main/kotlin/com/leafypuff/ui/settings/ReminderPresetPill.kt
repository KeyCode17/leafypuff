package com.leafypuff.ui.settings

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

internal val PresetGap = 6.dp

private val PresetPaddingH = 13.dp
private val PresetPaddingV = 7.dp

@Composable
internal fun PresetPill(label: String, selected: Boolean, onClick: () -> Unit) {
    val colors = LocalLeafyColors.current

    Text(
        text = label,
        style = LocalLeafyTypography.current.chipLabel.copy(
            fontWeight = if (selected) FontWeight.W600 else FontWeight.W500,
        ),
        color = if (selected) colors.onAccent else colors.accentDeep,
        modifier = Modifier
            .clip(LeafyShapes.pill)
            .background(if (selected) colors.accent else colors.soft)
            .clickable(onClick = onClick)
            .padding(horizontal = PresetPaddingH, vertical = PresetPaddingV),
    )
}
