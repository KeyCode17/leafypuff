package com.leafypuff.ui.editor.sticker

import com.leafypuff.ui.settings.StickerPack

enum class StickerId(val id: String) {
    BunSit("bunSit"),
    BunSleep("bunSleep"),
    Carrot("carrot"),
    Heart("heart"),
    Star("star"),
    Cloud("cloud"),
    Flower("flower"),
    Moon("moon"),
    MoodHappy("moodHappy"),
    MoodCalm("moodCalm"),
    MoodGrateful("moodGrateful"),
    MoodExcited("moodExcited"),
    MoodOkay("moodOkay"),
    MoodTired("moodTired"),
    MoodAnxious("moodAnxious"),
    MoodSad("moodSad"),
    MoodAngry("moodAngry"),
    MoodSick("moodSick"),
    MoodLonely("moodLonely"),
    MoodLoved("moodLoved"),
}

private val MoodPack = listOf(
    StickerId.MoodHappy,
    StickerId.MoodCalm,
    StickerId.MoodGrateful,
    StickerId.MoodExcited,
    StickerId.MoodOkay,
    StickerId.MoodTired,
    StickerId.MoodAnxious,
    StickerId.MoodSad,
    StickerId.MoodAngry,
    StickerId.MoodSick,
    StickerId.MoodLonely,
    StickerId.MoodLoved,
)

private val BunnyPack = listOf(
    StickerId.BunSit,
    StickerId.BunSleep,
    StickerId.Carrot,
    StickerId.Heart,
) + MoodPack

private val SimplePack = listOf(
    StickerId.Heart,
    StickerId.Star,
    StickerId.Cloud,
    StickerId.Flower,
    StickerId.Moon,
)

fun stickersFor(pack: StickerPack): List<StickerId> = when (pack) {
    StickerPack.Bunny -> BunnyPack
    StickerPack.Simple -> SimplePack
    StickerPack.Mixed -> StickerId.entries.toList()
}
