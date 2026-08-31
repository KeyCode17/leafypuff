package com.leafypuff.theme

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.remember
import androidx.compose.runtime.staticCompositionLocalOf

val LocalLeafyColors = staticCompositionLocalOf { LeafyLightColors }
val LocalLeafyTypeScale = staticCompositionLocalOf { LeafyTypeScaleMedium }
val LocalLeafyTypography = staticCompositionLocalOf { leafyTypography(LeafyTypeScaleMedium) }

@Composable
fun LeafyTheme(
    darkOverride: Boolean? = null,
    typeScale: LeafyTypeScale = LeafyTypeScaleMedium,
    content: @Composable () -> Unit,
) {
    val dark = darkOverride ?: isSystemInDarkTheme()
    val colors = if (dark) LeafyDarkColors else LeafyLightColors
    val typography = remember(typeScale) { leafyTypography(typeScale) }
    CompositionLocalProvider(
        LocalLeafyColors provides colors,
        LocalLeafyTypeScale provides typeScale,
        LocalLeafyTypography provides typography,
        content = content,
    )
}
