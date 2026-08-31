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
    bg = Color(0xFF191D14),
    surface = Color(0xFF22271B),
    sheet = Color(0xFF22271B),
    ink = Color(0xFFF1F4E6),
    ink2 = Color(0xFFA9B096),
    ink3 = Color(0xFF79806A),
    accent = Color(0xFFA9BA76),
    accentDeep = Color(0xFFC4CE96),
    soft = Color(0xFF333B26),
    soft2 = Color(0xFF2C3321),
    line = Color(0xFF343B29),
    onAccent = Color(0xFF1A1F12),
    key = Color(0xFF2C3321),
    keyLine = Color(0xFF3F4830),
)
