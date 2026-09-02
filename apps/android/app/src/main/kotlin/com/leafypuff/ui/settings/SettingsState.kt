package com.leafypuff.ui.settings

import androidx.compose.runtime.Immutable
import kotlinx.datetime.LocalDate
import kotlinx.datetime.LocalTime

enum class StickerPack(val label: String) {
    Bunny("Bunny"),
    Simple("Simple"),
    Mixed("Mixed"),
}

enum class TextSize(val label: String) {
    Small("Small"),
    Medium("Medium"),
    Large("Large"),
}

@Immutable
data class SettingsState(
    val name: String,
    val writingSince: LocalDate?,
    val darkMode: Boolean,
    val reminderEnabled: Boolean,
    val reminderTime: LocalTime,
    val lockEnabled: Boolean,
    val biometricEnabled: Boolean,
    val stickerPack: StickerPack,
    val textSize: TextSize,
    val entryCount: Int,
)
