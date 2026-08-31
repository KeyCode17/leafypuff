package com.leafypuff.ui.common

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.size
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.StrokeCap
import androidx.compose.ui.graphics.StrokeJoin
import androidx.compose.ui.graphics.drawscope.DrawScope
import androidx.compose.ui.graphics.drawscope.DrawStyle
import androidx.compose.ui.graphics.drawscope.Fill
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.graphics.drawscope.rotate
import androidx.compose.ui.graphics.drawscope.scale
import androidx.compose.ui.graphics.drawscope.translate
import androidx.compose.ui.graphics.vector.PathParser
import androidx.compose.ui.unit.dp
import com.leafypuff.domain.Mood

internal val BunnyFur = Color(0xFFFAFDFE)
internal val BunnyOutline = Color(0xFF3B4531)
internal const val OutlineWidth = 1.5f

private const val ViewBox = 48f
private const val DefaultFaceSize = 48
private const val HeadCx = 24f
private const val HeadCy = 30.5f
private const val HeadRx = 13f
private const val HeadRy = 11.4f
private const val LeftEarCx = 18f
private const val RightEarCx = 30f
private const val EarCy = 13.5f
private const val EarRx = 4.4f
private const val EarRy = 10.2f
private const val LeftEarInnerCx = 18.2f
private const val RightEarInnerCx = 29.8f
private const val EarInnerCy = 14f
private const val EarInnerRx = 2f
private const val EarInnerRy = 6.2f
private const val EarInnerAlpha = 0.4f
private const val NoseCy = 32.6f
private const val NoseRx = 1.5f
private const val NoseRy = 1.1f

@Composable
fun BunnyFace(mood: Mood, modifier: Modifier = Modifier) {
    val moodColor = Color(mood.dotArgb)
    val tilt = mood.earTilt.degrees
    Canvas(modifier.size(DefaultFaceSize.dp)) {
        val unit = size.minDimension / ViewBox
        translate(
            left = (size.width - ViewBox * unit) / 2f,
            top = (size.height - ViewBox * unit) / 2f,
        ) {
            scale(unit, Offset.Zero) {
                drawEar(LeftEarCx, LeftEarInnerCx, -tilt, moodColor)
                drawEar(RightEarCx, RightEarInnerCx, tilt, moodColor)
                furEllipse(HeadCx, HeadCy, HeadRx, HeadRy)
                drawEyes(mood.eyes)
                ellipse(HeadCx, NoseCy, NoseRx, NoseRy, moodColor)
                drawMouth(mood.mouth)
                mood.props.forEach { drawProp(it, moodColor) }
            }
        }
    }
}

private fun DrawScope.drawEar(cx: Float, innerCx: Float, tiltDegrees: Float, moodColor: Color) {
    rotate(tiltDegrees, Offset(cx, EarCy)) {
        furEllipse(cx, EarCy, EarRx, EarRy)
    }
    rotate(tiltDegrees, Offset(innerCx, EarInnerCy)) {
        ellipse(innerCx, EarInnerCy, EarInnerRx, EarInnerRy, moodColor, EarInnerAlpha)
    }
}

internal fun DrawScope.furEllipse(cx: Float, cy: Float, rx: Float, ry: Float) {
    ellipse(cx, cy, rx, ry, BunnyFur)
    ellipse(cx, cy, rx, ry, BunnyOutline, style = outlineStroke(OutlineWidth))
}

internal fun DrawScope.ellipse(
    cx: Float,
    cy: Float,
    rx: Float,
    ry: Float,
    color: Color,
    alpha: Float = 1f,
    style: DrawStyle = Fill,
) {
    drawOval(
        color = color,
        topLeft = Offset(cx - rx, cy - ry),
        size = Size(rx * 2f, ry * 2f),
        alpha = alpha,
        style = style,
    )
}

internal fun DrawScope.strokePath(path: Path, width: Float) {
    drawPath(path, BunnyOutline, style = outlineStroke(width))
}

internal fun outlineStroke(width: Float) =
    Stroke(width = width, cap = StrokeCap.Round, join = StrokeJoin.Round)

internal fun svgPath(data: String): Path = PathParser().parsePathString(data).toPath()
