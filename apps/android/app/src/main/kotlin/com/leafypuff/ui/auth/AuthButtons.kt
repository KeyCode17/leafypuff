package com.leafypuff.ui.auth

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyBrush
import com.leafypuff.theme.LeafyElevation
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LeafyStroke
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

private val PrimaryHeight = 50.dp
private val SecondaryHeight = 48.dp
private val DividerGap = 12.dp

@Composable
fun PrimaryCta(
    label: String,
    enabled: Boolean,
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current

    Box(
        modifier = modifier
            .fillMaxWidth()
            .height(PrimaryHeight)
            .shadow(
                elevation = LeafyElevation.glow,
                shape = LeafyShapes.button,
                ambientColor = colors.accent,
                spotColor = colors.accent,
            )
            .clip(LeafyShapes.button)
            .background(LeafyBrush.cta(colors.accent))
            .clickable(enabled = enabled, onClick = onClick),
        contentAlignment = Alignment.Center,
    ) {
        Text(
            text = label,
            style = LocalLeafyTypography.current.buttonLabel,
            color = colors.onAccent,
        )
    }
}

@Composable
internal fun ProviderButton(label: String, onClick: () -> Unit, modifier: Modifier = Modifier) {
    val colors = LocalLeafyColors.current

    Box(
        modifier = modifier
            .fillMaxWidth()
            .height(SecondaryHeight)
            .clip(LeafyShapes.button)
            .background(colors.surface)
            .border(LeafyStroke.border, colors.line, LeafyShapes.button)
            .clickable(onClick = onClick),
        contentAlignment = Alignment.Center,
    ) {
        Text(text = label, style = LocalLeafyTypography.current.chipLabel, color = colors.ink2)
    }
}

@Composable
internal fun OrDivider(modifier: Modifier = Modifier) {
    val colors = LocalLeafyColors.current

    Row(
        modifier = modifier.fillMaxWidth(),
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.spacedBy(DividerGap),
    ) {
        Hairline(Modifier.weight(1f))
        Text(text = "OR", style = LocalLeafyTypography.current.metaLabel, color = colors.ink3)
        Hairline(Modifier.weight(1f))
    }
}

@Composable
private fun Hairline(modifier: Modifier = Modifier) {
    Box(
        modifier = modifier
            .height(LeafyStroke.hairline)
            .background(LocalLeafyColors.current.line),
    )
}
