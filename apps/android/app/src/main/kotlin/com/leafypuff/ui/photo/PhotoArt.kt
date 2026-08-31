package com.leafypuff.ui.photo

import androidx.compose.ui.graphics.Canvas
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.Paint
import androidx.compose.ui.geometry.Rect

fun bandedPhoto(width: Int, height: Int, topRows: Int, top: Color, bottom: Color): ImageBitmap {
    val bitmap = ImageBitmap(width, height)
    val canvas = Canvas(bitmap)
    canvas.drawRect(
        Rect(0f, 0f, width.toFloat(), topRows.toFloat()),
        Paint().apply { color = top },
    )
    canvas.drawRect(
        Rect(0f, topRows.toFloat(), width.toFloat(), height.toFloat()),
        Paint().apply { color = bottom },
    )
    return bitmap
}
