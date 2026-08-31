package com.leafypuff.ui.lock

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.size
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.unit.dp
import com.leafypuff.R
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

private const val AppName = "leafyPuff"

private val PlateColor = Color(0xFFF7FAEF)
private val PlateSize = 132.dp
private val PlateElevation = 12.dp
private val MarkSize = 108.dp

private const val HintEmpty = "Enter your PIN"
private const val HintTyping = "Keep going"

@Composable
internal fun LockMarkPlate(modifier: Modifier = Modifier) {
    Box(
        modifier = modifier
            .size(PlateSize)
            .shadow(PlateElevation, LeafyShapes.lockIconPlate)
            .clip(LeafyShapes.lockIconPlate)
            .background(PlateColor),
        contentAlignment = Alignment.Center,
    ) {
        Image(
            painter = painterResource(R.drawable.leafypuff_mark),
            contentDescription = AppName,
            contentScale = ContentScale.Fit,
            modifier = Modifier.size(MarkSize),
        )
    }
}

@Composable
internal fun LockTitle(modifier: Modifier = Modifier) {
    Text(
        text = AppName,
        style = LocalLeafyTypography.current.lockTitle,
        color = LocalLeafyColors.current.ink,
        modifier = modifier,
    )
}

@Composable
internal fun LockHint(pinLength: Int, modifier: Modifier = Modifier) {
    Text(
        text = if (pinLength > 0) HintTyping else HintEmpty,
        style = LocalLeafyTypography.current.body,
        color = LocalLeafyColors.current.ink2,
        modifier = modifier,
    )
}
