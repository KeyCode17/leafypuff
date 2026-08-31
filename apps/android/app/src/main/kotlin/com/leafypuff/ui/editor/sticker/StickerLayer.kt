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
) {
    val current by rememberUpdatedState(sticker)
    val width by rememberUpdatedState(layerWidth)
    val height by rememberUpdatedState(layerHeight)

    Box(
        modifier = Modifier
            .offset(sticker.x.dp - HandleInset, sticker.y.dp - HandleInset)
            .size(sticker.size.dp + HandleInset * 2)
            .pointerInput(Unit) {
                detectTapGestures(onTap = { onSelect(current.key) })
            }
            .pointerInput(Unit) {
                detectDragGestures(
                    onDragStart = { onSelect(current.key) },
                    onDrag = { change, drag ->
                        change.consume()
                        onChange(
                            current.movedBy(
                                deltaX = drag.x.toDp().value,
                                deltaY = drag.y.toDp().value,
                                layerWidth = width,
                                layerHeight = height,
                            ),
                        )
                    },
                )
            },
    ) {
        StickerArt(
            sticker = sticker.sticker,
            modifier = Modifier
                .align(Alignment.Center)
                .size(sticker.size.dp)
                .graphicsLayer { rotationZ = sticker.rotation },
        )

        if (selected) {
            SelectionOutline(
                modifier = Modifier
                    .align(Alignment.Center)
                    .size(sticker.size.dp + OutlineInset * 2),
            )
            RemoveHandle(
                onRemove = {
                    onRemove(current.key)
                    onSelect(null)
                },
                modifier = Modifier.align(Alignment.TopStart),
            )
            RotateHandle(
                stickerSize = sticker.size,
                rotation = sticker.rotation,
                onRotate = { onChange(current.rotatedTo(it)) },
                modifier = Modifier.align(Alignment.TopEnd),
            )
            ResizeHandle(
                stickerSize = sticker.size,
                onResize = { onChange(current.resizedTo(it)) },
                modifier = Modifier.align(Alignment.BottomEnd),
            )
        }
    }
}
