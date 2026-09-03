package com.leafypuff.ui.lock

import androidx.compose.foundation.layout.size
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyElevation
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.ui.common.AppMark
import com.leafypuff.theme.LocalLeafyTypography


private const val AppName = "leafyPuff"
private val PlateSize = 132.dp

@Composable
internal fun LockMarkPlate(modifier: Modifier = Modifier) {
    AppMark(
        size = PlateSize,
        shape = LeafyShapes.lockIconPlate,
        elevation = LeafyElevation.plate,
        modifier = modifier,
    )
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
internal fun LockHint(text: String, modifier: Modifier = Modifier) {
    Text(
        text = text,
        style = LocalLeafyTypography.current.body,
        color = LocalLeafyColors.current.ink2,
        modifier = modifier,
    )
}
