package com.leafypuff.data

import android.content.Context
import com.leafypuff.ui.settings.StickerPack
import com.leafypuff.ui.settings.TextSize
import kotlinx.datetime.LocalTime

private const val PreferencesName = "leafypuff.settings"
private const val NameKey = "name"
private const val DarkModeKey = "dark.mode"
private const val DarkModeSetKey = "dark.mode.set"
private const val ReminderEnabledKey = "reminder.enabled"
private const val ReminderMinutesKey = "reminder.minutes"
private const val LockEnabledKey = "lock.enabled"
private const val StickerPackKey = "sticker.pack"
private const val TextSizeKey = "text.size"

private const val MinutesPerHour = 60

class PreferenceStore(private val context: Context) {
    fun load(): AppPreferences {
        val held = preferences()
        val minutes = held.getInt(ReminderMinutesKey, DefaultReminderMinutes)
        return AppPreferences(
            name = held.getString(NameKey, "").orEmpty(),
            darkMode = held.getBoolean(DarkModeSetKey, false) && held.getBoolean(DarkModeKey, false),
            reminderEnabled = held.getBoolean(ReminderEnabledKey, false),
            reminderTime = LocalTime(minutes / MinutesPerHour, minutes % MinutesPerHour),
            lockEnabled = held.getBoolean(LockEnabledKey, false),
            stickerPack = readPack(held.getString(StickerPackKey, null)),
            textSize = readTextSize(held.getString(TextSizeKey, null)),
        )
    }

    fun save(preferences: AppPreferences) {
        preferences().edit()
            .putString(NameKey, preferences.name)
            .putBoolean(DarkModeKey, preferences.darkMode)
            .putBoolean(DarkModeSetKey, true)
            .putBoolean(ReminderEnabledKey, preferences.reminderEnabled)
            .putInt(
                ReminderMinutesKey,
                preferences.reminderTime.hour * MinutesPerHour + preferences.reminderTime.minute,
            )
            .putBoolean(LockEnabledKey, preferences.lockEnabled)
            .putString(StickerPackKey, preferences.stickerPack.name)
            .putString(TextSizeKey, preferences.textSize.name)
            .apply()
    }

    fun lockEnabled(): Boolean = preferences().getBoolean(LockEnabledKey, false)

    private fun preferences() =
        context.getSharedPreferences(PreferencesName, Context.MODE_PRIVATE)

    private fun readPack(stored: String?): StickerPack =
        StickerPack.entries.firstOrNull { it.name == stored } ?: StickerPack.Mixed

    private fun readTextSize(stored: String?): TextSize =
        TextSize.entries.firstOrNull { it.name == stored } ?: TextSize.Medium

    private companion object {
        const val DefaultReminderMinutes = 21 * MinutesPerHour
    }
}
