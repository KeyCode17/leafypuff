package com.leafypuff.ui.editor.sticker

import androidx.compose.foundation.Canvas
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.drawscope.scale
import androidx.compose.ui.graphics.drawscope.translate
import com.leafypuff.domain.Mood
import com.leafypuff.ui.common.BunnyFace

@Composable
fun StickerArt(sticker: StickerId, modifier: Modifier = Modifier) {
    when (sticker) {
        StickerId.BunSit -> BunnyFace(Mood.Happy, modifier)
        StickerId.BunSleep -> BunnyFace(Mood.Tired, modifier)
        StickerId.MoodHappy -> BunnyFace(Mood.Happy, modifier)
        StickerId.MoodCalm -> BunnyFace(Mood.Calm, modifier)
        StickerId.MoodGrateful -> BunnyFace(Mood.Grateful, modifier)
        StickerId.MoodExcited -> BunnyFace(Mood.Excited, modifier)
        StickerId.MoodOkay -> BunnyFace(Mood.Okay, modifier)
        StickerId.MoodTired -> BunnyFace(Mood.Tired, modifier)
        StickerId.MoodAnxious -> BunnyFace(Mood.Anxious, modifier)
        StickerId.MoodSad -> BunnyFace(Mood.Sad, modifier)
        StickerId.MoodAngry -> BunnyFace(Mood.Angry, modifier)
        StickerId.MoodSick -> BunnyFace(Mood.Sick, modifier)
        StickerId.MoodLonely -> BunnyFace(Mood.Lonely, modifier)
        StickerId.MoodLoved -> BunnyFace(Mood.Loved, modifier)
        else -> ShapeArt(sticker, modifier)
    }
}

@Composable
private fun ShapeArt(sticker: StickerId, modifier: Modifier) {
    Canvas(modifier) {
        val unit = size.minDimension / StickerViewBox
        translate(
            left = (size.width - StickerViewBox * unit) / 2f,
            top = (size.height - StickerViewBox * unit) / 2f,
        ) {
            scale(unit, Offset.Zero) {
                drawStickerShape(sticker)
            }
        }
    }
}
