package com.leafypuff.theme

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.staticCompositionLocalOf

val LocalLeafyColors = staticCompositionLocalOf { LeafyLightColors }
val LocalLeafyTypeScale = staticCompositionLocalOf { LeafyTypeScaleMedium }

@Composable
fun LeafyTheme(
    darkOverride: Boolean? = null,
    typeScale: LeafyTypeScale = LeafyTypeScaleMedium,
    content: @Composable () -> Unit,
) {
    val dark = darkOverride ?: isSystemInDarkTheme()
    val colors = if (dark) LeafyDarkColors else LeafyLightColors
    CompositionLocalProvider(
        LocalLeafyColors provides colors,
        LocalLeafyTypeScale provides typeScale,
        content = content,
    )
}
