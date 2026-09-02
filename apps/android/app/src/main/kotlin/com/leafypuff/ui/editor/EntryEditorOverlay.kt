package com.leafypuff.ui.editor

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.core.CubicBezierEasing
import androidx.compose.animation.core.FiniteAnimationSpec
import androidx.compose.animation.core.tween
import androidx.compose.animation.slideInVertically
import androidx.compose.animation.slideOutVertically
import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.IntOffset
import com.leafypuff.ui.photo.EntryPhoto
import com.leafypuff.ui.photo.PhotoPlacement
import com.leafypuff.ui.settings.StickerPack

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
    onRemovePhoto: ((String) -> Unit)? = null,
    onFramePhoto: ((String) -> Unit)? = null,
    onMakeCover: ((String) -> Unit)? = null,
    onPlaceFreely: ((String) -> Unit)? = null,
    onPlacementChange: ((String, PhotoPlacement) -> Unit)? = null,
    onPutBack: ((String) -> Unit)? = null,
    onCropPlaced: ((String) -> Unit)? = null,
    originals: Map<String, ImageBitmap> = emptyMap(),
    stickerPack: StickerPack = StickerPack.Mixed,
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
            onRemovePhoto = onRemovePhoto,
            onFramePhoto = onFramePhoto,
            onMakeCover = onMakeCover,
            onPlaceFreely = onPlaceFreely,
            onPlacementChange = onPlacementChange,
            onPutBack = onPutBack,
            onCropPlaced = onCropPlaced,
            originals = originals,
            stickerPack = stickerPack,
        )
    }
}
