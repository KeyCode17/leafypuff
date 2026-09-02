package com.leafypuff.ui.settings

import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.tooling.preview.Preview
import com.leafypuff.theme.LeafyTheme
import com.leafypuff.theme.LeafyTypeScaleLarge
import com.leafypuff.theme.LeafyTypeScaleMedium
import com.leafypuff.theme.LeafyTypeScaleSmall
import kotlinx.datetime.LocalDate
import kotlinx.datetime.LocalTime

private const val FrameWidth = 375
private const val FrameHeight = 812
private const val PreviewVersion = "0.3.0"

private val PreviewState = SettingsState(
    name = "Karyudi",
    writingSince = LocalDate(2026, 3, 1),
    darkMode = false,
    reminderEnabled = true,
    reminderTime = LocalTime(21, 0),
    lockEnabled = true,
    stickerPack = StickerPack.Mixed,
    textSize = TextSize.Medium,
    entryCount = 10,
)

@Preview(name = "Settings on light", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun SettingsLightPreview() {
    SettingsFrame(PreviewState)
}

@Preview(name = "Settings on dark", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun SettingsDarkPreview() {
    SettingsFrame(PreviewState.copy(darkMode = true))
}

@Preview(name = "Settings with the reminder off", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun SettingsReminderOffPreview() {
    SettingsFrame(PreviewState.copy(reminderEnabled = false, writingSince = null))
}

@Composable
private fun SettingsFrame(initial: SettingsState) {
    var state by remember { mutableStateOf(initial) }
    val scale = when (state.textSize) {
        TextSize.Small -> LeafyTypeScaleSmall
        TextSize.Medium -> LeafyTypeScaleMedium
        TextSize.Large -> LeafyTypeScaleLarge
    }

    LeafyTheme(darkOverride = state.darkMode, typeScale = scale) {
        SettingsScreen(
            state = state,
            versionName = PreviewVersion,
            onNameChange = { state = state.copy(name = it) },
            onToggleDark = { state = state.copy(darkMode = it) },
            onToggleReminder = { state = state.copy(reminderEnabled = it) },
            onReminderTimeChange = { state = state.copy(reminderTime = it) },
            onToggleLock = { state = state.copy(lockEnabled = it) },
            onChangePin = { },
            onStickerPackChange = { state = state.copy(stickerPack = it) },
            onTextSizeChange = { state = state.copy(textSize = it) },
            lastSynced = "Just now",
            onSync = { },
            onExport = { },
            onDeleteAll = { },
        )
    }
}
