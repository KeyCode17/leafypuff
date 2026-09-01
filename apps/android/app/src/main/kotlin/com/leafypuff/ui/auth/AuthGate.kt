package com.leafypuff.ui.auth

import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import com.leafypuff.core.CoreClient
import com.leafypuff.core.LeafyPuffCoreException
import com.leafypuff.data.SessionStore
import com.leafypuff.ui.common.ToastOverlay
import com.leafypuff.ui.common.ToastRequest
import com.leafypuff.ui.common.plainToast
import kotlinx.coroutines.launch

@Composable
fun AuthGate(
    apiBaseUrl: String,
    client: CoreClient?,
    signedIn: Boolean,
    onSignedIn: (SignedIn) -> Unit,
    modifier: Modifier = Modifier,
    content: @Composable () -> Unit,
) {
    val context = LocalContext.current
    val session = remember(context) { SessionStore(context) }
    val scope = rememberCoroutineScope()

    var state by remember { mutableStateOf(AuthFormState()) }
    var pending by remember { mutableStateOf(false) }
    var toast by remember { mutableStateOf<ToastRequest?>(null) }

    if (signedIn) {
        content()
        return
    }

    AuthScreen(
        state = state,
        pending = pending || client == null,
        onChange = { state = it },
        onSwitchMode = {
            state = AuthFormState(
                mode = if (state.mode == AuthMode.Login) AuthMode.Signup else AuthMode.Login,
                email = state.email,
            )
        },
        onForgotPassword = { toast = plainToast("Reset link sent to your email.") },
        onSubmit = {
            val complaint = state.validate()
            val core = client
            when {
                complaint != null -> state = state.copy(error = complaint)
                core == null -> state = state.copy(error = "One moment, still opening your diary.")
                else -> {
                    pending = true
                    scope.launch {
                        state = advance(core, apiBaseUrl, state, session, onSignedIn)
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

private suspend fun advance(
    client: CoreClient,
    apiBaseUrl: String,
    state: AuthFormState,
    session: SessionStore,
    onSignedIn: (SignedIn) -> Unit,
): AuthFormState = runCatching {
    when (state.mode) {
        AuthMode.Signup -> {
            client.register(apiBaseUrl, state.email, state.password, state.name)
            state.copy(mode = AuthMode.VerifyEmail, code = "", error = null)
        }

        AuthMode.VerifyEmail -> {
            client.verifyEmail(apiBaseUrl, state.email, state.code)
            runCatching { client.signIn(apiBaseUrl, state.email, state.password) }.fold(
                onSuccess = { state.copy(mode = AuthMode.VerifySignIn, code = "", error = null) },
                onFailure = { failure ->
                    state.copy(
                        mode = AuthMode.Login,
                        code = "",
                        error = readable(AuthMode.Login, failure),
                    )
                },
            )
        }

        AuthMode.Login -> {
            client.signIn(apiBaseUrl, state.email, state.password)
            state.copy(mode = AuthMode.VerifySignIn, code = "", error = null)
        }

        AuthMode.VerifySignIn -> {
            val issued = client.verifySignIn(apiBaseUrl, state.email, state.code)
            session.start(state.email, issued.accessToken, issued.refreshToken)
            onSignedIn(SignedIn(state.name, state.password, issued.accessToken))
            state
        }
    }
}.getOrElse { failure -> state.copy(error = readable(state.mode, failure)) }

private fun readable(mode: AuthMode, failure: Throwable): String = when (failure) {
    is LeafyPuffCoreException.InvalidCredentials -> when {
        mode.verifying -> "That code is wrong or has expired."
        else -> "That email and password do not match."
    }

    is LeafyPuffCoreException.EmailNotVerified -> "Confirm your email first."
    is LeafyPuffCoreException.EmailTaken -> "That address already has an account."
    is LeafyPuffCoreException.TooManyAttempts -> "Too many attempts. Wait a moment."
    is LeafyPuffCoreException.MailUnavailable -> "We could not send the code. Try again shortly."
    is LeafyPuffCoreException.ServiceUnavailable -> "The service is busy. Try again shortly."
    is LeafyPuffCoreException.Timeout -> "The server is taking too long. Try again."
    is LeafyPuffCoreException.Unreadable -> "The server answered something we could not read."
    is LeafyPuffCoreException.Storage -> "No connection. Check your network."
    else -> "Something went wrong. Try again."
}

data class SignedIn(val name: String, val password: String, val accessToken: String)
