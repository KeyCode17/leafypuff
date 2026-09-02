package com.leafypuff.ui.editor.sticker

import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.drawscope.DrawScope
import com.leafypuff.ui.common.outlineStroke
import com.leafypuff.ui.common.svgPath

internal const val StickerViewBox = 48f

private val CarrotBody = Color(0xFFE3A05C)
private val CarrotRib = Color(0xFFC97F3A)
private val CarrotLeafDeep = Color(0xFF8B9A5F)
private val CarrotLeafPale = Color(0xFFA8B98B)
private val HeartFill = Color(0xFFE0909A)
private val StarFill = Color(0xFFE3C766)
private val CloudFill = Color(0xFFC3C8B4)
private val PetalFill = Color(0xFFDEE5BC)
private val FlowerCenterFill = Color(0xFFE3C766)
private val MoonFill = Color(0xFFC4CE96)

private const val RibWidth = 1.4f
private const val PetalRadius = 6f
private const val FlowerCenterRadius = 5.4f

private val Body = svgPath("M15.5 21C15.5 18.4 32.5 18.4 32.5 21L25.4 42.4C24.9 44.2 23.1 44.2 22.6 42.4Z")
private val LeafDeep = svgPath("M24 20.4C21 14.4 21.5 8.2 24 4.6C26.5 8.2 27 14.4 24 20.4Z")
private val LeafPale =
    svgPath(
        "M23.2 20C18 17 14.6 11.6 16 8.6C19.6 8.7 22.6 13.6 24 19Z" +
            "M24.8 20C30 17 33.4 11.6 32 8.6C28.4 8.7 25.4 13.6 24 19Z",
    )
private val Ribs = listOf(svgPath("M19.5 27L28.5 29"), svgPath("M21.5 34L26.5 35.4"))
private val HeartOutline = svgPath(
    "M24 40C16 34.6 8 28.6 8 21.2 8 15.6 12.2 12 16.8 12c3 0 5.6 1.6 7.2 4 1.6-2.4 4.2-4 " +
        "7.2-4C35.8 12 40 15.6 40 21.2 40 28.6 32 34.6 24 40z",
)
private val StarOutline = svgPath(
    "M24 7l5.2 10.6L41 19.2l-8.5 8.3 2 11.7L24 33.7l-10.5 5.5 2-11.7L7 19.2l11.8-1.6z",
)
private val CloudOutline = svgPath("M14 34a7 7 0 0 1 .8-13.9A10 10 0 0 1 34 21.6 6.2 6.2 0 0 1 34 34z")
private val MoonOutline = svgPath("M30 8a17 17 0 1 0 10 30A20 20 0 0 1 30 8z")

private val Petals = listOf(
    Offset(24f, 14f),
    Offset(33f, 21f),
    Offset(30f, 32f),
    Offset(18f, 32f),
    Offset(15f, 21f),
)

internal fun DrawScope.drawStickerShape(sticker: StickerId) {
    when (sticker) {
        StickerId.Carrot -> drawCarrot()
        StickerId.Heart -> fill(HeartOutline, HeartFill)
        StickerId.Star -> fill(StarOutline, StarFill)
        StickerId.Cloud -> fill(CloudOutline, CloudFill)
        StickerId.Flower -> drawFlower()
        StickerId.Moon -> fill(MoonOutline, MoonFill)
        StickerId.BunSit, StickerId.BunSleep -> Unit
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
        -> Unit
    }
}

private fun DrawScope.drawCarrot() {
    fill(Body, CarrotBody)
    fill(LeafDeep, CarrotLeafDeep)
    fill(LeafPale, CarrotLeafPale)
    Ribs.forEach { drawPath(it, CarrotRib, style = outlineStroke(RibWidth)) }
}

private fun DrawScope.drawFlower() {
    Petals.forEach { drawCircle(PetalFill, PetalRadius, it) }
    drawCircle(FlowerCenterFill, FlowerCenterRadius, Offset(24f, 24f))
}

private fun DrawScope.fill(path: Path, color: Color) = drawPath(path, color)
