package com.leafypuff.ui.photo

import com.leafypuff.ui.crop.PhotoFraming
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNull

private const val Tolerance = 0.001f

class PhotoPlacementTest {
    @Test
    fun `a placement without a crop is square, as every placement was before crops existed`() {
        val placed = PhotoPlacement(x = 0.1f, y = 0.2f, size = 168f, rotation = 0f)

        assertNull(placed.crop)
        assertEquals(1f, placed.ratio, Tolerance)
        assertEquals(168f, placed.height, Tolerance)
    }

    @Test
    fun `the height follows the ratio the crop chose`() {
        val placed = PhotoPlacement(
            x = 0f,
            y = 0f,
            size = 180f,
            rotation = 0f,
            crop = PhotoFraming(0.1, 0.1, 0.5),
            ratio = 16f / 9f,
        )

        assertEquals(320f, placed.height, Tolerance)
    }
}
