package com.leafypuff.ui.lock

import android.content.Context
import androidx.biometric.BiometricManager
import androidx.biometric.BiometricPrompt
import androidx.core.content.ContextCompat
import androidx.fragment.app.FragmentActivity

private const val PromptTitle = "Unlock leafyPuff"
private const val PromptSubtitle = "Use your fingerprint or face instead of the PIN"
private const val PromptCancel = "Use PIN"
private const val Authenticators = BiometricManager.Authenticators.BIOMETRIC_WEAK

fun biometricReady(context: Context): Boolean =
    BiometricManager.from(context).canAuthenticate(Authenticators) ==
        BiometricManager.BIOMETRIC_SUCCESS

fun unlockWithBiometric(
    context: Context,
    onProblem: (String) -> Unit,
    onUnlocked: () -> Unit,
) {
    val activity = context as? FragmentActivity ?: return
    if (!biometricReady(context)) {
        onProblem("This device has no biometric set up.")
        return
    }

    val prompt = BiometricPrompt(
        activity,
        ContextCompat.getMainExecutor(context),
        object : BiometricPrompt.AuthenticationCallback() {
            override fun onAuthenticationSucceeded(result: BiometricPrompt.AuthenticationResult) {
                onUnlocked()
            }

            override fun onAuthenticationError(code: Int, message: CharSequence) {
                onProblem(message.toString())
            }
        },
    )
    prompt.authenticate(
        BiometricPrompt.PromptInfo.Builder()
            .setTitle(PromptTitle)
            .setSubtitle(PromptSubtitle)
            .setNegativeButtonText(PromptCancel)
            .setAllowedAuthenticators(Authenticators)
            .build(),
    )
}
