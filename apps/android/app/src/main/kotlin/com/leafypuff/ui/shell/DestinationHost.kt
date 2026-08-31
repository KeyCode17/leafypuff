package com.leafypuff.ui.shell

import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import com.leafypuff.data.AppPreferences
import com.leafypuff.domain.Entry
import com.leafypuff.ui.calendar.CalendarScreen
import com.leafypuff.ui.diary.DiaryScreen
import com.leafypuff.ui.settings.SettingsScreen
import com.leafypuff.ui.settings.SettingsState
import com.leafypuff.ui.stats.StatRange
import com.leafypuff.ui.stats.StatisticsScreen
import kotlinx.datetime.LocalDate

@Composable
fun DestinationHost(
    destination: Destination,
    entries: List<Entry>,
    today: LocalDate,
    selected: LocalDate,
    visibleMonth: LocalDate,
    preferences: AppPreferences,
    versionName: String,
    onSelectDay: (LocalDate) -> Unit,
    onMonthChange: (LocalDate) -> Unit,
    onToday: () -> Unit,
    onCompose: () -> Unit,
    onPreferencesChange: (AppPreferences) -> Unit,
    onDeleteAll: () -> Unit,
) {
    var range by remember { mutableStateOf(StatRange.SevenDays) }

    when (destination) {
        Destination.Diary -> DiaryScreen(entries)

        Destination.Calendar -> CalendarScreen(
            entries = entries,
            visibleMonth = visibleMonth,
            selected = selected,
            onMonthChange = onMonthChange,
            onSelect = onSelectDay,
            onToday = onToday,
            onCreateForSelected = onCompose,
        )

        Destination.Statistics -> StatisticsScreen(
            entries = entries,
            range = range,
            onRangeChange = { range = it },
        )

        Destination.Settings -> SettingsScreen(
            state = settingsState(preferences, entries),
            versionName = versionName,
            onNameChange = { onPreferencesChange(preferences.copy(name = it)) },
            onToggleDark = { onPreferencesChange(preferences.copy(darkMode = it)) },
            onToggleReminder = { onPreferencesChange(preferences.copy(reminderEnabled = it)) },
            onReminderTimeChange = { onPreferencesChange(preferences.copy(reminderTime = it)) },
            onToggleLock = { onPreferencesChange(preferences.copy(lockEnabled = it)) },
            onStickerPackChange = { onPreferencesChange(preferences.copy(stickerPack = it)) },
            onTextSizeChange = { onPreferencesChange(preferences.copy(textSize = it)) },
            onExport = { },
            onDeleteAll = onDeleteAll,
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
