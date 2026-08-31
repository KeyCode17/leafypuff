package com.leafypuff.ui.shell

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import com.leafypuff.theme.LocalLeafyColors

@Composable
fun LeafyApp() {
    val colors = LocalLeafyColors.current
    Box(
        modifier = Modifier
            .fillMaxSize()
            .background(colors.bg),
    )
}
