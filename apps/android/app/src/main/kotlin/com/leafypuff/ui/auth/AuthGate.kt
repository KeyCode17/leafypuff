package com.leafypuff.ui.auth

import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import com.leafypuff.core.CoreClient
import com.leafypuff.data.SessionStore
import com.leafypuff.ui.common.ToastOverlay
import com.leafypuff.ui.common.ToastRequest
import com.leafypuff.ui.common.plainToast
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch

private const val ResendCooldownSeconds = 45
private const val SecondInMillis = 1_000L

@Composable
fun AuthGate(
    apiBaseUrl: String,
    client: CoreClient?,
    signedIn: Boolean,
    onSignedIn: (SignedIn) -> Unit,
    onPasswordReset: () -> Unit,
    modifier: Modifier = Modifier,
    content: @Composable () -> Unit,
) {
    val context = LocalContext.current
    val session = remember(context) { SessionStore(context) }
    val scope = rememberCoroutineScope()

    var state by remember { mutableStateOf(AuthFormState()) }
    var pending by remember { mutableStateOf(false) }
    var cooldown by remember { mutableIntStateOf(0) }
    var toast by remember { mutableStateOf<ToastRequest?>(null) }

    LaunchedEffect(signedIn) {
        if (signedIn) {
            state = AuthFormState()
            pending = false
            cooldown = 0
            toast = null
        }
    }

    if (signedIn) {
        content()
        return
    }

    LaunchedEffect(cooldown) {
        if (cooldown > 0) {
            delay(SecondInMillis)
            cooldown -= 1
        }
    }

    AuthScreen(
        state = state,
        pending = pending || client == null,
        resendIn = cooldown,
        onChange = { state = it },
        onSwitchMode = {
            state = AuthFormState(
                mode = if (state.mode == AuthMode.Login) AuthMode.Signup else AuthMode.Login,
                email = state.email,
            )
        },
        onForgotPassword = {
            cooldown = 0
            state = state.copy(mode = AuthMode.ForgotPassword, code = "", error = null)
        },
        onBack = {
            cooldown = 0
            state = state.copy(mode = state.mode.back, code = "", error = null)
        },
        onResend = {
            val core = client
            if (core != null && !pending && cooldown == 0) {
                pending = true
                cooldown = ResendCooldownSeconds
                scope.launch {
                    state = resend(core, apiBaseUrl, state) { toast = plainToast(it) }
                    pending = false
                }
            }
        },
        onSubmit = {
            val complaint = state.validate()
            val core = client
            when {
                complaint != null -> state = state.copy(error = complaint)
                core == null -> state = state.copy(error = "One moment, still opening your diary.")
                else -> {
                    pending = true
                    val before = state.mode
                    scope.launch {
                        state = advance(core, apiBaseUrl, state, session, onSignedIn) {
                            toast = plainToast(it)
                        }
                        if (before == AuthMode.ResetPassword && state.error == null) {
                            onPasswordReset()
                        }
                        pending = false
                    }
                }
            }
        },
        modifier = modifier,
    )

    ToastOverlay(
        visible = toast != null,
        request = toast,
        onAccept = { toast = null },
        onDismiss = { toast = null },
    )
}
