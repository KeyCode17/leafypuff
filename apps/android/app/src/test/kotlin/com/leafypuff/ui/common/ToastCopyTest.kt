package com.leafypuff.ui.common

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNull
import kotlinx.datetime.LocalDate

class ToastCopyTest {

    @Test
    fun theExifPromptReadsExactlyAsTheHandoffWroteIt() {
        assertEquals(
            "This photo was taken on 4 Jul 2026. Use that as the entry date?",
            exifPromptText(LocalDate(2026, 7, 4)),
        )
    }

    @Test
    fun theExifPromptCarriesBothAnswers() {
        val request = exifPromptToast(LocalDate(2026, 12, 25))

        assertEquals("This photo was taken on 25 Dec 2026. Use that as the entry date?", request.text)
        assertEquals("Accept", request.prompt?.accept)
        assertEquals("Reject", request.prompt?.reject)
    }

    @Test
    fun aPlainToastCarriesNoAnswers() {
        assertNull(plainToast("Entry saved.").prompt)
    }

    @Test
    fun theShortDateDropsTheWeekdayThatTheCardKeeps() {
        val day = LocalDate(2026, 1, 9)

        assertEquals("9 Jan 2026", formatShortDate(day))
        assertEquals("Friday, 9 Jan 2026", formatEntryDate(day))
    }
}
