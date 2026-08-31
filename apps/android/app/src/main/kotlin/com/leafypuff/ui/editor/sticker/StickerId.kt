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
}

private val BunnyPack = listOf(
    StickerId.BunSit,
    StickerId.BunSleep,
    StickerId.Carrot,
    StickerId.Heart,
)

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
