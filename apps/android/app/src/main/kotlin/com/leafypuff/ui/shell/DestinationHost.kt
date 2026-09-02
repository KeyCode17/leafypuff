package com.leafypuff.ui.shell

import androidx.compose.runtime.Composable
import androidx.compose.ui.graphics.ImageBitmap
import com.leafypuff.data.AppPreferences
import com.leafypuff.domain.Entry
import com.leafypuff.ui.calendar.CalendarScreen
import com.leafypuff.ui.diary.DiaryScreen
import com.leafypuff.ui.settings.SettingsScreen
import com.leafypuff.ui.settings.SettingsState
import com.leafypuff.ui.stats.StatRange
import com.leafypuff.ui.stats.StatisticsScreen
import com.leafypuff.ui.stats.StatsSummary
import kotlinx.datetime.LocalDate

@Composable
fun DestinationHost(
    destination: Destination,
    entries: List<Entry>,
    today: LocalDate,
    selected: LocalDate,
    visibleMonth: LocalDate,
    covers: Map<String, ImageBitmap>,
    statistics: StatsSummary,
    range: StatRange,
    preferences: AppPreferences,
    versionName: String,
    onSelectDay: (LocalDate) -> Unit,
    onMonthChange: (LocalDate) -> Unit,
    onToday: () -> Unit,
    onCompose: () -> Unit,
    onOpenEntry: (Entry) -> Unit,
    onRangeChange: (StatRange) -> Unit,
    lastSynced: String,
    onSync: () -> Unit,
    onExport: () -> Unit,
    onPreferencesChange: (AppPreferences) -> Unit,
    onToggleLock: (Boolean) -> Unit,
    onChangePin: () -> Unit,
    onSignOut: () -> Unit,
    onDeleteAll: () -> Unit,
) {
    when (destination) {
        Destination.Diary -> DiaryScreen(
            entries = entries,
            covers = covers,
            onOpen = onOpenEntry,
        )

        Destination.Calendar -> CalendarScreen(
            entries = entries,
            visibleMonth = visibleMonth,
            selected = selected,
            onMonthChange = onMonthChange,
            onSelect = onSelectDay,
            onToday = onToday,
            onCreateForSelected = onCompose,
            covers = covers,
            onOpen = onOpenEntry,
        )

        Destination.Statistics -> StatisticsScreen(
            summary = statistics,
            range = range,
            onRangeChange = onRangeChange,
        )

        Destination.Settings -> SettingsScreen(
            state = settingsState(preferences, entries),
            versionName = versionName,
            onNameChange = { onPreferencesChange(preferences.copy(name = it)) },
            onToggleDark = { onPreferencesChange(preferences.copy(darkMode = it)) },
            onToggleReminder = { onPreferencesChange(preferences.copy(reminderEnabled = it)) },
            onReminderTimeChange = { onPreferencesChange(preferences.copy(reminderTime = it)) },
            onToggleLock = onToggleLock,
            onChangePin = onChangePin,
            onStickerPackChange = { onPreferencesChange(preferences.copy(stickerPack = it)) },
            onTextSizeChange = { onPreferencesChange(preferences.copy(textSize = it)) },
            lastSynced = lastSynced,
            onSync = onSync,
            onExport = onExport,
            onDeleteAll = onDeleteAll,
            onSignOut = onSignOut,
        )
    }
}

private fun settingsState(preferences: AppPreferences, entries: List<Entry>): SettingsState =
    SettingsState(
        name = preferences.name,
        writingSince = entries.minByOrNull { it.date }?.date,
        darkMode = preferences.darkMode,
        reminderEnabled = preferences.reminderEnabled,
        reminderTime = preferences.reminderTime,
        lockEnabled = preferences.lockEnabled,
        stickerPack = preferences.stickerPack,
        textSize = preferences.textSize,
        entryCount = entries.size,
    )
