package com.leafypuff.ui.auth

data class AuthFormState(
    val mode: AuthMode = AuthMode.Login,
    val name: String = "",
    val email: String = "",
    val password: String = "",
    val code: String = "",
    val passwordShown: Boolean = false,
    val error: String? = null,
)

private const val MinimumPasswordLength = 12
private const val CodeLength = 6

internal fun AuthFormState.validate(): String? = when (mode) {
    AuthMode.VerifyEmail, AuthMode.VerifySignIn -> codeComplaint()
    AuthMode.ResetPassword -> codeComplaint() ?: passwordComplaint()
    AuthMode.ForgotPassword -> emailComplaint()
    AuthMode.Signup -> nameComplaint() ?: emailComplaint() ?: passwordComplaint()
    AuthMode.Login -> emailComplaint() ?: passwordComplaint()
}

private fun AuthFormState.codeComplaint(): String? =
    if (code.length == CodeLength && code.all(Char::isDigit)) null else "Enter the six digits"

private fun AuthFormState.nameComplaint(): String? =
    if (name.isBlank()) "Your name is required" else null

private fun AuthFormState.emailComplaint(): String? =
    if (email.contains('@') && email.substringAfter('@').contains('.')) {
        null
    } else {
        "Enter the address you registered with"
    }

private fun AuthFormState.passwordComplaint(): String? =
    if (password.length < MinimumPasswordLength) {
        "At least $MinimumPasswordLength characters"
    } else {
        null
    }
