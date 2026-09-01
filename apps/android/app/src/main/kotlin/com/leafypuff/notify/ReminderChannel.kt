package com.leafypuff.notify

import android.app.NotificationChannel
import android.app.NotificationManager
import android.content.Context
import android.os.Build

internal const val ReminderChannelId = "leafypuff.reminder"

private const val ChannelName = "Daily writing reminder"
private const val ChannelDescription = "One nudge a day, at the time you chose."

internal fun ensureReminderChannel(context: Context) {
    if (Build.VERSION.SDK_INT < Build.VERSION_CODES.O) {
        return
    }
    val channel = NotificationChannel(
        ReminderChannelId,
        ChannelName,
        NotificationManager.IMPORTANCE_DEFAULT,
    ).apply { description = ChannelDescription }
    context.getSystemService(NotificationManager::class.java)?.createNotificationChannel(channel)
}
