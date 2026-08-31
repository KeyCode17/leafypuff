package com.leafypuff.ui.photo

import androidx.compose.runtime.Immutable
import androidx.compose.ui.graphics.ImageBitmap
import kotlinx.datetime.LocalDate

@Immutable
data class EntryPhoto(
    val id: String,
    val cover: ImageBitmap,
    val takenOn: LocalDate?,
)
