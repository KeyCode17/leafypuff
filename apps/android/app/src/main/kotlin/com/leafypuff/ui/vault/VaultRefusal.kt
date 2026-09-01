package com.leafypuff.ui.vault

import com.leafypuff.core.LeafyPuffCoreException
import com.leafypuff.data.VaultKeep

internal data class Refusal(val reason: String, val recoverable: Boolean)

internal fun refusalOf(failure: Throwable): Refusal = Refusal(
    reason = reasonOf(failure),
    recoverable = failure is LeafyPuffCoreException.Crypto,
)

private fun reasonOf(failure: Throwable): String = when (failure) {
    is LeafyPuffCoreException.Crypto ->
        "That password did not open this diary. It was sealed with the password it was created " +
            "with, so your recovery code is the way back in."

    is LeafyPuffCoreException.InvalidCredentials -> "This session has expired. Sign in again."
    is LeafyPuffCoreException.Timeout -> "The server is taking too long. Try again."
    is LeafyPuffCoreException.ServiceUnavailable -> "The service is busy. Try again shortly."
    is LeafyPuffCoreException.Unreadable -> "The server answered something we could not read."
    is LeafyPuffCoreException.Storage -> "No connection. Check your network and try again."
    else -> failure.message ?: "Something went wrong opening this diary."
}

internal fun recoveryFailureOf(failure: Throwable): String = when (failure) {
    is LeafyPuffCoreException.Crypto ->
        "That recovery code did not open this diary. Check every character and try again."

    else -> reasonOf(failure)
}

internal fun keepFailureOf(keep: VaultKeep): String = when (keep) {
    VaultKeep.OnThisDevice ->
        "Your diary is open. This device will ask for your password again next time."

    VaultKeep.OnTheServer ->
        "Your diary is open, but its key is not backed up yet. A new phone could not restore it."
}
