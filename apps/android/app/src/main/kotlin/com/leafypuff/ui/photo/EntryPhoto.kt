package com.leafypuff.ui.photo

import androidx.compose.runtime.Immutable
import androidx.compose.ui.graphics.ImageBitmap
import kotlinx.datetime.LocalDate

@Immutable
data class PhotoPlacement(
    val x: Float,
    val y: Float,
    val size: Float,
    val rotation: Float,
)

data class EntryPhoto(
    val id: String,
    val cover: ImageBitmap,
    val takenOn: LocalDate?,
    val place: PhotoPlacement? = null,
)
