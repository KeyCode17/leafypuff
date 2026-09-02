package com.leafypuff.ui.editor.sticker

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LeafyTheme
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.ui.editor.StickerTrayDrawer
import com.leafypuff.ui.settings.StickerPack

private const val FrameWidth = 375
private const val FrameHeight = 520
private val NoteHeight = 300.dp
private val FrameGutter = 24.dp

@Preview(name = "Sticker layer on light", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun StickerLayerLightPreview() {
    StickerFrame(dark = false)
}

@Preview(name = "Sticker layer on dark", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun StickerLayerDarkPreview() {
    StickerFrame(dark = true)
}

@Composable
private fun StickerFrame(dark: Boolean) {
    LeafyTheme(darkOverride = dark) {
        var stickers by remember { mutableStateOf(PreviewStickers) }
        var selected by remember { mutableStateOf<String?>("sk-1") }

        Column(
            modifier = Modifier
                .fillMaxSize()
                .background(LocalLeafyColors.current.bg),
            verticalArrangement = Arrangement.spacedBy(FrameGutter),
        ) {
            Box(
                modifier = Modifier
                    .padding(horizontal = FrameGutter)
                    .fillMaxWidth()
                    .height(NoteHeight),
            ) {
                Box(
                    modifier = Modifier
                        .fillMaxSize()
                        .clip(LeafyShapes.card)
                        .background(LocalLeafyColors.current.sheet),
                )
                StickerLayer(
                    stickers = stickers,
                    selectedKey = selected,
                    onSelect = { selected = it },
                    onChange = { next ->
                        stickers = stickers.map { if (it.key == next.key) next else it }
                    },
                    onRemove = { key -> stickers = stickers.filterNot { it.key == key } },
                    onBounds = { },
                )
            }
            StickerTrayDrawer(
                pack = StickerPack.Mixed,
                onPick = { picked ->
                    val dropped = dropSticker(picked, stickers.size, "sk-${stickers.size + 1}")
                    stickers = stickers + dropped
                    selected = dropped.key
                },
            )
        }
    }
}

private val PreviewStickers = listOf(
    PlacedSticker("sk-1", StickerId.BunSleep, 0.17f, 0.20f, 72f, 0f),
    PlacedSticker("sk-2", StickerId.Carrot, 0.58f, 0.75f, 62f, 11f),
)
