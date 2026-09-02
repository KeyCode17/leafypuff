package com.leafypuff.ui.photo

import androidx.compose.runtime.Immutable
import androidx.compose.ui.graphics.ImageBitmap
import com.leafypuff.ui.crop.PhotoFraming
import kotlinx.datetime.LocalDate

fun List<EntryPhoto>.flowing(): List<EntryPhoto> = filter { it.place == null }

fun List<EntryPhoto>.placed(): List<EntryPhoto> = filter { it.place != null }

fun List<EntryPhoto>.inCoverOrder(): List<EntryPhoto> = flowing() + placed()

@Immutable
data class PhotoCover(
    val id: String,
    val cover: ImageBitmap,
)

data class PhotoPlacement(
    val x: Float,
    val y: Float,
    val size: Float,
    val rotation: Float,
    val crop: PhotoFraming? = null,
    val ratio: Float = 1f,
) {
    val height: Float get() = size * ratio
}

@Immutable
data class PhotoCropped(
    val id: String,
    val crop: PhotoFraming,
    val ratio: Float,
)

data class EntryPhoto(
    val id: String,
    val cover: ImageBitmap,
    val takenOn: LocalDate?,
    val place: PhotoPlacement? = null,
)
