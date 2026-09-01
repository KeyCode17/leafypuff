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
        AuthMode.VerifyEmail -> "Confirm your email"
        AuthMode.VerifySignIn -> "One last code"
    }

internal fun AuthMode.subtitle(email: String): String = when (this) {
    AuthMode.Login -> "Your stories are waiting."
    AuthMode.Signup -> "One page a day is enough."
    AuthMode.VerifyEmail -> "Enter the six digits we sent to $email."
    AuthMode.VerifySignIn -> "Enter the sign-in code we just sent to $email."
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
