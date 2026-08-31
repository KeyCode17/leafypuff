package com.leafypuff.ui.settings

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

private val GroupGap = 18.dp
private val GroupLabelGap = 10.dp
private val PillGap = 8.dp
private val PillPaddingV = 9.dp

@Composable
internal fun SettingsChoiceCard(
    stickerPack: StickerPack,
    textSize: TextSize,
    onStickerPackChange: (StickerPack) -> Unit,
    onTextSizeChange: (TextSize) -> Unit,
    modifier: Modifier = Modifier,
) {
    SettingsCard(
        padding = BlockCardPadding,
        modifier = modifier,
        verticalArrangement = Arrangement.spacedBy(GroupGap),
    ) {
        ChoiceGroup(label = "Sticker pack") {
            StickerPack.entries.forEach { pack ->
                ChoicePill(
                    label = pack.label,
                    selected = pack == stickerPack,
                    onClick = { onStickerPackChange(pack) },
                )
            }
        }
        ChoiceGroup(label = "Text size") {
            TextSize.entries.forEach { size ->
                ChoicePill(
                    label = size.label,
                    selected = size == textSize,
                    onClick = { onTextSizeChange(size) },
                )
            }
        }
    }
}

@Composable
private fun ChoiceGroup(label: String, options: @Composable RowScope.() -> Unit) {
    val colors = LocalLeafyColors.current

    Column(
        modifier = Modifier.fillMaxWidth(),
        verticalArrangement = Arrangement.spacedBy(GroupLabelGap),
    ) {
        Text(
            text = label,
            style = LocalLeafyTypography.current.body.copy(fontWeight = FontWeight.W500),
            color = colors.ink,
        )
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(PillGap),
            verticalAlignment = Alignment.CenterVertically,
            content = options,
        )
    }
}

@Composable
private fun RowScope.ChoicePill(label: String, selected: Boolean, onClick: () -> Unit) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Text(
        text = label,
        style = typography.chipLabel.copy(
            fontWeight = if (selected) FontWeight.W600 else FontWeight.W500,
        ),
        color = if (selected) colors.onAccent else colors.ink2,
        textAlign = TextAlign.Center,
        modifier = Modifier
            .weight(1f)
            .clip(LeafyShapes.pill)
            .background(if (selected) colors.accent else colors.soft2)
            .clickable(onClick = onClick)
            .padding(vertical = PillPaddingV),
    )
}
