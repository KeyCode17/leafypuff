package com.leafypuff.ui.lock

import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import com.leafypuff.data.PinLength
import com.leafypuff.data.PinLock
import kotlinx.coroutines.delay

private const val UnlockDelayMillis = 240L

@Composable
fun LockGate(
    enabled: Boolean,
    biometricEnabled: Boolean,
    unlocked: Boolean,
    onUnlocked: () -> Unit,
    modifier: Modifier = Modifier,
    content: @Composable () -> Unit,
) {
    val context = LocalContext.current
    val lock = remember { PinLock(context) }
    if (!enabled || unlocked || !lock.isSet()) {
        content()
        return
    }

    var wrong by remember { mutableStateOf(false) }
    var digits by remember { mutableStateOf("") }
    var settling by remember { mutableStateOf(false) }

    LaunchedEffect(digits) {
        if (digits.length < PinLength) {
            settling = false
            return@LaunchedEffect
        }
        settling = true
        wrong = false
        delay(UnlockDelayMillis)
        val entered = digits
        digits = ""
        settling = false
        if (lock.matches(entered)) {
            onUnlocked()
        } else {
            wrong = true
        }
    }

    LockScreen(
        pinLength = digits.length,
        hint = lockHint(digits.length, wrong),
        onDigit = { digit ->
            if (!settling && digits.length < PinLength) {
                digits += digit
            }
        },
        onBackspace = { digits = digits.dropLast(1) },
        onBiometric = when {
            biometricEnabled && biometricReady(context) -> {
                { unlockWithBiometric(context, onProblem = { wrong = true }) { onUnlocked() } }
            }

            else -> null
        },
        onCancel = null,
        modifier = modifier,
    )
}

internal fun lockHint(pinLength: Int, wrong: Boolean): String = when {
    wrong -> "That is not your PIN"
    pinLength > 0 -> "Keep going"
    else -> "Enter your PIN"
}
