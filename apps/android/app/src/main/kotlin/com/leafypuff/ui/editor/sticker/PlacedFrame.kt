package com.leafypuff.ui.editor.sticker

import androidx.compose.foundation.gestures.detectDragGestures
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.BoxScope
import androidx.compose.foundation.layout.offset
import androidx.compose.foundation.layout.size
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.rememberUpdatedState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Rect
import androidx.compose.ui.graphics.graphicsLayer
import androidx.compose.ui.layout.boundsInWindow
import androidx.compose.ui.layout.onGloballyPositioned
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.unit.dp

@Composable
internal fun PlacedFrame(
    x: Float,
    y: Float,
    size: Float,
    rotation: Float,
    selected: Boolean,
    height: Float = size,
    layerWidth: Float,
    layerHeight: Float,
    onSelect: () -> Unit,
    onMove: (Float, Float) -> Unit,
    onRotate: (Float) -> Unit,
    onResize: (Float) -> Unit,
    onRemove: () -> Unit,
    onBounds: (Rect) -> Unit,
    content: @Composable BoxScope.() -> Unit,
) {
    val select by rememberUpdatedState(onSelect)
    val move by rememberUpdatedState(onMove)

    Box(
        modifier = Modifier
            .offset((x * layerWidth).dp - HandleInset, (y * layerHeight).dp - HandleInset)
            .size(width = size.dp + HandleInset * 2, height = height.dp + HandleInset * 2)
            .onGloballyPositioned { if (selected) onBounds(it.boundsInWindow()) }
            .pointerInput(Unit) { detectTapGestures(onTap = { select() }) }
            .pointerInput(Unit) {
                detectDragGestures(
                    onDragStart = { select() },
                    onDrag = { change, drag ->
                        change.consume()
                        move(drag.x.toDp().value, drag.y.toDp().value)
                    },
                )
            },
    ) {
        content()

        if (selected) {
            SelectionOutline(
                modifier = Modifier
                    .align(Alignment.Center)
                    .size(width = size.dp + OutlineInset * 2, height = height.dp + OutlineInset * 2)
                    .graphicsLayer { rotationZ = rotation },
            )
            RemoveHandle(onRemove = onRemove, modifier = Modifier.align(Alignment.TopStart))
            RotateHandle(
                stickerSize = size,
                rotation = rotation,
                onRotate = onRotate,
                modifier = Modifier.align(Alignment.TopEnd),
            )
            ResizeHandle(
                stickerSize = size,
                onResize = onResize,
                modifier = Modifier.align(Alignment.BottomEnd),
            )
        }
    }
}
