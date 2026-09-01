package com.leafypuff.ui.auth

/**
 * Login and Signup are the two modes the design draws. The two verification steps are what the API
 * adds — the handoff allows for "a verification step if the backend needs one", and this one needs
 * two: registering confirms the address, and every sign-in is then confirmed with its own code.
 */
enum class AuthMode {
    Login,
    Signup,
    VerifyEmail,
    VerifySignIn,
}

internal val AuthMode.verifying: Boolean
    get() = this == AuthMode.VerifyEmail || this == AuthMode.VerifySignIn

internal val AuthMode.title: String
    get() = when (this) {
        AuthMode.Login -> "Welcome back"
        AuthMode.Signup -> "Start your diary"
        AuthMode.VerifyEmail, AuthMode.VerifySignIn -> "Check your inbox"
    }

internal val AuthMode.subtitle: String
    get() = when (this) {
        AuthMode.Login -> "Your stories are waiting."
        AuthMode.Signup -> "One page a day is enough."
        AuthMode.VerifyEmail -> "Enter the six digits we sent you to confirm the address."
        AuthMode.VerifySignIn -> "Enter the six digits we sent you."
    }

internal val AuthMode.cta: String
    get() = when (this) {
        AuthMode.Login -> "LOG IN"
        AuthMode.Signup -> "CREATE ACCOUNT"
        AuthMode.VerifyEmail -> "CONFIRM EMAIL"
        AuthMode.VerifySignIn -> "VERIFY"
    }

/// What the button says while the request is out. The design draws no pending state, and without
/// one "working" and "broken" look identical: the button greys out and nothing else moves.
internal val AuthMode.working: String
    get() = when (this) {
        AuthMode.Login -> "SIGNING IN…"
        AuthMode.Signup -> "CREATING…"
        AuthMode.VerifyEmail, AuthMode.VerifySignIn -> "CHECKING…"
    }
