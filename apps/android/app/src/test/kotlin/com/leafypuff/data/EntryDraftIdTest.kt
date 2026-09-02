package com.leafypuff.data

import com.leafypuff.ui.editor.EntryDraft
import java.util.UUID
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotEquals
import kotlinx.datetime.LocalDate

private fun draft(id: String?): EntryDraft = EntryDraft(
    id = id,
    date = LocalDate(2026, 9, 2),
    mood = com.leafypuff.domain.Mood.Calm,
    title = "a title",
    body = "a body",
    tags = emptyList(),
    weather = "Sunny",
    location = "Home",
)

class EntryDraftIdTest {
    @Test
    fun aNewDraftReachesTheCoreWithAnIdItCanParse() {
        val carried = draft(null).toCoreDraft(emptyList()).id

        UUID.fromString(carried)
        assertNotEquals(UUID(0, 0).toString(), carried)
    }

    @Test
    fun anExistingDraftKeepsTheIdItAlreadyHad() {
        val held = UUID.randomUUID().toString()

        assertEquals(held, draft(held).toCoreDraft(emptyList()).id)
    }
}
