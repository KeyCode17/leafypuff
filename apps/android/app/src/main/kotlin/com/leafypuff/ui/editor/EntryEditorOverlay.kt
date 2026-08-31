package com.leafypuff.ui.editor

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.core.CubicBezierEasing
import androidx.compose.animation.core.FiniteAnimationSpec
import androidx.compose.animation.core.tween
import androidx.compose.animation.slideInVertically
import androidx.compose.animation.slideOutVertically
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.IntOffset
import com.leafypuff.ui.photo.EntryPhoto

private val SheetEasing = CubicBezierEasing(0.4f, 0f, 0.2f, 1f)
private const val SlideMillis = 260

fun editorSlideSpec(): FiniteAnimationSpec<IntOffset> = tween(SlideMillis, easing = SheetEasing)

@Composable
fun EntryEditorOverlay(
    visible: Boolean,
    draft: EntryDraft,
    onDraftChange: (EntryDraft) -> Unit,
    onSave: () -> Unit,
    onClose: () -> Unit,
    onMoodClick: () -> Unit,
    onDateClick: () -> Unit,
    onWeatherClick: () -> Unit,
    onLocationClick: () -> Unit,
    onAddPhoto: () -> Unit,
    photos: List<EntryPhoto> = emptyList(),
    modifier: Modifier = Modifier,
) {
    AnimatedVisibility(
        visible = visible,
        modifier = modifier,
        enter = slideInVertically(
            animationSpec = editorSlideSpec(),
            initialOffsetY = { it },
        ),
        exit = slideOutVertically(
            animationSpec = editorSlideSpec(),
            targetOffsetY = { it },
        ),
    ) {
        EntryEditor(
            draft = draft,
            onDraftChange = onDraftChange,
            onSave = onSave,
            onClose = onClose,
            onMoodClick = onMoodClick,
            onDateClick = onDateClick,
            onWeatherClick = onWeatherClick,
            onLocationClick = onLocationClick,
            onAddPhoto = onAddPhoto,
            photos = photos,
        )
    }
}
