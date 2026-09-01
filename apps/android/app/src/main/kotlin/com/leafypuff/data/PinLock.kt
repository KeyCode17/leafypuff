package com.leafypuff.data

import android.content.Context
import com.leafypuff.core.hashPin
import com.leafypuff.core.verifyPin

private const val PreferencesName = "leafypuff.lock"
private const val PinHashKey = "pin.hash"

const val PinLength = 4

class PinLock(private val context: Context) {
    fun isSet(): Boolean = stored() != null

    fun set(pin: String) {
        preferences()
            .edit()
            .putString(PinHashKey, hashPin(pin))
            .apply()
    }

    fun matches(pin: String): Boolean {
        val stored = stored() ?: return false
        return verifyPin(pin, stored)
    }

    fun clear() {
        preferences().edit().remove(PinHashKey).apply()
    }

    private fun stored(): String? = preferences().getString(PinHashKey, null)

    private fun preferences() =
        context.getSharedPreferences(PreferencesName, Context.MODE_PRIVATE)
}
