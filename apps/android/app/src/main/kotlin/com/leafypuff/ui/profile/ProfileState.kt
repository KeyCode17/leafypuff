package com.leafypuff.ui.profile

enum class ProfileStep {
    Details,
    ConfirmEmail,
}

data class ProfileState(
    val name: String = "",
    val email: String = "",
    val code: String = "",
    val pending: Boolean = false,
    val error: String? = null,
    val step: ProfileStep = ProfileStep.Details,
)

internal val ProfileStep.title: String
    get() = when (this) {
        ProfileStep.Details -> "Your profile"
        ProfileStep.ConfirmEmail -> "Confirm the new address"
    }

internal fun ProfileStep.subtitle(email: String): String = when (this) {
    ProfileStep.Details ->
        "Your name and photo stay on this device. Your address is what signs you in."

    ProfileStep.ConfirmEmail ->
        "Enter the six digits we sent to $email. The old address keeps working until you do."
}

internal val ProfileStep.cta: String
    get() = when (this) {
        ProfileStep.Details -> "SAVE"
        ProfileStep.ConfirmEmail -> "CONFIRM ADDRESS"
    }

internal val ProfileStep.working: String
    get() = when (this) {
        ProfileStep.Details -> "SAVING…"
        ProfileStep.ConfirmEmail -> "CHECKING…"
    }
