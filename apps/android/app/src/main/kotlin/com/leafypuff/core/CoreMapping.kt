package com.leafypuff.core

import kotlinx.datetime.Clock
import kotlinx.datetime.Instant
import kotlinx.datetime.LocalDate
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime

internal fun EntryDraft.toRecord(): FfiEntry {
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

internal fun FfiEntry.toDraft(): EntryDraft = EntryDraft(
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

internal fun FfiChallenge.toChallenge(): MailedChallenge =
    MailedChallenge(expiresInSeconds = expiresInSeconds)

internal fun FfiSession.toIssued(): IssuedSession = IssuedSession(
    accessToken = accessToken,
    refreshToken = refreshToken,
    expiresInSeconds = expiresInSeconds,
)

internal fun FfiPhoto.toImported(): ImportedPhoto = ImportedPhoto(
    id = id,
    path = path,
    takenOn = takenAt?.let { Instant.parse(it).toLocalDateTime(TimeZone.UTC).date },
)

internal fun FfiProfile.toStored(): StoredProfile = StoredProfile(
    displayName = displayName,
    avatarPhotoId = avatarPhotoId,
    updatedAtMs = updatedAtMs,
)
