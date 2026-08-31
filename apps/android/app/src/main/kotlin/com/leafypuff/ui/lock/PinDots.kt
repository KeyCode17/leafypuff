package com.leafypuff.ui.lock

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LocalLeafyColors

private const val DotCount = 4

private val DotSize = 13.dp
private val DotFillSize = 15.dp
private val DotRing = 1.5.dp
private val DotGap = 18.dp

@Composable
internal fun PinDots(pinLength: Int, modifier: Modifier = Modifier) {
    val colors = LocalLeafyColors.current

    Row(
        modifier = modifier,
        horizontalArrangement = Arrangement.spacedBy(DotGap),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        repeat(DotCount) { index ->
            Box(
                modifier = Modifier
                    .size(DotSize)
                    .border(DotRing, colors.ink3, CircleShape),
                contentAlignment = Alignment.Center,
            ) {
                if (index < pinLength) {
                    Box(
                        modifier = Modifier
                            .size(DotFillSize)
                            .background(colors.accent, CircleShape),
                    )
                }
            }
        }
    }
}
