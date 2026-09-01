package com.leafypuff.notify

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent

/**
 * Alarms do not survive a reboot. Without this the reminder would go quiet until the owner next
 * opened the app, which is exactly the day a reminder is for.
 */
class BootReceiver : BroadcastReceiver() {
    override fun onReceive(context: Context, intent: Intent) {
        if (intent.action == Intent.ACTION_BOOT_COMPLETED) {
            ReminderScheduler(context).rebook()
        }
    }
}
