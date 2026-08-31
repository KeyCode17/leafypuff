package com.leafypuff.ui.editor

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.slideInVertically
import androidx.compose.animation.slideOutVertically
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.ui.photo.EntryPhoto

private val ScreenGutter = 24.dp
private val ScrollBottomPadding = 210.dp

@Composable
fun EntryEditor(
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
    var tool by remember { mutableStateOf<EditorTool?>(null) }
    var lastTool by remember { mutableStateOf(EditorTool.Hashtag) }

    Box(
        modifier = modifier
            .fillMaxSize()
            .background(LocalLeafyColors.current.bg),
    ) {
        Column(
            modifier = Modifier
                .fillMaxSize()
                .verticalScroll(rememberScrollState())
                .padding(horizontal = ScreenGutter)
                .padding(bottom = ScrollBottomPadding),
        ) {
            EntryEditorHeader(
                title = if (draft.id == null) "New Entry" else "Edit Entry",
                onClose = onClose,
                onSave = onSave,
            )

            EntryMetaBlock(
                date = draft.date,
                mood = draft.mood,
                weather = draft.weather,
                location = draft.location,
                onDateClick = onDateClick,
                onMoodClick = onMoodClick,
                onWeatherClick = onWeatherClick,
                onLocationClick = onLocationClick,
            )

            EntryNoteCard(
                title = draft.title,
                body = draft.body,
                tags = draft.tags,
                photos = photos,
                onTitleChange = { onDraftChange(draft.copy(title = it)) },
                onBodyChange = { onDraftChange(draft.copy(body = it)) },
                onRemoveTag = { index ->
                    onDraftChange(
                        draft.copy(tags = draft.tags.filterIndexed { at, _ -> at != index }),
                    )
                },
            )
        }

        Column(
            modifier = Modifier
                .fillMaxWidth()
                .align(Alignment.BottomCenter),
        ) {
            AnimatedVisibility(
                visible = tool != null,
                enter = slideInVertically(
                    animationSpec = editorSlideSpec(),
                    initialOffsetY = { it },
                ),
                exit = slideOutVertically(
                    animationSpec = editorSlideSpec(),
                    targetOffsetY = { it },
                ),
            ) {
                when (lastTool) {
                    EditorTool.Sticker -> StickerTrayDrawer()
                    EditorTool.Hashtag -> HashtagPanel(
                        selected = draft.tags,
                        onAddTag = { tag -> onDraftChange(draft.copy(tags = draft.tags + tag)) },
                    )
                }
            }

            EntryToolbar(
                tool = tool,
                onToggleTool = { picked ->
                    tool = if (tool == picked) null else picked
                    lastTool = picked
                },
                onAddPhoto = onAddPhoto,
            )
        }
    }
}
