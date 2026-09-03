package com.leafypuff.ui.crop

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.gestures.detectTransformGestures
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material3.Slider
import androidx.compose.material3.SliderDefaults
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.rememberUpdatedState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.unit.IntOffset
import androidx.compose.ui.unit.IntSize
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.auth.PrimaryCta
import kotlin.math.abs
import kotlin.math.ln
import kotlin.math.pow

private val TopPadding = 68.dp
private val SidePadding = 24.dp
private val BlockGap = 18.dp
private val PreviewMaxHeight = 380.dp

@Composable
fun CropScreen(
    photo: ImageBitmap?,
    framing: PhotoFraming,
    pending: Boolean,
    title: String = "Frame the thumbnail",
    blurb: String = "Drag to move it, pinch to change how much it holds. This is what the diary " +
        "and the calendar show.",
    ratio: Double = PhotoFraming.CoverTallness,
    round: Boolean = false,
    adjustableRatio: Boolean = false,
    onRatioChange: (Double) -> Unit = { },
    onFramingChange: (PhotoFraming) -> Unit,
    onSubmit: () -> Unit,
    onBack: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current
    val tallness = photo?.let { it.width.toDouble() / it.height.toDouble() } ?: 1.0
    val held by rememberUpdatedState(framing)
    val report by rememberUpdatedState(onFramingChange)
    val focusManager = LocalFocusManager.current

    LaunchedEffect(Unit) {
        focusManager.clearFocus()
    }

    LaunchedEffect(tallness, ratio) {
        val fitted = held.fitted(tallness, ratio)
        if (fitted != held) {
            report(fitted)
        }
    }

    Column(
        modifier = modifier
            .fillMaxSize()
            .background(colors.bg)
            .padding(start = SidePadding, top = TopPadding, end = SidePadding),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(BlockGap),
    ) {
        Text(
            text = title,
            style = typography.authTitle,
            color = colors.ink,
            modifier = Modifier.fillMaxWidth(),
        )
        Text(
            text = blurb,
            style = typography.body,
            color = colors.ink2,
            modifier = Modifier.fillMaxWidth(),
        )

        Box(
            modifier = Modifier
                .heightIn(max = PreviewMaxHeight)
                .aspectRatio((1.0 / ratio).toFloat())
                .clip(if (round) CircleShape else LeafyShapes.card)
                .background(colors.soft2)
                .pointerInput(tallness) {
                    detectTransformGestures { _, pan, zoom, _ ->
                        val standing = held
                        val moved = standing.panned(
                            acrossFraction = -pan.x.toDouble() / size.width * standing.width,
                            downFraction = -pan.y.toDouble() / size.height *
                                standing.height(tallness, ratio),
                            tallness = tallness,
                            ratio = ratio,
                        )
                        report(moved.zoomed(zoom.toDouble(), tallness, ratio))
                    }
                },
        ) {
            if (photo != null) {
                Canvas(modifier = Modifier.fillMaxSize()) {
                    val across = (photo.width * framing.width).toInt().coerceAtLeast(1)
                    val down = (across * ratio).toInt().coerceAtLeast(1).coerceAtMost(photo.height)
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

        if (adjustableRatio) {
            RatioPicker(ratio = ratio, onRatioChange = onRatioChange)
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

private val RatioShapes = listOf(
    "1:1" to 1.0,
    "4:5" to 5.0 / 4.0,
    "3:2" to 2.0 / 3.0,
    "2:3" to 3.0 / 2.0,
    "16:9" to 9.0 / 16.0,
    "9:16" to 16.0 / 9.0,
)
private const val FarthestRatio = 3.0
private const val RatioSnap = 0.02
private val ChipGap = 8.dp
private val ChipPaddingH = 12.dp
private val ChipPaddingV = 6.dp

@Composable
private fun RatioPicker(ratio: Double, onRatioChange: (Double) -> Unit) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Column(
        modifier = Modifier.fillMaxWidth(),
        verticalArrangement = Arrangement.spacedBy(ChipGap),
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .horizontalScroll(rememberScrollState()),
            horizontalArrangement = Arrangement.spacedBy(ChipGap),
        ) {
            RatioShapes.forEach { (label, shape) ->
                val chosen = abs(shape - ratio) < RatioSnap
                Text(
                    text = label,
                    style = typography.chipLabel,
                    color = if (chosen) colors.bg else colors.ink,
                    modifier = Modifier
                        .clip(LeafyShapes.pill)
                        .background(if (chosen) colors.accentDeep else colors.soft2)
                        .clickable { onRatioChange(shape) }
                        .padding(horizontal = ChipPaddingH, vertical = ChipPaddingV),
                )
            }
        }
        Slider(
            value = ratioToSlide(ratio),
            onValueChange = { onRatioChange(slideToRatio(it)) },
            valueRange = -1f..1f,
            colors = SliderDefaults.colors(
                thumbColor = colors.accentDeep,
                activeTrackColor = colors.accent,
                inactiveTrackColor = colors.soft2,
            ),
        )
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
        ) {
            Text(text = "Landscape", style = typography.chipLabel, color = colors.ink3)
            Text(text = shapeLabel(ratio), style = typography.chipLabel, color = colors.ink)
            Text(text = "Portrait", style = typography.chipLabel, color = colors.ink3)
        }
    }
}

private fun ratioToSlide(ratio: Double): Float =
    (ln(ratio) / ln(FarthestRatio)).toFloat().coerceIn(-1f, 1f)

private fun slideToRatio(slide: Float): Double = FarthestRatio.pow(slide.toDouble())

private fun shapeLabel(ratio: Double): String {
    val named = RatioShapes.firstOrNull { (_, shape) -> abs(shape - ratio) < RatioSnap }
    if (named != null) {
        return named.first
    }
    return when {
        ratio < 1.0 -> "%.2f:1".format(1.0 / ratio)
        else -> "1:%.2f".format(ratio)
    }
}
