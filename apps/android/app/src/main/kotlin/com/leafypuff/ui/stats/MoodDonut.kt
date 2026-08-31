package com.leafypuff.ui.stats

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.size
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.drawscope.DrawScope
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.leafypuff.domain.MoodGroup
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

private const val ViewBox = 88f
private const val Radius = 34f
private const val StrokeWidth = 15f
private const val StartAngle = -90f
private const val FullTurn = 360f

internal val DonutSize = 104.dp
private val TotalTextSize = 20.sp

private val PositiveColor = Color(0xFF8B9A5F)
private val NeutralColor = Color(0xFFC3C8B4)
private val NegativeColor = Color(0xFFD08C8C)

fun groupColor(group: MoodGroup): Color = when (group) {
    MoodGroup.Positive -> PositiveColor
    MoodGroup.Neutral -> NeutralColor
    MoodGroup.Negative -> NegativeColor
}

fun groupLabel(group: MoodGroup): String = group.name

@Composable
fun MoodDonut(slices: List<GroupCount>, total: Int, modifier: Modifier = Modifier) {
    val colors = LocalLeafyColors.current
    val empty = colors.line

    Box(modifier = modifier.size(DonutSize), contentAlignment = Alignment.Center) {
        Canvas(modifier = Modifier.fillMaxSize()) {
            if (total <= 0) drawRing(empty, StartAngle, FullTurn) else drawSlices(slices, total)
        }
        Text(
            text = total.toString(),
            style = LocalLeafyTypography.current.statFigure.copy(
                fontSize = TotalTextSize,
                fontWeight = FontWeight.W600,
            ),
            color = colors.ink2,
        )
    }
}

private fun DrawScope.drawSlices(slices: List<GroupCount>, total: Int) {
    var start = StartAngle
    slices.forEach { slice ->
        if (slice.count > 0) {
            val sweep = FullTurn * slice.count.toFloat() / total.toFloat()
            drawRing(groupColor(slice.group), start, sweep)
            start += sweep
        }
    }
}

private fun DrawScope.drawRing(color: Color, startAngle: Float, sweepAngle: Float) {
    val scale = size.minDimension / ViewBox
    val radius = Radius * scale
    val stroke = StrokeWidth * scale
    drawArc(
        color = color,
        startAngle = startAngle,
        sweepAngle = sweepAngle,
        useCenter = false,
        topLeft = Offset(center.x - radius, center.y - radius),
        size = Size(radius * 2f, radius * 2f),
        style = Stroke(width = stroke),
    )
}
