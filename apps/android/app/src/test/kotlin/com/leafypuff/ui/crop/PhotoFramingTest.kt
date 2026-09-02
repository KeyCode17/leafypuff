package com.leafypuff.ui.crop

import kotlin.math.abs
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

private const val Tolerance = 1e-9
private const val WideTallness = 1920.0 / 1080.0
private const val TallTallness = 1200.0 / 1600.0

class PhotoFramingTest {
    @Test
    fun `a source wider than the cover cannot be framed at full width`() {
        val widest = PhotoFraming().widest(WideTallness)

        assertTrue(abs(widest - 0.84375) < Tolerance, "widest was $widest")
    }

    @Test
    fun `a source taller than the cover may be framed at full width`() {
        assertEquals(1.0, PhotoFraming().widest(TallTallness), Tolerance)
    }

    @Test
    fun `fitting a wide source narrows the framing so the crop fits`() {
        val fitted = PhotoFraming(x = 0.0, y = 0.0, width = 1.0).fitted(WideTallness)

        assertTrue(abs(fitted.width - 0.84375) < Tolerance, "width was ${fitted.width}")
        assertTrue(
            fitted.width * PhotoFraming.CoverTallness * WideTallness <= 1.0 + Tolerance,
            "the crop is taller than the photo",
        )
    }

    @Test
    fun `a fitted framing never reaches past the photo`() {
        val fitted = PhotoFraming(x = 0.9, y = 0.9, width = 0.5).fitted(WideTallness)

        assertTrue(fitted.x + fitted.width <= 1.0 + Tolerance, "x was ${fitted.x}")
        assertTrue(
            fitted.y + fitted.width * PhotoFraming.CoverTallness * WideTallness <= 1.0 + Tolerance,
            "y was ${fitted.y}",
        )
    }

    @Test
    fun `fitting is idempotent`() {
        val once = PhotoFraming(x = 0.3, y = 0.4, width = 1.0).fitted(WideTallness)

        assertEquals(once, once.fitted(WideTallness))
    }

    @Test
    fun `a square framing of a wide source is narrower still`() {
        val widest = PhotoFraming().widest(WideTallness, PhotoFraming.SquareTallness)

        assertTrue(abs(widest - 0.5625) < Tolerance, "widest was $widest")
    }

    @Test
    fun `panning right moves the framing right`() {
        val start = PhotoFraming(x = 0.0, y = 0.0, width = 0.5)

        val moved = start.panned(0.2, 0.0, WideTallness)

        assertEquals(0.2, moved.x, Tolerance)
    }
}
