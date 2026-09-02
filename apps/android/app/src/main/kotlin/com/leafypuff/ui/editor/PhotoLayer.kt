package com.leafypuff.ui.editor

import androidx.compose.foundation.Canvas
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
import androidx.compose.runtime.rememberUpdatedState
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.geometry.Rect
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.graphicsLayer
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.layout.onSizeChanged
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.unit.IntOffset
import androidx.compose.ui.unit.IntSize
import androidx.compose.ui.unit.dp
import com.leafypuff.ui.editor.sticker.PlacedFrame
import com.leafypuff.ui.editor.sticker.clampStickerPosition
import com.leafypuff.ui.editor.sticker.fractionOf
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
    originals: Map<String, ImageBitmap>,
    selectedId: String?,
    onSelect: (String?) -> Unit,
    onCrop: (String) -> Unit,
    onChange: (String, PhotoPlacement) -> Unit,
    onPutBack: (String) -> Unit,
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
        photos.forEach { photo ->
            val held = photo.place ?: return@forEach
            key(photo.id) {
                val placed by rememberUpdatedState(held)
                PlacedFrame(
                    x = placed.x,
                    y = placed.y,
                    size = placed.size,
                    rotation = placed.rotation,
                    selected = photo.id == selectedId,
                    height = placed.height,
                    layerWidth = layerWidth,
                    layerHeight = layerHeight,
                    onSelect = {
                        when (photo.id) {
                            selectedId -> onCrop(photo.id)
                            else -> onSelect(photo.id)
                        }
                    },
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
                    onBounds = onBounds,
                ) {
                    val frame = Modifier
                        .align(Alignment.Center)
                        .size(width = placed.size.dp, height = placed.height.dp)
                        .graphicsLayer { rotationZ = placed.rotation }
                        .clip(RoundedCornerShape(PhotoCorner))
                    val original = originals[photo.id]
                    val crop = placed.crop
                    if (original != null && crop != null) {
                        Canvas(modifier = frame) {
                            val across = (original.width * crop.width).toInt().coerceAtLeast(1)
                            val down = (across * placed.ratio).toInt()
                                .coerceIn(1, original.height)
                            val left = (original.width * crop.x).toInt()
                                .coerceIn(0, (original.width - across).coerceAtLeast(0))
                            val top = (original.height * crop.y).toInt()
                                .coerceIn(0, (original.height - down).coerceAtLeast(0))
                            drawImage(
                                image = original,
                                srcOffset = IntOffset(left, top),
                                srcSize = IntSize(across, down),
                                dstSize = IntSize(size.width.toInt(), size.height.toInt()),
                            )
                        }
                    } else {
                        Image(
                            bitmap = photo.cover,
                            contentDescription = null,
                            contentScale = ContentScale.Crop,
                            modifier = frame,
                        )
                    }
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
    x = clampStickerPosition(x + fractionOf(deltaX, layerWidth), size, layerWidth),
    y = clampStickerPosition(y + fractionOf(deltaY, layerHeight), height, layerHeight),
)

fun droppedPlacement(index: Int): PhotoPlacement = PhotoPlacement(
    x = 0.12f + (index % 3) * 0.14f,
    y = 0.14f + (index % 4) * 0.12f,
    size = PhotoPlaceDropSize,
    rotation = 0f,
)
