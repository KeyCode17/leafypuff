package com.leafypuff.ui.editor.sticker

import kotlin.math.PI
import kotlin.math.abs
import kotlin.math.atan2
import kotlin.math.roundToInt

const val StickerMinSize = 36f
const val StickerMaxSize = 180f
const val StickerDropSize = 64f
const val StickerOverhang = 12f
const val StickerSnapWindow = 5f

private const val QuarterTurn = 90f
private const val FullTurn = 360f
private const val DegreesPerRadian = 180f / PI.toFloat()
private const val DropOriginX = 34f
private const val DropOriginY = 60f
private const val DropStepX = 66f
private const val DropStepY = 44f
private const val DropColumns = 3
private const val DropRows = 4

fun clampStickerSize(size: Float): Float = size.coerceIn(StickerMinSize, StickerMaxSize)

fun clampStickerPosition(value: Float, size: Float, extent: Float): Float {
    val lower = -StickerOverhang
    val upper = extent - size + StickerOverhang
    return value.coerceIn(lower, maxOf(lower, upper))
}

fun normalizeRotation(degrees: Float): Float = degrees.mod(FullTurn)

fun snapRotation(degrees: Float): Float {
    val normalized = normalizeRotation(degrees)
    val nearest = (normalized / QuarterTurn).roundToInt() * QuarterTurn
    return if (abs(normalized - nearest) < StickerSnapWindow) {
        normalizeRotation(nearest)
    } else {
        normalized
    }
}

fun pointerAngle(centerX: Float, centerY: Float, x: Float, y: Float): Float =
    atan2(y - centerY, x - centerX) * DegreesPerRadian

fun dropX(index: Int): Float = DropOriginX + (index % DropColumns) * DropStepX

fun dropY(index: Int): Float = DropOriginY + (index % DropRows) * DropStepY

fun PlacedSticker.movedBy(
    deltaX: Float,
    deltaY: Float,
    layerWidth: Float,
    layerHeight: Float,
): PlacedSticker = copy(
    x = clampStickerPosition(x + deltaX, size, layerWidth),
    y = clampStickerPosition(y + deltaY, size, layerHeight),
)

fun PlacedSticker.resizedTo(next: Float): PlacedSticker = copy(size = clampStickerSize(next))

fun PlacedSticker.rotatedTo(next: Float): PlacedSticker = copy(rotation = snapRotation(next))
