package com.leafypuff.ui.editor.sticker

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.gestures.detectDragGestures
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.rememberUpdatedState
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.geometry.CornerRadius
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.PathEffect
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.input.pointer.PointerInputScope
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LocalLeafyColors

internal val HandleSize = 24.dp
internal val HandleInset = 11.dp
internal val OutlineInset = 7.dp

private val RemoveFill = Color(0xFFEF4E4E)
private val HandleLift = 2.dp
private val OutlineRadius = 14.dp
private val OutlineWidth = 1.5.dp
private val DashOn = 4.dp
private val DashOff = 3.dp
private val CrossArm = 3.6.dp
private val CropArm = 3.2.dp
private val CrossWidth = 1.8.dp

@Composable
internal fun SelectionOutline(modifier: Modifier = Modifier) {
    val accent = LocalLeafyColors.current.accent

    Canvas(modifier) {
        val stroke = OutlineWidth.toPx()
        drawRoundRect(
            color = accent,
            topLeft = Offset(stroke / 2f, stroke / 2f),
            size = Size(size.width - stroke, size.height - stroke),
            cornerRadius = CornerRadius(OutlineRadius.toPx()),
            style = Stroke(
                width = stroke,
                pathEffect = PathEffect.dashPathEffect(
                    floatArrayOf(DashOn.toPx(), DashOff.toPx()),
                ),
            ),
        )
    }
}

@Composable
internal fun RemoveHandle(onRemove: () -> Unit, modifier: Modifier = Modifier) {
    Box(
        modifier = modifier
            .size(HandleSize)
            .background(RemoveFill, CircleShape)
            .pointerInput(Unit) {
                detectTapGestures(onTap = { onRemove() })
            },
    ) {
        Canvas(Modifier.size(HandleSize)) {
            val arm = CrossArm.toPx()
            val center = Offset(size.width / 2f, size.height / 2f)
            drawCross(center, arm, CrossWidth.toPx())
        }
    }
}

@Composable
internal fun CropHandle(onCrop: () -> Unit, modifier: Modifier = Modifier) {
    val colors = LocalLeafyColors.current

    Box(
        modifier = modifier
            .size(HandleSize)
            .shadow(HandleLift, CircleShape)
            .background(colors.sheet, CircleShape)
            .pointerInput(Unit) {
                detectTapGestures(onTap = { onCrop() })
            },
    ) {
        Canvas(Modifier.size(HandleSize)) {
            drawCropGlyph(colors.accentDeep)
        }
    }
}

private fun androidx.compose.ui.graphics.drawscope.DrawScope.drawCropGlyph(color: Color) {
    val arm = CropArm.toPx()
    val stroke = CrossWidth.toPx()
    val c = Offset(size.width / 2f, size.height / 2f)
    drawLine(color, Offset(c.x - arm, c.y - arm * 1.6f), Offset(c.x - arm, c.y + arm), stroke)
    drawLine(color, Offset(c.x - arm, c.y + arm), Offset(c.x + arm * 1.6f, c.y + arm), stroke)
    drawLine(color, Offset(c.x - arm * 1.6f, c.y - arm), Offset(c.x + arm, c.y - arm), stroke)
    drawLine(color, Offset(c.x + arm, c.y - arm), Offset(c.x + arm, c.y + arm * 1.6f), stroke)
}

@Composable
internal fun RotateHandle(
    stickerSize: Float,
    rotation: Float,
    onRotate: (Float) -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current
    val current by rememberUpdatedState(stickerSize)
    val currentRotation by rememberUpdatedState(rotation)

    Box(
        modifier = modifier
            .size(HandleSize)
            .shadow(HandleLift, CircleShape)
            .background(colors.sheet, CircleShape)
            .pointerInput(Unit) {
                var base = 0f
                var grabbed = 0f
                detectDragGestures(
                    onDragStart = { offset ->
                        base = currentRotation
                        grabbed = angleFromCenter(current, offset)
                    },
                    onDrag = { change, _ ->
                        change.consume()
                        val now = angleFromCenter(current, change.position)
                        onRotate(base + now - grabbed)
                    },
                )
            },
    ) {
        Canvas(Modifier.size(HandleSize)) {
            drawRotateGlyph(colors.accentDeep)
        }
    }
}

@Composable
internal fun ResizeHandle(
    stickerSize: Float,
    onResize: (Float) -> Unit,
    modifier: Modifier = Modifier,
) {
    val accent = LocalLeafyColors.current.accent
    val current by rememberUpdatedState(stickerSize)

    Box(
        modifier = modifier
            .size(HandleSize)
            .shadow(HandleLift, CircleShape)
            .background(accent, CircleShape)
            .pointerInput(Unit) {
                var base = 0f
                var travelX = 0f
                var travelY = 0f
                detectDragGestures(
                    onDragStart = {
                        base = current
                        travelX = 0f
                        travelY = 0f
                    },
                    onDrag = { change, drag ->
                        change.consume()
                        travelX += drag.x
                        travelY += drag.y
                        onResize(base + maxOf(travelX, travelY).toDp().value)
                    },
                )
            },
    )
}

private fun PointerInputScope.angleFromCenter(
    stickerSize: Float,
    point: Offset,
): Float = pointerAngle(
    (HandleSize - HandleInset).toPx() - stickerSize.dp.toPx() / 2f,
    HandleInset.toPx() + stickerSize.dp.toPx() / 2f,
    point.x,
    point.y,
)
