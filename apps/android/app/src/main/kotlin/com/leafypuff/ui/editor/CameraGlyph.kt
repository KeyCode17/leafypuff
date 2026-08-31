package com.leafypuff.ui.editor

import androidx.compose.foundation.Canvas
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.CornerRadius
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.drawscope.Stroke

private const val ViewBox = 24f
private const val StrokeWidth = 1.6f
private const val BodyLeft = 2.4f
private const val BodyTop = 7.2f
private const val BodyWidth = 19.2f
private const val BodyHeight = 13.2f
private const val BodyRadius = 3.2f
private const val BumpLeft = 8.6f
private const val BumpRight = 15.4f
private const val BumpInset = 1.3f
private const val BumpTop = 4.2f
private const val LensCentreX = 12f
private const val LensCentreY = 13.8f
private const val LensRadius = 3.9f

@Composable
fun CameraGlyph(tint: Color, modifier: Modifier = Modifier) {
    Canvas(modifier = modifier) {
        val unit = size.minDimension / ViewBox
        val stroke = Stroke(width = StrokeWidth * unit)

        drawRoundRect(
            color = tint,
            topLeft = Offset(BodyLeft * unit, BodyTop * unit),
            size = Size(BodyWidth * unit, BodyHeight * unit),
            cornerRadius = CornerRadius(BodyRadius * unit, BodyRadius * unit),
            style = stroke,
        )

        val bump = Path().apply {
            moveTo(BumpLeft * unit, BodyTop * unit)
            lineTo((BumpLeft + BumpInset) * unit, BumpTop * unit)
            lineTo((BumpRight - BumpInset) * unit, BumpTop * unit)
            lineTo(BumpRight * unit, BodyTop * unit)
        }
        drawPath(path = bump, color = tint, style = stroke)

        drawCircle(
            color = tint,
            radius = LensRadius * unit,
            center = Offset(LensCentreX * unit, LensCentreY * unit),
            style = stroke,
        )
    }
}
