package com.leafypuff.core

import java.nio.file.Files
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlinx.coroutines.runBlocking
import kotlinx.datetime.LocalDate

private const val VaultPassphrase = "a passphrase only this test uses"

class RoundTripTest {

    @Test
    fun anEntrySurvivesTheBoundaryUnchanged() = runBlocking {
        val dir = Files.createTempDirectory("leafypuff")
        val client = CoreClient.open(dir.resolve("diary.sqlite").toString()).also { it.createVault(VaultPassphrase) }

        val draft = EntryDraft(
            id = "11111111-1111-4111-8111-111111111111",
            date = LocalDate.parse("2026-08-31"),
            mood = Mood.LOVED,
            title = "Quiet morning",
            body = "Tea on the balcony.",
            tags = listOf("#slowday", "#tea"),
            weather = Weather.SUNNY,
            location = Location.HOME,
            photos = listOf(PhotoDraft("photo-1", "/photos/one.jpg", 0)),
            stickers = listOf(
                StickerDraft("sticker-1", Sticker.BUN_SLEEP, 12.5f, 48.25f, 64.0f, 90.0f),
            ),
        )

        client.save(draft)
        val read = assertNotNull(client.entryById(draft.id))

        assertEquals(draft.id, read.id)
        assertEquals(draft.date, read.date)
        assertEquals(draft.mood, read.mood)
        assertEquals(draft.title, read.title)
        assertEquals(draft.body, read.body)
        assertEquals(draft.tags, read.tags)
        assertEquals(draft.weather, read.weather)
        assertEquals(draft.location, read.location)
        assertEquals(draft.photos, read.photos)
        assertEquals(draft.stickers, read.stickers)
    }
}
