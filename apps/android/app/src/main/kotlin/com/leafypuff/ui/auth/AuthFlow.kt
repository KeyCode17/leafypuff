package com.leafypuff.ui.auth

import com.leafypuff.core.CoreClient
import com.leafypuff.core.LeafyPuffCoreException
import com.leafypuff.data.SessionStore

internal const val NoticeEmailConfirmed =
    "Email confirmed. We sent a sign-in code — check your inbox again."
internal const val NoticeCodeResent = "New code sent. The previous one no longer works."

data class SignedIn(val name: String, val password: String, val accessToken: String)

internal suspend fun advance(
    client: CoreClient,
    apiBaseUrl: String,
    state: AuthFormState,
    session: SessionStore,
    onSignedIn: (SignedIn) -> Unit,
    onNotice: (String) -> Unit,
): AuthFormState = runCatching {
    when (state.mode) {
        AuthMode.Signup -> {
            client.register(apiBaseUrl, state.email, state.password, state.name)
            state.copy(mode = AuthMode.VerifyEmail, code = "", error = null)
        }

        AuthMode.VerifyEmail -> {
            client.verifyEmail(apiBaseUrl, state.email, state.code)
            runCatching { client.signIn(apiBaseUrl, state.email, state.password) }.fold(
                onSuccess = {
                    onNotice(NoticeEmailConfirmed)
                    state.copy(mode = AuthMode.VerifySignIn, code = "", error = null)
                },
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

internal suspend fun resend(
    client: CoreClient,
    apiBaseUrl: String,
    state: AuthFormState,
    onNotice: (String) -> Unit,
): AuthFormState = runCatching {
    if (state.mode == AuthMode.VerifyEmail) {
        client.register(apiBaseUrl, state.email, state.password, state.name)
    } else {
        client.signIn(apiBaseUrl, state.email, state.password)
    }
    onNotice(NoticeCodeResent)
    state.copy(code = "", error = null)
}.getOrElse { failure -> state.copy(code = "", error = readable(state.mode, failure)) }

private fun readable(mode: AuthMode, failure: Throwable): String = when (failure) {
    is LeafyPuffCoreException.InvalidCredentials -> when {
        mode.verifying -> "That code is wrong or has expired. Send a new one below."
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
