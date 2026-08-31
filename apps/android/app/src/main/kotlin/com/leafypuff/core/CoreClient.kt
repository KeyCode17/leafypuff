package com.leafypuff.core

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.datetime.Clock
import kotlinx.datetime.LocalDate

data class PhotoDraft(val id: String, val path: String, val ordinal: Int)

data class StickerDraft(
    val key: String,
    val sticker: Sticker,
    val x: Float,
    val y: Float,
    val size: Float,
    val rotation: Float,
)

data class EntryDraft(
    val id: String,
    val date: LocalDate,
    val mood: Mood,
    val title: String,
    val body: String,
    val tags: List<String>,
    val weather: Weather?,
    val location: Location?,
    val photos: List<PhotoDraft>,
    val stickers: List<StickerDraft>,
)

class CoreClient private constructor(private val core: LeafyPuffCore) {

    suspend fun save(draft: EntryDraft): EntryDraft = withContext(Dispatchers.IO) {
        core.saveEntry(draft.toRecord()).toDraft()
    }

    suspend fun entryById(id: String): EntryDraft? = withContext(Dispatchers.IO) {
        core.entryById(id)?.toDraft()
    }

    suspend fun list(limit: UInt): List<EntryDraft> = withContext(Dispatchers.IO) {
        core.listEntries(limit).map { it.toDraft() }
    }

    suspend fun deleteAll() = withContext(Dispatchers.IO) {
        core.deleteAllEntries()
    }

    companion object {
        suspend fun open(dbPath: String): CoreClient = withContext(Dispatchers.IO) {
            CoreClient(LeafyPuffCore.open(dbPath))
        }
    }
}

private fun EntryDraft.toRecord(): FfiEntry {
    val stamp = Clock.System.now().toString()
    return FfiEntry(
        id = id,
        date = date.toString(),
        mood = mood,
        title = title,
        body = body,
        tags = tags,
        weather = weather,
        location = location,
        photos = photos.map { FfiPhoto(it.id, it.path, it.ordinal, null) },
        stickers = stickers.map {
            FfiPlacedSticker(it.key, it.sticker, it.x, it.y, it.size, it.rotation)
        },
        createdAt = stamp,
        updatedAt = stamp,
    )
}

private fun FfiEntry.toDraft(): EntryDraft = EntryDraft(
    id = id,
    date = LocalDate.parse(date),
    mood = mood,
    title = title,
    body = body,
    tags = tags,
    weather = weather,
    location = location,
    photos = photos.map { PhotoDraft(it.id, it.path, it.ordinal) },
    stickers = stickers.map {
        StickerDraft(it.key, it.sticker, it.x, it.y, it.size, it.rotation)
    },
)
