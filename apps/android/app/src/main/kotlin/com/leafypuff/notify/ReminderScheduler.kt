package com.leafypuff.notify

import android.app.AlarmManager
import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import com.leafypuff.data.PreferenceStore
import java.util.Calendar
import kotlinx.datetime.LocalTime

private const val RequestCode = 100

class ReminderScheduler(private val context: Context) {
    fun apply(enabled: Boolean, at: LocalTime) {
        when {
            enabled -> book(at)
            else -> cancel()
        }
    }

    fun rebook() {
        val held = PreferenceStore(context).load()
        apply(held.reminderEnabled, held.reminderTime)
    }

    private fun book(at: LocalTime) {
        val alarms = context.getSystemService(AlarmManager::class.java) ?: return
        alarms.setAndAllowWhileIdle(AlarmManager.RTC_WAKEUP, nextOccurrence(at), pending())
    }

    private fun cancel() {
        context.getSystemService(AlarmManager::class.java)?.cancel(pending())
    }

    private fun pending(): PendingIntent = PendingIntent.getBroadcast(
        context,
        RequestCode,
        Intent(context, ReminderReceiver::class.java),
        PendingIntent.FLAG_UPDATE_CURRENT or PendingIntent.FLAG_IMMUTABLE,
    )
}

private fun nextOccurrence(at: LocalTime): Long {
    val calendar = Calendar.getInstance().apply {
        set(Calendar.HOUR_OF_DAY, at.hour)
        set(Calendar.MINUTE, at.minute)
        set(Calendar.SECOND, 0)
        set(Calendar.MILLISECOND, 0)
    }
    if (calendar.timeInMillis <= System.currentTimeMillis()) {
        calendar.add(Calendar.DAY_OF_YEAR, 1)
    }
    return calendar.timeInMillis
}
