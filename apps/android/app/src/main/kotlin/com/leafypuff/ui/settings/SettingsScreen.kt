package com.leafypuff.ui.settings

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import kotlinx.datetime.LocalTime

private val ScreenGutter = 24.dp
private val ScrollBottomPadding = 130.dp
private val TitleTopPadding = 20.dp
private val TitleBottomPadding = 24.dp

@Composable
fun SettingsScreen(
    state: SettingsState,
    versionName: String,
    onNameChange: (String) -> Unit,
    onToggleDark: (Boolean) -> Unit,
    onToggleReminder: (Boolean) -> Unit,
    onReminderTimeChange: (LocalTime) -> Unit,
    onToggleLock: (Boolean) -> Unit,
    onChangePin: () -> Unit,
    onStickerPackChange: (StickerPack) -> Unit,
    onTextSizeChange: (TextSize) -> Unit,
    lastSynced: String,
    onSync: () -> Unit,
    onExport: () -> Unit,
    onDeleteAll: () -> Unit,
    modifier: Modifier = Modifier,
) {
    var timePopupOpen by remember { mutableStateOf(false) }

    Column(
        modifier = modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState())
            .padding(horizontal = ScreenGutter)
            .padding(bottom = ScrollBottomPadding),
    ) {
        Text(
            text = "Settings",
            style = LocalLeafyTypography.current.screenTitle,
            color = LocalLeafyColors.current.ink,
            modifier = Modifier.padding(top = TitleTopPadding, bottom = TitleBottomPadding),
        )
        SettingsProfileCard(
            name = state.name,
            writingSince = state.writingSince,
            onNameChange = onNameChange,
            modifier = Modifier.padding(bottom = CardSpacing),
        )
        SettingsToggleCard(
            darkMode = state.darkMode,
            reminderEnabled = state.reminderEnabled,
            reminderTime = state.reminderTime,
            lockEnabled = state.lockEnabled,
            onToggleDark = onToggleDark,
            onToggleReminder = onToggleReminder,
            onReminderTimeClick = { timePopupOpen = true },
            onToggleLock = onToggleLock,
            onChangePin = onChangePin,
            modifier = Modifier.padding(bottom = CardSpacing),
        )
        SettingsChoiceCard(
            stickerPack = state.stickerPack,
            textSize = state.textSize,
            onStickerPackChange = onStickerPackChange,
            onTextSizeChange = onTextSizeChange,
            modifier = Modifier.padding(bottom = CardSpacing),
        )
        SettingsActionCard(
            entryCount = state.entryCount,
            versionName = versionName,
            lastSynced = lastSynced,
            onSync = onSync,
            onExport = onExport,
            onDeleteAll = onDeleteAll,
        )
    }

    if (timePopupOpen) {
        ReminderTimePopup(
            time = state.reminderTime,
            onTimeChange = onReminderTimeChange,
            onDismiss = { timePopupOpen = false },
        )
    }
}
