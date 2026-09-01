package com.leafypuff.theme

import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color

private val GradientHead = Color(0xFF9DAE6B)

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
