package com.leafypuff.ui.common

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.size
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Shape
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.unit.Dp
import com.leafypuff.R
import com.leafypuff.theme.MarkPlate

private const val AppName = "leafyPuff"
private const val MarkShare = 0.82f

@Composable
fun AppMark(size: Dp, shape: Shape, elevation: Dp, modifier: Modifier = Modifier) {
    Box(
        modifier = modifier
            .size(size)
            .shadow(elevation, shape)
            .clip(shape)
            .background(MarkPlate),
        contentAlignment = Alignment.Center,
    ) {
        Image(
            painter = painterResource(R.drawable.leafypuff_mark),
            contentDescription = AppName,
            contentScale = ContentScale.Fit,
            modifier = Modifier.size(size * MarkShare),
        )
    }
}
