package com.leafypuff.ui.settings

import androidx.compose.runtime.Immutable
import kotlinx.datetime.LocalTime

internal const val StepMinutes = 15

private const val MinutesPerHour = 60
private const val MinutesPerDay = 1440
private const val NoonHour = 12

@Immutable
internal data class TimePreset(val label: String, val time: LocalTime)

internal val ReminderPresets = listOf(
    TimePreset("Morning", LocalTime(7, 0)),
    TimePreset("Noon", LocalTime(12, 0)),
    TimePreset("Evening", LocalTime(20, 0)),
    TimePreset("Night", LocalTime(22, 0)),
)

internal fun hour12(time: LocalTime): Int = (time.hour % NoonHour).let { if (it == 0) NoonHour else it }

internal fun isPm(time: LocalTime): Boolean = time.hour >= NoonHour

internal fun meridiemLabel(time: LocalTime): String = if (isPm(time)) "PM" else "AM"

internal fun formatMinute(time: LocalTime): String = time.minute.toString().padStart(2, '0')

internal fun formatTime12(time: LocalTime): String =
    "${hour12(time)}:${formatMinute(time)} ${meridiemLabel(time)}"

internal fun shiftTime(time: LocalTime, minutes: Int): LocalTime {
    val raw = time.hour * MinutesPerHour + time.minute + minutes
    val total = ((raw % MinutesPerDay) + MinutesPerDay) % MinutesPerDay
    return LocalTime(total / MinutesPerHour, total % MinutesPerHour)
}

internal fun composeTime(hour12: Int, minute: Int, pm: Boolean): LocalTime {
    val hour = (hour12 % NoonHour) + if (pm) NoonHour else 0
    return LocalTime(hour, minute.coerceIn(0, 59))
}
