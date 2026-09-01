package com.leafypuff.ui.editor

import androidx.compose.runtime.Immutable
import com.leafypuff.domain.Mood
import com.leafypuff.ui.editor.sticker.PlacedSticker
import kotlinx.datetime.LocalDate

@Immutable
data class EntryDraft(
    val id: String?,
    val date: LocalDate,
    val mood: Mood,
    val title: String,
    val body: String,
    val tags: List<String>,
    val weather: String?,
    val location: String?,
    val stickers: List<PlacedSticker> = emptyList(),
)
