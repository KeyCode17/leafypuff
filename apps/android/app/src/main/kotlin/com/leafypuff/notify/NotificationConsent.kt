package com.leafypuff.notify

import android.Manifest
import android.content.Context
import android.content.pm.PackageManager
import android.os.Build
import androidx.core.content.ContextCompat

const val NotificationPermission = Manifest.permission.POST_NOTIFICATIONS

/**
 * Below Android 13 posting needs no consent, so asking there would open a dialog the system
 * answers itself.
 */
fun needsNotificationConsent(context: Context): Boolean =
    Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU &&
        ContextCompat.checkSelfPermission(context, NotificationPermission) !=
        PackageManager.PERMISSION_GRANTED
