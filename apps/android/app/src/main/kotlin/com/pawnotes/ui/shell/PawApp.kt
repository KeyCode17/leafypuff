package com.pawnotes.ui.shell

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import com.pawnotes.theme.LocalPawColors

@Composable
fun PawApp() {
    val colors = LocalPawColors.current
    Box(
        modifier = Modifier
            .fillMaxSize()
            .background(colors.bg),
    )
}
