package com.leafypuff.ui.common

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.core.CubicBezierEasing
import androidx.compose.animation.core.tween
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.animation.slideInVertically
import androidx.compose.animation.slideOutVertically
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.leafypuff.theme.Inter
import com.leafypuff.theme.LocalLeafyColors
import kotlinx.coroutines.delay

private val ToastEasing = CubicBezierEasing(0.4f, 0f, 0.2f, 1f)
private const val SlideMillis = 260
private const val PlainDismissMillis = 2500L
private val ToastTop = 620.dp
private val ToastGutter = 20.dp
private val ToastShape = RoundedCornerShape(20.dp)
private val ToastElevation = 12.dp
private val ToastPaddingX = 16.dp
private val ToastPaddingY = 14.dp
private val ToastGap = 12.dp
private val PillGap = 8.dp
private val RiseFrom = 24.dp
private val RejectFill = Color.White.copy(alpha = 0.14f)

private val ToastTextStyle = TextStyle(
    fontFamily = Inter,
    fontWeight = FontWeight.W400,
    fontSize = 13.sp,
    lineHeight = 18.85.sp,
)

@Composable
fun ToastOverlay(
    visible: Boolean,
    request: ToastRequest?,
    onAccept: () -> Unit,
    onDismiss: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val rise = with(LocalDensity.current) { RiseFrom.roundToPx() }

    if (visible && request != null && request.prompt == null) {
        LaunchedEffect(request) {
            delay(PlainDismissMillis)
            onDismiss()
        }
    }

    Box(modifier = modifier.fillMaxSize()) {
        AnimatedVisibility(
            visible = visible && request != null,
            modifier = Modifier
                .align(Alignment.TopStart)
                .fillMaxWidth()
                .padding(top = ToastTop, start = ToastGutter, end = ToastGutter),
            enter = slideInVertically(tween(SlideMillis, easing = ToastEasing)) { rise } +
                fadeIn(tween(SlideMillis, easing = ToastEasing)),
            exit = slideOutVertically(tween(SlideMillis, easing = ToastEasing)) { rise } +
                fadeOut(tween(SlideMillis, easing = ToastEasing)),
        ) {
            if (request != null) {
                ToastCard(request = request, onAccept = onAccept, onDismiss = onDismiss)
            }
        }
    }
}

@Composable
private fun ToastCard(request: ToastRequest, onAccept: () -> Unit, onDismiss: () -> Unit) {
    val colors = LocalLeafyColors.current

    Column(
        modifier = Modifier
            .fillMaxWidth()
            .shadow(ToastElevation, ToastShape)
            .clip(ToastShape)
            .background(colors.ink)
            .padding(horizontal = ToastPaddingX, vertical = ToastPaddingY),
        verticalArrangement = Arrangement.spacedBy(ToastGap),
    ) {
        Text(text = request.text, style = ToastTextStyle, color = colors.bg)

        if (request.prompt != null) {
            Row(horizontalArrangement = Arrangement.spacedBy(PillGap)) {
                PromptPill(
                    label = request.prompt.accept,
                    fill = colors.accent,
                    ink = colors.onAccent,
                    weight = FontWeight.W600,
                    onClick = onAccept,
                    modifier = Modifier.weight(1f),
                )
                PromptPill(
                    label = request.prompt.reject,
                    fill = RejectFill,
                    ink = colors.bg,
                    weight = FontWeight.W500,
                    onClick = onDismiss,
                    modifier = Modifier.weight(1f),
                )
            }
        }
    }
}
