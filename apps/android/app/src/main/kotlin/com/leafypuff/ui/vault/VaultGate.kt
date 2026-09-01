package com.leafypuff.ui.vault

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.material3.Text
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.leafypuff.data.VaultAccess
import com.leafypuff.data.VaultOpened
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.auth.PrimaryCta
import com.leafypuff.ui.auth.RecoveryCodeScreen
import com.leafypuff.ui.auth.SignedIn

private val TopPadding = 76.dp
private val SidePadding = 32.dp
private val BlockGap = 18.dp

private const val RefusedTitle = "This diary would not open"
private const val RefusedBody =
    "The key stored for this account did not fit. Signing in again is worth a try; if it " +
        "keeps happening, your recovery code is the way in."
private const val RefusedAction = "SIGN IN AGAIN"

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
    var refused by remember { mutableStateOf(false) }

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
        }.onFailure {
            // Signing out here and saying nothing would spin: the login screen appears, the owner
            // signs in, the same blob fails to open, and round it goes with no way to tell why.
            refused = true
        }
    }

    val code = recoveryCode
    when {
        refused -> VaultRefused(
            onSignIn = {
                refused = false
                onSignedOut()
            },
            modifier = modifier,
        )

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

@Composable
private fun VaultRefused(onSignIn: () -> Unit, modifier: Modifier = Modifier) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Column(
        modifier = modifier
            .fillMaxSize()
            .background(colors.bg)
            .padding(start = SidePadding, top = TopPadding, end = SidePadding),
        verticalArrangement = Arrangement.spacedBy(BlockGap),
    ) {
        Text(text = RefusedTitle, style = typography.authTitle, color = colors.ink)
        Text(text = RefusedBody, style = typography.body, color = colors.ink2)
        PrimaryCta(label = RefusedAction, enabled = true, onClick = onSignIn)
    }
}
