package com.leafypuff.ui.auth

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

internal val AuthMode.working: String
    get() = when (this) {
        AuthMode.Login -> "SIGNING IN…"
        AuthMode.Signup -> "CREATING…"
        AuthMode.VerifyEmail, AuthMode.VerifySignIn -> "CHECKING…"
    }
