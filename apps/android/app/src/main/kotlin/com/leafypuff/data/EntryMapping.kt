package com.leafypuff.data

import com.leafypuff.core.EntryDraft as CoreDraft
import com.leafypuff.core.PhotoDraft
import com.leafypuff.domain.Entry
import com.leafypuff.ui.editor.EntryDraft as UiDraft

fun CoreDraft.toEntry(): Entry = Entry(
    id = id,
    date = date,
    mood = mood.toDomain(),
    title = title,
    body = body,
    tags = tags,
)

fun CoreDraft.toUiDraft(): UiDraft = UiDraft(
    id = id,
    date = date,
    mood = mood.toDomain(),
    title = title,
    body = body,
    tags = tags,
    weather = weather?.label(),
    location = location?.label(),
)

fun UiDraft.toCoreDraft(photoIds: List<String>): CoreDraft = CoreDraft(
    id = id.orEmpty(),
    date = date,
    mood = mood.toCore(),
    title = title,
    body = body,
    tags = tags,
    weather = weatherFromLabel(weather),
    location = locationFromLabel(location),
    photos = photoIds.mapIndexed { index, photoId ->
        PhotoDraft(id = photoId, path = "", ordinal = index)
    },
    stickers = emptyList(),
)
