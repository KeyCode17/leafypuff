package com.leafypuff.data

import com.leafypuff.core.Sticker
import com.leafypuff.ui.editor.sticker.PlacedSticker
import com.leafypuff.core.StickerDraft as CoreSticker
import com.leafypuff.ui.editor.sticker.StickerId

fun StickerId.toCore(): Sticker = when (this) {
    StickerId.BunSit -> Sticker.BUN_SIT
    StickerId.BunSleep -> Sticker.BUN_SLEEP
    StickerId.Carrot -> Sticker.CARROT
    StickerId.Heart -> Sticker.HEART
    StickerId.Star -> Sticker.STAR
    StickerId.Cloud -> Sticker.CLOUD
    StickerId.Flower -> Sticker.FLOWER
    StickerId.Moon -> Sticker.MOON
}

fun Sticker.toUi(): StickerId = when (this) {
    Sticker.BUN_SIT -> StickerId.BunSit
    Sticker.BUN_SLEEP -> StickerId.BunSleep
    Sticker.CARROT -> StickerId.Carrot
    Sticker.HEART -> StickerId.Heart
    Sticker.STAR -> StickerId.Star
    Sticker.CLOUD -> StickerId.Cloud
    Sticker.FLOWER -> StickerId.Flower
    Sticker.MOON -> StickerId.Moon
}

fun CoreSticker.toPlaced(): PlacedSticker = PlacedSticker(
    key = key,
    sticker = sticker.toUi(),
    x = x,
    y = y,
    size = size,
    rotation = rotation,
)

fun PlacedSticker.toCoreSticker(): CoreSticker = CoreSticker(
    key = key,
    sticker = sticker.toCore(),
    x = x,
    y = y,
    size = size,
    rotation = rotation,
)
