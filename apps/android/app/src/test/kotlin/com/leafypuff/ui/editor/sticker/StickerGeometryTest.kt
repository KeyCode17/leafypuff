package com.leafypuff.ui.editor.sticker

import kotlin.test.Test
import kotlin.test.assertEquals

private const val Tolerance = 0.001f
private val Quarters = listOf(0f, 90f, 180f, 270f)

class StickerGeometryTest {

    @Test
    fun `four degrees under a quarter turn snaps to it`() {
        Quarters.forEach { quarter ->
            assertEquals(quarter, snapRotation(quarter - 4f), Tolerance)
        }
    }

    @Test
    fun `four degrees over a quarter turn snaps to it`() {
        Quarters.forEach { quarter ->
            assertEquals(quarter, snapRotation(quarter + 4f), Tolerance)
        }
    }

    @Test
    fun `six degrees off a quarter turn is left alone`() {
        Quarters.forEach { quarter ->
            assertEquals(quarter + 6f, snapRotation(quarter + 6f), Tolerance)
            assertEquals(normalizeRotation(quarter - 6f), snapRotation(quarter - 6f), Tolerance)
        }
    }

    @Test
    fun `a full turn back onto zero snaps to zero`() {
        assertEquals(0f, snapRotation(356f), Tolerance)
        assertEquals(0f, snapRotation(-4f), Tolerance)
        assertEquals(0f, snapRotation(-360f), Tolerance)
    }

    @Test
    fun `size clamps at both ends and passes the middle through`() {
        assertEquals(StickerMinSize, clampStickerSize(20f), Tolerance)
        assertEquals(StickerMinSize, clampStickerSize(StickerMinSize), Tolerance)
        assertEquals(StickerMaxSize, clampStickerSize(240f), Tolerance)
        assertEquals(StickerDropSize, clampStickerSize(StickerDropSize), Tolerance)
    }

    @Test
    fun `position clamps to the layer plus the overhang, as a fraction of it`() {
        assertEquals(-StickerOverhang / 300f, clampStickerPosition(-3f, 64f, 300f), Tolerance)
        assertEquals(248f / 300f, clampStickerPosition(3f, 64f, 300f), Tolerance)
        assertEquals(0.4f, clampStickerPosition(0.4f, 64f, 300f), Tolerance)
    }

    @Test
    fun `an unmeasured layer has no fraction to clamp into`() {
        assertEquals(0f, clampStickerPosition(0.4f, 64f, 0f), Tolerance)
    }

    @Test
    fun `a drag moves by the fraction of the layer the pointer covered`() {
        val placed = PlacedSticker("k", StickerId.Heart, 0.5f, 0.5f, 64f, 0f)

        val moved = placed.movedBy(deltaX = 30f, deltaY = 50f, layerWidth = 300f, layerHeight = 500f)

        assertEquals(0.6f, moved.x, Tolerance)
        assertEquals(0.6f, moved.y, Tolerance)
    }

    @Test
    fun `a drag is clamped on both axes`() {
        val placed = PlacedSticker("k", StickerId.Heart, 0.8f, 0.02f, 64f, 0f)

        val moved = placed.movedBy(deltaX = 90f, deltaY = -60f, layerWidth = 300f, layerHeight = 500f)

        assertEquals(248f / 300f, moved.x, Tolerance)
        assertEquals(-StickerOverhang / 500f, moved.y, Tolerance)
    }

    @Test
    fun `a placement keeps its spot when the note changes width`() {
        val placed = PlacedSticker("k", StickerId.Heart, 0.5f, 0.5f, 64f, 0f)

        val onNarrow = placed.movedBy(0f, 0f, layerWidth = 300f, layerHeight = 500f)
        val onWide = placed.movedBy(0f, 0f, layerWidth = 600f, layerHeight = 900f)

        assertEquals(onNarrow.x, onWide.x, Tolerance)
        assertEquals(onNarrow.y, onWide.y, Tolerance)
    }

    @Test
    fun `a drop lands on the design stagger, scaled to the note`() {
        val first = dropSticker(StickerId.Star, 0, "a")
        val fourth = dropSticker(StickerId.Star, 3, "b")

        assertEquals(34f / 327f, first.x, Tolerance)
        assertEquals(60f / 200f, first.y, Tolerance)
        assertEquals(StickerDropSize, first.size, Tolerance)
        assertEquals(0f, first.rotation, Tolerance)
        assertEquals(34f / 327f, fourth.x, Tolerance)
        assertEquals(192f / 200f, fourth.y, Tolerance)
    }

    @Test
    fun `the pointer angle is measured clockwise from the centre`() {
        assertEquals(0f, pointerAngle(0f, 0f, 10f, 0f), Tolerance)
        assertEquals(90f, pointerAngle(0f, 0f, 0f, 10f), Tolerance)
        assertEquals(180f, pointerAngle(0f, 0f, -10f, 0f), Tolerance)
    }
}
