package com.leafypuff.theme

import androidx.compose.runtime.Immutable
import androidx.compose.ui.graphics.Color

@Immutable
data class LeafyColors(
    val bg: Color,
    val surface: Color,
    val sheet: Color,
    val ink: Color,
    val ink2: Color,
    val ink3: Color,
    val accent: Color,
    val accentDeep: Color,
    val soft: Color,
    val soft2: Color,
    val line: Color,
    val onAccent: Color,
    val key: Color,
    val keyLine: Color,
)

val Destructive = Color(0xFFEF4E4E)
val OnDestructive = Color(0xFFFFFFFF)
val MarkPlate = Color(0xFFF7FAEF)

val LeafyLightColors = LeafyColors(
    bg = Color(0xFFF6F8EC),
    surface = Color(0xFFFAFDFE),
    sheet = Color(0xFFFFFFFF),
    ink = Color(0xFF242D35),
    ink2 = Color(0xFF6B7580),
    ink3 = Color(0xFF9BA1A8),
    accent = Color(0xFF8B9A5F),
    accentDeep = Color(0xFF6F7C48),
    soft = Color(0xFFDEE5BC),
    soft2 = Color(0xFFE5EBC1),
    line = Color(0xFFE7EBDA),
    onAccent = Color(0xFF242D35),
    key = Color(0xFFFFFFFF),
    keyLine = Color(0xFFE7EBDA),
)

val LeafyDarkColors = LeafyColors(
    bg = Color(0xFF1E2818),
    surface = Color(0xFF21271C),
    sheet = Color(0xFF313F2E),
    ink = Color(0xFFC9C0A1),
    ink2 = Color(0xFFB2AA92),
    ink3 = Color(0xFF8F8871),
    accent = Color(0xFFC9C0A1),
    accentDeep = Color(0xFFDCD5BC),
    soft = Color(0xFF45553F),
    soft2 = Color(0xFF3B4A38),
    line = Color(0xFF3D4B3A),
    onAccent = Color(0xFF1E2818),
    key = Color(0xFF313F2E),
    keyLine = Color(0xFF4A5A46),
)
