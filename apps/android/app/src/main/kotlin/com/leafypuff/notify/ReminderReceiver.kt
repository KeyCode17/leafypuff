package com.leafypuff.notify

import android.Manifest
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import androidx.core.app.NotificationCompat
import androidx.core.content.ContextCompat
import com.leafypuff.MainActivity
import com.leafypuff.R

private const val NotificationId = 1
private const val Title = "leafyPuff"
private const val Body = "Let's tell your today's story!"

class ReminderReceiver : BroadcastReceiver() {
    override fun onReceive(context: Context, intent: Intent) {
        if (!allowed(context)) {
            return
        }
        ensureReminderChannel(context)

        val open = PendingIntent.getActivity(
            context,
            0,
            Intent(context, MainActivity::class.java),
            PendingIntent.FLAG_IMMUTABLE,
        )
        val notification = NotificationCompat.Builder(context, ReminderChannelId)
            .setSmallIcon(R.drawable.leafypuff_mark)
            .setContentTitle(Title)
            .setContentText(Body)
            .setContentIntent(open)
            .setAutoCancel(true)
            .build()

        context.getSystemService(NotificationManager::class.java)
            ?.notify(NotificationId, notification)

        ReminderScheduler(context).rebook()
    }
}

private fun allowed(context: Context): Boolean = ContextCompat.checkSelfPermission(
    context,
    Manifest.permission.POST_NOTIFICATIONS,
) == PackageManager.PERMISSION_GRANTED
