package com.leafypuff.ui.settings

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material.icons.automirrored.filled.KeyboardArrowRight
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.leafypuff.theme.Destructive
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

private val TrailingGlyphSize = 16.dp
private val TrailingFontSize = 12.sp

@Composable
internal fun SettingsActionCard(
    entryCount: Int,
    versionName: String,
    lastSynced: String,
    onSync: () -> Unit,
    onExport: () -> Unit,
    onDeleteAll: () -> Unit,
    onSignOut: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current

    SettingsCard(padding = ListCardPadding, modifier = modifier) {
        ActionRow(onClick = onSync) {
            RowTitle(text = "Sync now", color = colors.ink)
            TrailingLabel(text = lastSynced)
            Icon(
                imageVector = Icons.AutoMirrored.Filled.KeyboardArrowRight,
                contentDescription = null,
                tint = colors.ink3,
                modifier = Modifier.size(TrailingGlyphSize),
            )
        }
        SettingsDivider()
        ActionRow(onClick = onExport) {
            RowTitle(text = "Export my diary", color = colors.ink)
            TrailingLabel(text = "$entryCount entries")
            Icon(
                imageVector = Icons.AutoMirrored.Filled.KeyboardArrowRight,
                contentDescription = null,
                tint = colors.ink3,
                modifier = Modifier.size(TrailingGlyphSize),
            )
        }
        SettingsDivider()
        ActionRow(onClick = onSignOut) {
            RowTitle(text = "Log out", color = colors.ink)
            TrailingLabel(text = "Keeps this diary on the device")
            Icon(
                imageVector = Icons.AutoMirrored.Filled.KeyboardArrowRight,
                contentDescription = null,
                tint = colors.ink3,
                modifier = Modifier.size(TrailingGlyphSize),
            )
        }
        SettingsDivider()
        ActionRow(onClick = onDeleteAll) {
            RowTitle(text = "Delete all entries", color = Destructive)
            Icon(
                imageVector = Icons.Filled.Delete,
                contentDescription = null,
                tint = Destructive,
                modifier = Modifier.size(TrailingGlyphSize),
            )
        }
        SettingsDivider()
        AboutRow(versionName = versionName)
    }
}

@Composable
private fun AboutRow(versionName: String) {
    ActionRow(onClick = null) {
        RowTitle(text = "About leafyPuff", color = LocalLeafyColors.current.ink)
        TrailingLabel(text = "Version $versionName")
    }
}

@Composable
private fun ActionRow(onClick: (() -> Unit)?, content: @Composable RowScope.() -> Unit) {
    val clickable = if (onClick == null) Modifier else Modifier.clickable(onClick = onClick)

    Row(
        modifier = Modifier
            .fillMaxWidth()
            .then(clickable)
            .padding(vertical = CardRowPaddingV),
        horizontalArrangement = Arrangement.spacedBy(CardContentGap),
        verticalAlignment = Alignment.CenterVertically,
        content = content,
    )
}

@Composable
private fun RowScope.RowTitle(text: String, color: Color) {
    Text(
        text = text,
        style = LocalLeafyTypography.current.body.copy(fontWeight = FontWeight.W500),
        color = color,
        modifier = Modifier.weight(1f),
    )
}

@Composable
private fun TrailingLabel(text: String) {
    Text(
        text = text,
        style = LocalLeafyTypography.current.chipLabel.copy(
            fontSize = TrailingFontSize,
            fontWeight = FontWeight.W400,
        ),
        color = LocalLeafyColors.current.ink3,
    )
}
