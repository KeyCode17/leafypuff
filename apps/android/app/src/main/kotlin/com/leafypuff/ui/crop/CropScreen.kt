package com.leafypuff.ui.crop

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.gestures.detectTransformGestures
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.unit.IntOffset
import androidx.compose.ui.unit.IntSize
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.auth.PrimaryCta

private val TopPadding = 68.dp
private val SidePadding = 24.dp
private val BlockGap = 18.dp
private const val CoverRatio = 3f / 2f

@Composable
fun CropScreen(
    photo: ImageBitmap?,
    framing: PhotoFraming,
    pending: Boolean,
    onFramingChange: (PhotoFraming) -> Unit,
    onSubmit: () -> Unit,
    onBack: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current
    val tallness = photo?.let { it.width.toDouble() / it.height.toDouble() } ?: 1.0

    Column(
        modifier = modifier
            .fillMaxSize()
            .background(colors.bg)
            .padding(start = SidePadding, top = TopPadding, end = SidePadding),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(BlockGap),
    ) {
        Text(
            text = "Frame the thumbnail",
            style = typography.authTitle,
            color = colors.ink,
            modifier = Modifier.fillMaxWidth(),
        )
        Text(
            text = "Drag to move it, pinch to change how much it holds. This is what the diary " +
                "and the calendar show.",
            style = typography.body,
            color = colors.ink2,
            modifier = Modifier.fillMaxWidth(),
        )

        Box(
            modifier = Modifier
                .fillMaxWidth()
                .aspectRatio(CoverRatio)
                .clip(LeafyShapes.card)
                .background(colors.soft2)
                .pointerInput(tallness) {
                    detectTransformGestures { _, pan, zoom, _ ->
                        val moved = framing.panned(
                            acrossFraction = -pan.x.toDouble() / size.width * framing.width,
                            downFraction = -pan.y.toDouble() / size.height *
                                framing.height(tallness),
                            tallness = tallness,
                        )
                        onFramingChange(moved.zoomed(zoom.toDouble(), tallness))
                    }
                },
        ) {
            if (photo != null) {
                Canvas(modifier = Modifier.fillMaxSize()) {
                    val across = (photo.width * framing.width).toInt().coerceAtLeast(1)
                    val down = (across * 2 / 3).coerceAtLeast(1).coerceAtMost(photo.height)
                    val left = (photo.width * framing.x).toInt()
                        .coerceIn(0, (photo.width - across).coerceAtLeast(0))
                    val top = (photo.height * framing.y).toInt()
                        .coerceIn(0, (photo.height - down).coerceAtLeast(0))
                    drawImage(
                        image = photo,
                        srcOffset = IntOffset(left, top),
                        srcSize = IntSize(across, down),
                        dstSize = IntSize(size.width.toInt(), size.height.toInt()),
                    )
                }
            }
        }

        PrimaryCta(
            label = if (pending) "SAVING…" else "USE THIS FRAME",
            enabled = !pending,
            onClick = onSubmit,
        )
        Text(
            text = "Back",
            style = typography.chipLabel,
            color = colors.ink2,
            modifier = Modifier.clickable(onClick = onBack),
        )
    }
}
