package com.leafypuff.ui.vault

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.leafypuff.data.VaultAccess
import com.leafypuff.data.VaultOpened
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.auth.PrimaryCta
import com.leafypuff.ui.auth.RecoveryCodeScreen
import com.leafypuff.ui.auth.SignedIn
import com.leafypuff.ui.common.ToastOverlay
import com.leafypuff.ui.common.ToastRequest
import com.leafypuff.ui.common.plainToast

private val TopPadding = 76.dp
private val SidePadding = 32.dp
private val BlockGap = 18.dp

private const val RefusedTitle = "This diary would not open"
private const val RefusedAction = "SIGN IN AGAIN"

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
    var refusal by remember { mutableStateOf<String?>(null) }
    var toast by remember { mutableStateOf<ToastRequest?>(null) }

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
            vault.openWith(
                apiBaseUrl = apiBaseUrl,
                accessToken = credentials.accessToken,
                password = credentials.password,
                onKeepFailed = { keep, _ -> toast = plainToast(keepFailureOf(keep)) },
            )
        }.onSuccess { outcome ->
            if (outcome is VaultOpened.Created) {
                recoveryCode = outcome.recoveryCode
            }
            unlocked = true
            onOpened()
        }.onFailure { failure -> refusal = refusalOf(failure) }
    }

    val code = recoveryCode
    val reason = refusal
    when {
        reason != null -> VaultRefused(
            reason = reason,
            onSignIn = {
                refusal = null
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

    ToastOverlay(
        visible = toast != null,
        request = toast,
        onAccept = { toast = null },
        onDismiss = { toast = null },
    )
}

@Composable
private fun VaultRefused(reason: String, onSignIn: () -> Unit, modifier: Modifier = Modifier) {
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
        Text(text = reason, style = typography.body, color = colors.ink2)
        PrimaryCta(label = RefusedAction, enabled = true, onClick = onSignIn)
    }
}
