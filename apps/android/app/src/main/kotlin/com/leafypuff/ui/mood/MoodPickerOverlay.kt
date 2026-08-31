package com.leafypuff.ui.mood

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.core.CubicBezierEasing
import androidx.compose.animation.core.tween
import androidx.compose.animation.slideInVertically
import androidx.compose.animation.slideOutVertically
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import com.leafypuff.domain.Mood
import kotlinx.datetime.LocalDate

private val SheetEasing = CubicBezierEasing(0.4f, 0f, 0.2f, 1f)
private const val SlideUpMillis = 280
private const val SlideDownMillis = 250

@Composable
fun MoodPickerOverlay(
    visible: Boolean,
    entryDate: LocalDate,
    onPick: (Mood) -> Unit,
    onClose: () -> Unit,
    onDateClick: () -> Unit,
    modifier: Modifier = Modifier,
) {
    AnimatedVisibility(
        visible = visible,
        modifier = modifier,
        enter = slideInVertically(
            animationSpec = tween(SlideUpMillis, easing = SheetEasing),
            initialOffsetY = { it },
        ),
        exit = slideOutVertically(
            animationSpec = tween(SlideDownMillis, easing = SheetEasing),
            targetOffsetY = { it },
        ),
    ) {
        MoodPicker(
            entryDate = entryDate,
            onPick = onPick,
            onClose = onClose,
            onDateClick = onDateClick,
        )
    }
}
