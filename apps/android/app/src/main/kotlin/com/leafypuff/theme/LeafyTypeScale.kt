package com.leafypuff.theme

import androidx.compose.runtime.Immutable
import androidx.compose.ui.unit.TextUnit
import androidx.compose.ui.unit.sp

@Immutable
data class LeafyTypeScale(
    val title: TextUnit,
    val body: TextUnit,
    val meta: TextUnit,
)

val LeafyTypeScaleSmall = LeafyTypeScale(title = 16.sp, body = 14.sp, meta = 10.sp)
val LeafyTypeScaleMedium = LeafyTypeScale(title = 17.sp, body = 15.sp, meta = 11.sp)
val LeafyTypeScaleLarge = LeafyTypeScale(title = 19.sp, body = 17.sp, meta = 12.sp)
