package com.leafypuff.ui.vault

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import com.leafypuff.data.VaultAccess
import com.leafypuff.data.VaultOpened
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.ui.auth.RecoveryCodeScreen
import com.leafypuff.ui.auth.SignedIn
import com.leafypuff.ui.common.ToastOverlay
import com.leafypuff.ui.common.ToastRequest
import com.leafypuff.ui.common.plainToast
import kotlinx.coroutines.launch

private data class RecoveryAttempt(
    val code: String = "",
    val pending: Boolean = false,
    val error: String? = null,
)

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
    val scope = rememberCoroutineScope()
    var unlocked by remember { mutableStateOf(false) }
    var freshCode by remember { mutableStateOf<String?>(null) }
    var acknowledged by remember { mutableStateOf(false) }
    var refusal by remember { mutableStateOf<Refusal?>(null) }
    var recovery by remember { mutableStateOf<RecoveryAttempt?>(null) }
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
                freshCode = outcome.recoveryCode
            }
            unlocked = true
            onOpened()
        }.onFailure { failure -> refusal = refusalOf(failure) }
    }

    val attempt = recovery
    val reason = refusal
    val shown = freshCode
    when {
        attempt != null -> RecoveryUnlockScreen(
            code = attempt.code,
            pending = attempt.pending,
            error = attempt.error,
            onChange = { recovery = attempt.copy(code = it, error = null) },
            onSubmit = {
                val vault = access
                val credentials = signedIn
                if (vault != null && credentials != null && !attempt.pending) {
                    recovery = attempt.copy(pending = true, error = null)
                    scope.launch {
                        runCatching {
                            vault.resealWithRecoveryCode(
                                apiBaseUrl = apiBaseUrl,
                                accessToken = credentials.accessToken,
                                code = attempt.code,
                                password = credentials.password,
                                onKeepFailed = { keep, _ ->
                                    toast = plainToast(keepFailureOf(keep))
                                },
                            )
                        }.onSuccess {
                            recovery = null
                            refusal = null
                            unlocked = true
                            onOpened()
                        }.onFailure { failure ->
                            recovery = attempt.copy(pending = false, error = recoveryFailureOf(failure))
                        }
                    }
                }
            },
            modifier = modifier,
        )

        reason != null -> VaultRefusedScreen(
            reason = reason.reason,
            onSignIn = {
                refusal = null
                onSignedOut()
            },
            onRecover = if (reason.recoverable) {
                { recovery = RecoveryAttempt() }
            } else {
                null
            },
            modifier = modifier,
        )

        shown != null -> RecoveryCodeScreen(
            code = shown,
            acknowledged = acknowledged,
            onAcknowledge = { acknowledged = it },
            onContinue = { freshCode = null },
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
