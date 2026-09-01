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

/**
 * The design asks for email format and password length before the request goes out. The floor is
 * the API's, not the placeholder's: the server refuses anything under twelve characters, so
 * accepting eight here would only turn a typo into a round trip.
 */
internal fun AuthFormState.validate(): String? = when {
    mode.verifying ->
        if (code.length == CodeLength && code.all(Char::isDigit)) null else "Enter the six digits"

    mode == AuthMode.Signup && name.isBlank() -> "Your name is required"
    !email.contains('@') || !email.substringAfter('@').contains('.') ->
        "Enter the address you registered with"

    password.length < MinimumPasswordLength -> "At least $MinimumPasswordLength characters"
    else -> null
}
