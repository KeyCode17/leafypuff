package com.leafypuff.ui.editor

import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.tooling.preview.Preview
import com.leafypuff.domain.Mood
import com.leafypuff.theme.LeafyTheme
import kotlinx.datetime.LocalDate

private const val FrameWidth = 375
private const val FrameHeight = 812
private val PreviewDate = LocalDate(2026, 9, 1)

private val NewDraft = EntryDraft(
    id = null,
    date = PreviewDate,
    mood = Mood.Happy,
    title = "",
    body = "",
    tags = emptyList(),
    weather = "Sunny",
    location = "Home",
)

private val EditDraft = EntryDraft(
    id = "e-2",
    date = PreviewDate,
    mood = Mood.Calm,
    title = "Slow Sunday",
    body = "Woke up late and let the morning stay quiet. Made toast, watered the plant " +
        "on the sill, read four pages before the phone found me.",
    tags = listOf("#slowday", "#home"),
    weather = "Cloudy",
    location = "Home",
)

@Preview(name = "New entry on light", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun NewEntryLightPreview() {
    EditorFrame(dark = false, initial = NewDraft, photos = emptyList())
}

@Preview(name = "Edit entry on light", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun EditEntryLightPreview() {
    EditorFrame(dark = false, initial = EditDraft, photos = listOf("photo-1", "photo-2"))
}

@Preview(name = "Edit entry on dark", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun EditEntryDarkPreview() {
    EditorFrame(dark = true, initial = EditDraft, photos = listOf("photo-1", "photo-2"))
}

@Composable
private fun EditorFrame(dark: Boolean, initial: EntryDraft, photos: List<String>) {
    LeafyTheme(darkOverride = dark) {
        var draft by remember { mutableStateOf(initial) }
        EntryEditorOverlay(
            visible = true,
            draft = draft,
            onDraftChange = { draft = it },
            onSave = { },
            onClose = { },
            onMoodClick = { },
            onDateClick = { },
            onWeatherClick = { },
            onLocationClick = { },
            onAddPhoto = { },
            photos = photos,
        )
    }
}
