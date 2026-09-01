package com.leafypuff.theme

import androidx.compose.ui.unit.dp

/**
 * The four shadows the design names. They are separate tokens even where two share a dp value —
 * a card and a lock plate drift apart the moment the design revises one of them.
 */
object LeafyElevation {
    val card = 8.dp
    val glow = 12.dp
    val plate = 12.dp
    val popup = 18.dp
    val nav = 8.dp
}
