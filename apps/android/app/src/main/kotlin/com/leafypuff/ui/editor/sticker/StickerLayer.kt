package com.leafypuff.ui.editor.sticker

import androidx.compose.foundation.gestures.detectDragGestures
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.offset
import androidx.compose.foundation.layout.size
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.key
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberUpdatedState
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.graphicsLayer
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.geometry.Rect
import androidx.compose.ui.layout.onSizeChanged
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.unit.dp

@Composable
fun StickerLayer(
    stickers: List<PlacedSticker>,
    selectedKey: String?,
    onSelect: (String?) -> Unit,
    onChange: (PlacedSticker) -> Unit,
    onRemove: (String) -> Unit,
    onBounds: (Rect) -> Unit,
    modifier: Modifier = Modifier,
) {
    val density = LocalDensity.current
    var layerWidth by remember { mutableStateOf(0f) }
    var layerHeight by remember { mutableStateOf(0f) }

    Box(
        modifier = modifier
            .fillMaxSize()
            .onSizeChanged {
                with(density) {
                    layerWidth = it.width.toDp().value
                    layerHeight = it.height.toDp().value
                }
            },
    ) {
        stickers.forEach { placed ->
            key(placed.key) {
                PlacedStickerBox(
                    sticker = placed,
                    selected = placed.key == selectedKey,
                    layerWidth = layerWidth,
                    layerHeight = layerHeight,
                    onSelect = onSelect,
                    onChange = onChange,
                    onRemove = onRemove,
                    onBounds = onBounds,
                )
            }
        }
    }
}

@Composable
private fun PlacedStickerBox(
    sticker: PlacedSticker,
    selected: Boolean,
    layerWidth: Float,
    layerHeight: Float,
    onSelect: (String?) -> Unit,
    onChange: (PlacedSticker) -> Unit,
    onRemove: (String) -> Unit,
    onBounds: (Rect) -> Unit,
) {
    val current by rememberUpdatedState(sticker)
    val width by rememberUpdatedState(layerWidth)
    val height by rememberUpdatedState(layerHeight)

    PlacedFrame(
        x = sticker.x,
        y = sticker.y,
        size = sticker.size,
        rotation = sticker.rotation,
        selected = selected,
        layerWidth = layerWidth,
        layerHeight = layerHeight,
        onSelect = { onSelect(current.key) },
        onMove = { deltaX, deltaY ->
            onChange(current.movedBy(deltaX, deltaY, width, height))
        },
        onRotate = { onChange(current.rotatedTo(it)) },
        onResize = { onChange(current.resizedTo(it)) },
        onRemove = {
            onRemove(current.key)
            onSelect(null)
        },
        onBounds = onBounds,
    ) {
        StickerArt(
            sticker = sticker.sticker,
            modifier = Modifier
                .align(Alignment.Center)
                .size(sticker.size.dp)
                .graphicsLayer { rotationZ = sticker.rotation },
        )
    }
}
