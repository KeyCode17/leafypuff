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
import com.leafypuff.ui.common.ToastOverlay
import com.leafypuff.ui.common.ToastRequest
import com.leafypuff.ui.common.exifPromptToast
import com.leafypuff.ui.mood.MoodPickerOverlay
import com.leafypuff.ui.photo.EntryPhoto
import com.leafypuff.ui.photo.PhotoLibrary
import com.leafypuff.ui.photo.rememberPhotoPicker
import com.leafypuff.ui.popups.DatePopup
import com.leafypuff.ui.popups.LocationOptions
import com.leafypuff.ui.popups.OptionPopup
import com.leafypuff.ui.popups.WeatherOptions
import com.leafypuff.ui.settings.StickerPack
import kotlinx.coroutines.launch
import kotlinx.datetime.LocalDate

private enum class MetaPopup { Date, Weather, Location }

@Composable
fun EntryComposer(
    open: Boolean,
    today: LocalDate,
    library: PhotoLibrary,
    onClose: () -> Unit,
    onSave: (EntryDraft, List<String>) -> Unit,
    modifier: Modifier = Modifier,
    existing: EntryDraft? = null,
    existingPhotos: List<EntryPhoto> = emptyList(),
    stickerPack: StickerPack = StickerPack.Mixed,
) {
    val scope = rememberCoroutineScope()
    val blocker = remember { MutableInteractionSource() }
    var draft by remember { mutableStateOf(blankDraft(today)) }
    var editing by remember { mutableStateOf(false) }
    var photos by remember { mutableStateOf(emptyList<EntryPhoto>()) }
    var toast by remember { mutableStateOf<ToastRequest?>(null) }
    var asking by remember { mutableStateOf(false) }
    var promptedDay by remember { mutableStateOf<LocalDate?>(null) }
    var popup by remember { mutableStateOf<MetaPopup?>(null) }

    val addPhoto = rememberPhotoPicker { bytes ->
        scope.launch {
            val picked = library.import(bytes) ?: return@launch
            photos = photos + picked
            val day = picked.takenOn
            if (day != null) {
                promptedDay = day
                toast = exifPromptToast(day)
                asking = true
            }
        }
    }

    LaunchedEffect(open, existing) {
        if (open) {
            draft = existing ?: blankDraft(today)
            // An entry that already exists has already had its mood chosen; reopening it should
            // land on the note, not send the writer back through the mood deck.
            editing = existing != null
            photos = existingPhotos
            asking = false
            popup = null
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
            onDateClick = { popup = MetaPopup.Date },
            modifier = Modifier.statusBarsPadding(),
        )

        EntryEditorOverlay(
            visible = open && editing,
            draft = draft,
            onDraftChange = { draft = it },
            onSave = { onSave(draft, photos.map { photo -> photo.id }) },
            onClose = onClose,
            onMoodClick = { editing = false },
            onDateClick = { popup = MetaPopup.Date },
            onWeatherClick = { popup = MetaPopup.Weather },
            onLocationClick = { popup = MetaPopup.Location },
            onAddPhoto = addPhoto,
            photos = photos,
            stickerPack = stickerPack,
            modifier = Modifier.statusBarsPadding(),
        )

        if (open) {
            MetaPopupHost(
                popup = popup,
                draft = draft,
                onDraftChange = { draft = it },
                onDismiss = { popup = null },
            )
        }

        ToastOverlay(
            visible = open && asking,
            request = toast,
            onAccept = {
                promptedDay?.let { day -> draft = draft.copy(date = day) }
                asking = false
            },
            onDismiss = { asking = false },
        )
    }
}

@Composable
private fun MetaPopupHost(
    popup: MetaPopup?,
    draft: EntryDraft,
    onDraftChange: (EntryDraft) -> Unit,
    onDismiss: () -> Unit,
) {
    when (popup) {
        null -> Unit

        MetaPopup.Date -> DatePopup(
            selected = draft.date,
            onSelect = {
                onDraftChange(draft.copy(date = it))
                onDismiss()
            },
            onDismiss = onDismiss,
        )

        MetaPopup.Weather -> OptionPopup(
            title = "Weather",
            options = WeatherOptions,
            selected = draft.weather,
            onSelect = {
                onDraftChange(draft.copy(weather = it))
                onDismiss()
            },
            onDismiss = onDismiss,
        )

        MetaPopup.Location -> OptionPopup(
            title = "Location",
            options = LocationOptions,
            selected = draft.location,
            onSelect = {
                onDraftChange(draft.copy(location = it))
                onDismiss()
            },
            onDismiss = onDismiss,
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
