package com.leafypuff.data

import android.content.Context

private const val PreferencesName = "leafypuff.session"
private const val AccessTokenKey = "access.token"
private const val RefreshTokenKey = "refresh.token"
private const val EmailKey = "email"

class SessionStore(private val context: Context) {
    fun signedIn(): Boolean = accessToken() != null

    fun accessToken(): String? = preferences().getString(AccessTokenKey, null)

    fun refreshToken(): String? = preferences().getString(RefreshTokenKey, null)

    fun renew(accessToken: String, refreshToken: String) {
        preferences().edit()
            .putString(AccessTokenKey, accessToken)
            .putString(RefreshTokenKey, refreshToken)
            .apply()
    }

    fun email(): String = preferences().getString(EmailKey, "").orEmpty()

    fun start(email: String, accessToken: String, refreshToken: String) {
        preferences().edit()
            .putString(EmailKey, email)
            .putString(AccessTokenKey, accessToken)
            .putString(RefreshTokenKey, refreshToken)
            .apply()
    }

    fun clear() {
        preferences().edit().clear().apply()
    }

    private fun preferences() =
        context.getSharedPreferences(PreferencesName, Context.MODE_PRIVATE)
}
