package com.pawnotes.theme

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.CompositionLocalProvider
import androidx.compose.runtime.staticCompositionLocalOf

val LocalPawColors = staticCompositionLocalOf { PawLightColors }
val LocalPawTypeScale = staticCompositionLocalOf { PawTypeScaleMedium }

@Composable
fun PawTheme(
    darkOverride: Boolean? = null,
    typeScale: PawTypeScale = PawTypeScaleMedium,
    content: @Composable () -> Unit,
) {
    val dark = darkOverride ?: isSystemInDarkTheme()
    val colors = if (dark) PawDarkColors else PawLightColors
    CompositionLocalProvider(
        LocalPawColors provides colors,
        LocalPawTypeScale provides typeScale,
        content = content,
    )
}
