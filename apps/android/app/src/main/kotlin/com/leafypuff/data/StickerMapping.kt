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
    StickerId.MoodHappy -> Sticker.MOOD_HAPPY
    StickerId.MoodCalm -> Sticker.MOOD_CALM
    StickerId.MoodGrateful -> Sticker.MOOD_GRATEFUL
    StickerId.MoodExcited -> Sticker.MOOD_EXCITED
    StickerId.MoodOkay -> Sticker.MOOD_OKAY
    StickerId.MoodTired -> Sticker.MOOD_TIRED
    StickerId.MoodAnxious -> Sticker.MOOD_ANXIOUS
    StickerId.MoodSad -> Sticker.MOOD_SAD
    StickerId.MoodAngry -> Sticker.MOOD_ANGRY
    StickerId.MoodSick -> Sticker.MOOD_SICK
    StickerId.MoodLonely -> Sticker.MOOD_LONELY
    StickerId.MoodLoved -> Sticker.MOOD_LOVED
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
    Sticker.MOOD_HAPPY -> StickerId.MoodHappy
    Sticker.MOOD_CALM -> StickerId.MoodCalm
    Sticker.MOOD_GRATEFUL -> StickerId.MoodGrateful
    Sticker.MOOD_EXCITED -> StickerId.MoodExcited
    Sticker.MOOD_OKAY -> StickerId.MoodOkay
    Sticker.MOOD_TIRED -> StickerId.MoodTired
    Sticker.MOOD_ANXIOUS -> StickerId.MoodAnxious
    Sticker.MOOD_SAD -> StickerId.MoodSad
    Sticker.MOOD_ANGRY -> StickerId.MoodAngry
    Sticker.MOOD_SICK -> StickerId.MoodSick
    Sticker.MOOD_LONELY -> StickerId.MoodLonely
    Sticker.MOOD_LOVED -> StickerId.MoodLoved
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
