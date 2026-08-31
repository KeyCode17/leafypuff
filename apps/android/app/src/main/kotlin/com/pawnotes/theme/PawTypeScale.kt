package com.pawnotes.theme

import androidx.compose.runtime.Immutable
import androidx.compose.ui.unit.TextUnit
import androidx.compose.ui.unit.sp

@Immutable
data class PawTypeScale(
    val title: TextUnit,
    val body: TextUnit,
    val meta: TextUnit,
)

val PawTypeScaleSmall = PawTypeScale(title = 16.sp, body = 14.sp, meta = 10.sp)
val PawTypeScaleMedium = PawTypeScale(title = 17.sp, body = 15.sp, meta = 11.sp)
val PawTypeScaleLarge = PawTypeScale(title = 19.sp, body = 17.sp, meta = 12.sp)
