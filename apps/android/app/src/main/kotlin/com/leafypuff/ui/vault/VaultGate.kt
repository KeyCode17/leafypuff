package com.leafypuff.ui.vault

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import com.leafypuff.data.VaultAccess
import com.leafypuff.data.VaultOpened
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.ui.auth.RecoveryCodeScreen
import com.leafypuff.ui.auth.SignedIn

/**
 * Between signing in and reading the diary. On a handset that has opened this account before the
 * step is silent; on one that has not, the password just typed fetches the account's vault, or
 * creates it when the account has none yet.
 *
 * A stored session with no local copy of the key is not an error state worth inventing a screen
 * for -- it means the password is needed again, and the app already has a screen that asks.
 */
@Composable
fun VaultGate(
    access: VaultAccess?,
    signedIn: SignedIn?,
    apiBaseUrl: String,
    onSignedOut: () -> Unit,
    onOpened: () -> Unit,
    modifier: Modifier = Modifier,
    content: @Composable () -> Unit,
) {
    var unlocked by remember { mutableStateOf(false) }
    var recoveryCode by remember { mutableStateOf<String?>(null) }
    var acknowledged by remember { mutableStateOf(false) }

    LaunchedEffect(access, signedIn) {
        val vault = access ?: return@LaunchedEffect
        if (unlocked) {
            return@LaunchedEffect
        }
        if (vault.openWithDeviceKey()) {
            unlocked = true
            onOpened()
            return@LaunchedEffect
        }
        val credentials = signedIn ?: run {
            onSignedOut()
            return@LaunchedEffect
        }
        runCatching {
            vault.openWith(apiBaseUrl, credentials.accessToken, credentials.password)
        }.onSuccess { outcome ->
            if (outcome is VaultOpened.Created) {
                recoveryCode = outcome.recoveryCode
            }
            unlocked = true
            onOpened()
        }.onFailure { onSignedOut() }
    }

    val code = recoveryCode
    when {
        code != null -> RecoveryCodeScreen(
            code = code,
            acknowledged = acknowledged,
            onAcknowledge = { acknowledged = it },
            onContinue = { recoveryCode = null },
            modifier = modifier,
        )

        unlocked -> content()

        else -> Box(
            modifier = modifier
                .fillMaxSize()
                .background(LocalLeafyColors.current.bg),
        )
    }
}
