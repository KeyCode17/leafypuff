package com.leafypuff.ui.editor.sticker

import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.StrokeCap
import androidx.compose.ui.graphics.drawscope.DrawScope
import androidx.compose.ui.graphics.drawscope.scale
import androidx.compose.ui.graphics.drawscope.translate
import com.leafypuff.ui.common.outlineStroke
import com.leafypuff.ui.common.svgPath

private const val GlyphViewBox = 16f
private const val GlyphFraction = 13f / 24f
private const val GlyphStroke = 1.7f

private val RotateArrow = listOf(
    svgPath("M13.2 6.4A5.6 5.6 0 1 0 8 13.6"),
    svgPath("M13.6 2.6v3.9h-3.9"),
)

internal fun DrawScope.drawRotateGlyph(color: Color) {
    val glyph = size.minDimension * GlyphFraction
    val unit = glyph / GlyphViewBox
    translate(
        left = (size.width - glyph) / 2f,
        top = (size.height - glyph) / 2f,
    ) {
        scale(unit, Offset.Zero) {
            RotateArrow.forEach { drawPath(it, color, style = outlineStroke(GlyphStroke)) }
        }
    }
}

internal fun DrawScope.drawCross(center: Offset, arm: Float, width: Float) {
    drawLine(
        color = Color.White,
        start = Offset(center.x - arm, center.y - arm),
        end = Offset(center.x + arm, center.y + arm),
        strokeWidth = width,
        cap = StrokeCap.Round,
    )
    drawLine(
        color = Color.White,
        start = Offset(center.x + arm, center.y - arm),
        end = Offset(center.x - arm, center.y + arm),
        strokeWidth = width,
        cap = StrokeCap.Round,
    )
}
