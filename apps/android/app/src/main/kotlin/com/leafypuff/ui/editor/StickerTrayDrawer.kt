package com.leafypuff.ui.editor

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.rememberScrollState
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.editor.sticker.StickerArt
import com.leafypuff.ui.editor.sticker.StickerId
import com.leafypuff.ui.editor.sticker.stickersFor
import com.leafypuff.ui.settings.StickerPack

private val TileSize = 52.dp
private val TileArtSize = 34.dp
private val TileGap = 8.dp
private val Hairline = 0.5.dp

@Composable
fun StickerTrayDrawer(
    pack: StickerPack = StickerPack.Mixed,
    onPick: (StickerId) -> Unit = {},
    modifier: Modifier = Modifier,
) {
    EditorDrawer(modifier = modifier) {
        Text(
            text = "Tap one to drop it on the note, then drag, rotate or resize it.",
            style = LocalLeafyTypography.current.chipLabel,
            color = LocalLeafyColors.current.ink3,
        )
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .horizontalScroll(rememberScrollState()),
            horizontalArrangement = Arrangement.spacedBy(TileGap),
        ) {
            stickersFor(pack).forEach { sticker ->
                StickerTrayTile(sticker = sticker, onClick = { onPick(sticker) })
            }
        }
    }
}

@Composable
private fun StickerTrayTile(sticker: StickerId, onClick: () -> Unit) {
    val colors = LocalLeafyColors.current

    Box(
        modifier = Modifier
            .size(TileSize)
            .clip(LeafyShapes.stickerTrayTile)
            .background(colors.sheet)
            .border(Hairline, colors.line, LeafyShapes.stickerTrayTile)
            .clickable(onClick = onClick),
        contentAlignment = Alignment.Center,
    ) {
        StickerArt(sticker = sticker, modifier = Modifier.size(TileArtSize))
    }
}
