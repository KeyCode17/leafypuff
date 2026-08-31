package com.leafypuff.ui.editor

import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import com.leafypuff.domain.Mood
import com.leafypuff.ui.mood.MoodPickerOverlay
import com.leafypuff.ui.photo.EntryPhoto
import com.leafypuff.ui.photo.PhotoImporter
import com.leafypuff.ui.photo.rememberPhotoPicker
import kotlinx.coroutines.launch
import kotlinx.datetime.LocalDate

@Composable
fun EntryComposer(
    open: Boolean,
    today: LocalDate,
    importer: PhotoImporter,
    onClose: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val scope = rememberCoroutineScope()
    val blocker = remember { MutableInteractionSource() }
    var draft by remember { mutableStateOf(blankDraft(today)) }
    var editing by remember { mutableStateOf(false) }
    var photos by remember { mutableStateOf(emptyList<EntryPhoto>()) }

    val addPhoto = rememberPhotoPicker { bytes ->
        scope.launch {
            val picked = importer.import(bytes) ?: return@launch
            photos = photos + picked
        }
    }

    LaunchedEffect(open) {
        if (open) {
            draft = blankDraft(today)
            editing = false
            photos = emptyList()
        }
    }

    Box(
        modifier = if (open) {
            modifier.clickable(interactionSource = blocker, indication = null, onClick = { })
        } else {
            modifier
        },
    ) {
        MoodPickerOverlay(
            visible = open && !editing,
            entryDate = draft.date,
            onPick = { mood ->
                draft = draft.copy(mood = mood)
                editing = true
            },
            onClose = onClose,
            onDateClick = { },
            modifier = Modifier.statusBarsPadding(),
        )

        EntryEditorOverlay(
            visible = open && editing,
            draft = draft,
            onDraftChange = { draft = it },
            onSave = onClose,
            onClose = onClose,
            onMoodClick = { editing = false },
            onDateClick = { },
            onWeatherClick = { },
            onLocationClick = { },
            onAddPhoto = addPhoto,
            photos = photos,
            modifier = Modifier.statusBarsPadding(),
        )
    }
}

private fun blankDraft(today: LocalDate): EntryDraft = EntryDraft(
    id = null,
    date = today,
    mood = Mood.Happy,
    title = "",
    body = "",
    tags = emptyList(),
    weather = "Sunny",
    location = "Home",
)
