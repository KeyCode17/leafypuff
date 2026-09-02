package com.leafypuff.data

import com.leafypuff.ui.settings.StickerPack
import com.leafypuff.ui.settings.TextSize
import kotlinx.datetime.LocalTime

data class AppPreferences(
    val name: String = "",
    val darkMode: Boolean = false,
    val reminderEnabled: Boolean = false,
    val reminderTime: LocalTime = LocalTime(21, 0),
    val lockEnabled: Boolean = false,
    val stickerPack: StickerPack = StickerPack.Mixed,
    val textSize: TextSize = TextSize.Medium,
    val avatarPhotoId: String? = null,
)
