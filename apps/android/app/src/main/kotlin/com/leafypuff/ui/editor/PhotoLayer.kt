package com.leafypuff.ui.editor

import androidx.compose.foundation.Image
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.key
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.graphicsLayer
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.layout.onSizeChanged
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.unit.dp
import com.leafypuff.ui.editor.sticker.PlacedFrame
import com.leafypuff.ui.editor.sticker.snapRotation
import com.leafypuff.ui.photo.EntryPhoto
import com.leafypuff.ui.photo.PhotoPlacement

const val PhotoPlaceMinSize = 72f
const val PhotoPlaceMaxSize = 320f
const val PhotoPlaceDropSize = 168f

private val PhotoCorner = 14.dp

@Composable
fun PhotoLayer(
    photos: List<EntryPhoto>,
    selectedId: String?,
    onSelect: (String?) -> Unit,
    onChange: (String, PhotoPlacement) -> Unit,
    onPutBack: (String) -> Unit,
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
        photos.forEach { photo ->
            val placed = photo.place ?: return@forEach
            key(photo.id) {
                PlacedFrame(
                    x = placed.x,
                    y = placed.y,
                    size = placed.size,
                    rotation = placed.rotation,
                    selected = photo.id == selectedId,
                    layerWidth = layerWidth,
                    layerHeight = layerHeight,
                    onSelect = { onSelect(photo.id) },
                    onMove = { deltaX, deltaY ->
                        onChange(
                            photo.id,
                            placed.movedBy(deltaX, deltaY, layerWidth, layerHeight),
                        )
                    },
                    onRotate = { onChange(photo.id, placed.copy(rotation = snapRotation(it))) },
                    onResize = {
                        onChange(
                            photo.id,
                            placed.copy(size = it.coerceIn(PhotoPlaceMinSize, PhotoPlaceMaxSize)),
                        )
                    },
                    onRemove = {
                        onPutBack(photo.id)
                        onSelect(null)
                    },
                ) {
                    Image(
                        bitmap = photo.cover,
                        contentDescription = null,
                        contentScale = ContentScale.Crop,
                        modifier = Modifier
                            .align(Alignment.Center)
                            .size(placed.size.dp)
                            .graphicsLayer { rotationZ = placed.rotation }
                            .clip(RoundedCornerShape(PhotoCorner)),
                    )
                }
            }
        }
    }
}

private fun PhotoPlacement.movedBy(
    deltaX: Float,
    deltaY: Float,
    layerWidth: Float,
    layerHeight: Float,
): PhotoPlacement = copy(
    x = roam(x + fractionOf(deltaX, layerWidth), size, layerWidth),
    y = roam(y + fractionOf(deltaY, layerHeight), size, layerHeight),
)

private fun roam(value: Float, size: Float, extent: Float): Float {
    if (extent <= 0f) {
        return 0f
    }
    val lip = size / 2f
    val lower = -lip / extent
    val upper = (extent - size + lip) / extent
    return value.coerceIn(lower, maxOf(lower, upper))
}

private fun fractionOf(delta: Float, extent: Float): Float = when {
    extent <= 0f -> 0f
    else -> delta / extent
}

fun droppedPlacement(index: Int): PhotoPlacement = PhotoPlacement(
    x = 0.12f + (index % 3) * 0.14f,
    y = 0.14f + (index % 4) * 0.12f,
    size = PhotoPlaceDropSize,
    rotation = 0f,
)
