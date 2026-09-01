package com.leafypuff.notify

import android.Manifest
import android.content.Context
import android.content.pm.PackageManager
import android.os.Build
import androidx.core.content.ContextCompat

const val NotificationPermission = Manifest.permission.POST_NOTIFICATIONS

fun needsNotificationConsent(context: Context): Boolean =
    Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU &&
        ContextCompat.checkSelfPermission(context, NotificationPermission) !=
        PackageManager.PERMISSION_GRANTED
