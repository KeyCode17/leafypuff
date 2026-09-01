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

fun unlockWithBiometric(context: Context, onUnlocked: () -> Unit) {
    val activity = context as? FragmentActivity ?: return
    if (BiometricManager.from(context).canAuthenticate(Authenticators) !=
        BiometricManager.BIOMETRIC_SUCCESS
    ) {
        return
    }

    val prompt = BiometricPrompt(
        activity,
        ContextCompat.getMainExecutor(context),
        object : BiometricPrompt.AuthenticationCallback() {
            override fun onAuthenticationSucceeded(result: BiometricPrompt.AuthenticationResult) {
                onUnlocked()
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
