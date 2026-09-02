package com.leafypuff.data

import com.leafypuff.core.LeafyPuffCoreException

enum class SyncOutcome {
    Moved,
    Held,
    SignedOut,
}

class SyncRunner(
    private val store: EntryStore,
    private val session: SessionStore,
    private val apiBaseUrl: String,
) {
    suspend fun run(): SyncOutcome {
        val token = session.accessToken() ?: return SyncOutcome.SignedOut
        return runCatching { store.sync(apiBaseUrl, token) }.fold(
            onSuccess = { SyncOutcome.Moved },
            onFailure = { failure ->
                if (failure is LeafyPuffCoreException.InvalidCredentials) renew() else SyncOutcome.Held
            },
        )
    }

    private suspend fun renew(): SyncOutcome {
        val held = session.refreshToken() ?: return SyncOutcome.SignedOut
        return runCatching { store.client.refreshSession(apiBaseUrl, held) }.fold(
            onSuccess = { issued ->
                session.renew(issued.accessToken, issued.refreshToken)
                runCatching { store.sync(apiBaseUrl, issued.accessToken) }.fold(
                    onSuccess = { SyncOutcome.Moved },
                    onFailure = { SyncOutcome.Held },
                )
            },
            onFailure = { failure ->
                if (failure is LeafyPuffCoreException.InvalidCredentials) {
                    SyncOutcome.SignedOut
                } else {
                    SyncOutcome.Held
                }
            },
        )
    }
}
