package com.leafypuff.ui.shell

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

@Composable
fun PlaceholderScreen(destination: Destination, modifier: Modifier = Modifier) {
    val colors = LocalLeafyColors.current

    Box(
        modifier = modifier
            .fillMaxSize()
            .padding(24.dp),
        contentAlignment = Alignment.Center,
    ) {
        Text(
            text = "${destination.label} lands in a later slice.",
            style = LocalLeafyTypography.current.body,
            color = colors.ink3,
        )
    }
}
