package com.leafypuff.ui.common

import androidx.compose.ui.geometry.CornerRadius
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.drawscope.DrawScope
import androidx.compose.ui.graphics.drawscope.rotate
import com.leafypuff.domain.EyeStyle
import com.leafypuff.domain.FaceProp
import com.leafypuff.domain.MouthStyle

private const val LeftEyeCx = 19.5f
private const val RightEyeCx = 28.5f
private const val EyeCy = 29.4f
private const val DotEyeRadius = 1.8f
private const val WideEyeRadius = 2.5f
private const val AngryEyeRadius = 1.7f
private const val AngryEyeDrop = 0.4f
private const val CatchlightRadius = 0.8f
private const val CatchlightDx = 0.8f
private const val CatchlightDy = 0.9f
private const val ArcEyeWidth = 1.6f
private const val OpenMouthCx = 24f
private const val OpenMouthCy = 35.4f
private const val OpenMouthRx = 1.7f
private const val OpenMouthRy = 2f
private const val BlushCy = 33f
private const val BlushRx = 2.6f
private const val BlushRy = 1.6f
private const val BlushAlpha = 0.5f
private const val LeftBlushCx = 15.4f
private const val RightBlushCx = 32.6f
private const val PlasterTilt = -18f
private const val PlasterAlpha = 0.85f
private const val NearSleepZWidth = 1.4f
private const val FarSleepZWidth = 1.2f

private val PlasterPivot = Offset(33.2f, 26.3f)
private val PadTopLeft = Offset(29.2f, 24.6f)
private val PadSize = Size(8f, 3.4f)
private val PadCorner = CornerRadius(1.7f)
private val GauzeTopLeft = Offset(32.2f, 23.4f)
private val GauzeSize = Size(2f, 5.8f)
private val GauzeCorner = CornerRadius(1f)

private val ClosedEyes = listOf(
    svgPath("M17.2 29.2q2.3 2.2 4.6 0"),
    svgPath("M26.2 29.2q2.3 2.2 4.6 0"),
)
private val ArcEyes = listOf(
    svgPath("M17.2 30.4q2.3 -2.6 4.6 0"),
    svgPath("M26.2 30.4q2.3 -2.6 4.6 0"),
)
private val AngryBrows = listOf(
    svgPath("M17.2 26.4l4 1.4"),
    svgPath("M30.8 26.4l-4 1.4"),
)
private val SmileMouth = svgPath("M21.8 34.6q2.2 2 4.4 0")
private val FlatMouth = svgPath("M22.2 35.1h3.6")
private val FrownMouth = svgPath("M21.8 35.6q2.2 -1.8 4.4 0")
private val WavyMouth = svgPath("M21.6 35.1q1.1 -1.3 2.2 0q1.1 1.3 2.2 0")
private val Tear = svgPath("M31.4 32.6c1 1.6 1.6 2.6 1.6 3.3a1.6 1.6 0 0 1-3.2 0c0-.7.6-1.7 1.6-3.3z")
private val NearSleepZ = svgPath("M36 9h4l-4 4.6h4")
private val FarSleepZ = svgPath("M42.4 2.6h2.8l-2.8 3.2h2.8")
private val Heart =
    svgPath("M39 8.4c1.4-1.5 3.8-.5 3.8 1.5 0 2-2.6 3.7-3.8 4.6-1.2-.9-3.8-2.6-3.8-4.6 0-2 2.4-3 3.8-1.5z")

internal fun DrawScope.drawEyes(style: EyeStyle) {
    when (style) {
        EyeStyle.Closed -> ClosedEyes.forEach { strokePath(it, OutlineWidth) }
        EyeStyle.Arc -> ArcEyes.forEach { strokePath(it, ArcEyeWidth) }
        EyeStyle.Dot -> drawPupils(EyeCy, DotEyeRadius)
        EyeStyle.Wide -> drawWideEyes()
        EyeStyle.Angry -> drawAngryEyes()
    }
}

private fun DrawScope.drawWideEyes() {
    drawPupils(EyeCy, WideEyeRadius)
    drawCatchlight(LeftEyeCx)
    drawCatchlight(RightEyeCx)
}

private fun DrawScope.drawAngryEyes() {
    drawPupils(EyeCy + AngryEyeDrop, AngryEyeRadius)
    AngryBrows.forEach { strokePath(it, OutlineWidth) }
}

private fun DrawScope.drawPupils(cy: Float, radius: Float) {
    drawCircle(BunnyOutline, radius, Offset(LeftEyeCx, cy))
    drawCircle(BunnyOutline, radius, Offset(RightEyeCx, cy))
}

private fun DrawScope.drawCatchlight(eyeCx: Float) {
    drawCircle(BunnyFur, CatchlightRadius, Offset(eyeCx + CatchlightDx, EyeCy - CatchlightDy))
}

internal fun DrawScope.drawMouth(style: MouthStyle) {
    when (style) {
        MouthStyle.Smile -> strokePath(SmileMouth, OutlineWidth)
        MouthStyle.Flat -> strokePath(FlatMouth, OutlineWidth)
        MouthStyle.Frown -> strokePath(FrownMouth, OutlineWidth)
        MouthStyle.Wavy -> strokePath(WavyMouth, OutlineWidth)
        MouthStyle.Open ->
            ellipse(OpenMouthCx, OpenMouthCy, OpenMouthRx, OpenMouthRy, BunnyOutline)
    }
}

internal fun DrawScope.drawProp(prop: FaceProp, moodColor: Color) {
    when (prop) {
        FaceProp.Blush -> drawBlush(moodColor)
        FaceProp.Tear -> drawPath(Tear, moodColor)
        FaceProp.SleepZ -> drawSleepZ()
        FaceProp.Plaster -> drawPlaster(moodColor)
        FaceProp.Heart -> drawPath(Heart, moodColor)
    }
}

private fun DrawScope.drawBlush(moodColor: Color) {
    ellipse(LeftBlushCx, BlushCy, BlushRx, BlushRy, moodColor, BlushAlpha)
    ellipse(RightBlushCx, BlushCy, BlushRx, BlushRy, moodColor, BlushAlpha)
}

private fun DrawScope.drawSleepZ() {
    strokePath(NearSleepZ, NearSleepZWidth)
    strokePath(FarSleepZ, FarSleepZWidth)
}

private fun DrawScope.drawPlaster(moodColor: Color) {
    rotate(PlasterTilt, PlasterPivot) {
        drawRoundRect(moodColor, PadTopLeft, PadSize, PadCorner)
        drawRoundRect(BunnyFur, GauzeTopLeft, GauzeSize, GauzeCorner, alpha = PlasterAlpha)
    }
}
