package com.leafypuff.theme

import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color

private val GradientHead = Color(0xFF9DAE6B)

/**
 * The accent gradient, `linear-gradient(<angle>, #9DAE6B, accent)`. A CSS angle names the
 * direction the gradient travels: 225deg runs top-right to bottom-left, 256deg is the same
 * diagonal flattened towards horizontal. Infinite offsets let each brush size itself to the
 * box it fills.
 */
object LeafyBrush {
    fun fab(accent: Color): Brush = Brush.linearGradient(
        colors = listOf(GradientHead, accent),
        start = Offset(Float.POSITIVE_INFINITY, 0f),
        end = Offset(0f, Float.POSITIVE_INFINITY),
    )

    fun cta(accent: Color): Brush = Brush.linearGradient(
        colors = listOf(GradientHead, accent),
        start = Offset(Float.POSITIVE_INFINITY, 0f),
        end = Offset.Zero,
    )
}
