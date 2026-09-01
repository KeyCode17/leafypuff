package com.leafypuff.ui.auth

enum class AuthMode {
    Login,
    Signup,
    VerifyEmail,
    VerifySignIn,
    ForgotPassword,
    ResetPassword,
}

internal val AuthMode.verifying: Boolean
    get() = this == AuthMode.VerifyEmail ||
        this == AuthMode.VerifySignIn ||
        this == AuthMode.ResetPassword

internal val AuthMode.stepping: Boolean
    get() = this != AuthMode.Login && this != AuthMode.Signup

internal val AuthMode.title: String
    get() = when (this) {
        AuthMode.Login -> "Welcome back"
        AuthMode.Signup -> "Start your diary"
        AuthMode.VerifyEmail -> "Confirm your email"
        AuthMode.VerifySignIn -> "One last code"
        AuthMode.ForgotPassword -> "Forgot your password"
        AuthMode.ResetPassword -> "Choose a new password"
    }

internal fun AuthMode.subtitle(email: String): String = when (this) {
    AuthMode.Login -> "Your stories are waiting."
    AuthMode.Signup -> "One page a day is enough."
    AuthMode.VerifyEmail -> "Enter the six digits we sent to $email."
    AuthMode.VerifySignIn -> "Enter the sign-in code we just sent to $email."
    AuthMode.ForgotPassword -> "We will send a code to the address you signed up with."
    AuthMode.ResetPassword -> "Enter the code we sent to $email and the password you want now."
}

internal val AuthMode.notice: String?
    get() = when (this) {
        AuthMode.ResetPassword ->
            "A new password gets you back into the account. Your entries stay sealed by the " +
                "password they were written under, so opening them again needs your recovery code."

        else -> null
    }

internal val AuthMode.cta: String
    get() = when (this) {
        AuthMode.Login -> "LOG IN"
        AuthMode.Signup -> "CREATE ACCOUNT"
        AuthMode.VerifyEmail -> "CONFIRM EMAIL"
        AuthMode.VerifySignIn -> "VERIFY"
        AuthMode.ForgotPassword -> "SEND RESET CODE"
        AuthMode.ResetPassword -> "CHANGE PASSWORD"
    }

internal val AuthMode.working: String
    get() = when (this) {
        AuthMode.Login -> "SIGNING IN…"
        AuthMode.Signup -> "CREATING…"
        AuthMode.VerifyEmail, AuthMode.VerifySignIn -> "CHECKING…"
        AuthMode.ForgotPassword -> "SENDING…"
        AuthMode.ResetPassword -> "CHANGING…"
    }

internal val AuthMode.back: AuthMode
    get() = when (this) {
        AuthMode.VerifyEmail -> AuthMode.Signup
        else -> AuthMode.Login
    }
