package com.leafypuff.ui.editor.sticker

/// `x` and `y` are fractions of the note, `size` is dp. See the core's PlacedSticker for why.
data class PlacedSticker(
    val key: String,
    val sticker: StickerId,
    val x: Float,
    val y: Float,
    val size: Float,
    val rotation: Float,
)

fun dropSticker(sticker: StickerId, index: Int, key: String): PlacedSticker = PlacedSticker(
    key = key,
    sticker = sticker,
    x = dropX(index),
    y = dropY(index),
    size = StickerDropSize,
    rotation = 0f,
)
