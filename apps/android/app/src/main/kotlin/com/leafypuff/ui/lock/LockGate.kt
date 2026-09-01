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

internal enum class LockStep { Choose, Confirm, Verify }

/**
 * The screen the app opens behind when Settings has the lock on. With the lock off it is not a
 * screen at all — the design has no PIN in the way of someone who never asked for one.
 *
 * The first launch after the lock is turned on asks for a PIN twice; after that, once. The 240 ms
 * settle is what makes the fourth digit visible before the screen changes, rather than the keypad
 * appearing to swallow it.
 */
@Composable
fun LockGate(
    enabled: Boolean,
    modifier: Modifier = Modifier,
    content: @Composable () -> Unit,
) {
    if (!enabled) {
        content()
        return
    }

    val context = LocalContext.current
    val lock = remember { PinLock(context) }
    var step by remember { mutableStateOf(if (lock.isSet()) LockStep.Verify else LockStep.Choose) }
    var wrong by remember { mutableStateOf(false) }
    var unlocked by remember { mutableStateOf(false) }
    var chosen by remember { mutableStateOf("") }
    var digits by remember { mutableStateOf("") }
    var settling by remember { mutableStateOf(false) }

    LaunchedEffect(digits, step) {
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
        when (step) {
            LockStep.Choose -> {
                chosen = entered
                step = LockStep.Confirm
            }

            LockStep.Confirm -> if (entered == chosen) {
                lock.set(entered)
                unlocked = true
            } else {
                chosen = ""
                wrong = true
                step = LockStep.Choose
            }

            LockStep.Verify -> if (lock.matches(entered)) {
                unlocked = true
            } else {
                wrong = true
            }
        }
    }

    if (unlocked) {
        content()
        return
    }

    LockScreen(
        pinLength = digits.length,
        hint = lockHint(step, digits.length, wrong),
        onDigit = { digit ->
            if (!settling && digits.length < PinLength) {
                digits += digit
            }
        },
        onBackspace = { digits = digits.dropLast(1) },
        onBiometric = {
            if (step == LockStep.Verify) {
                unlockWithBiometric(context) { unlocked = true }
            }
        },
        modifier = modifier,
    )
}

/**
 * The design writes only the two hints the unlock screen shows. Choosing and confirming a PIN are
 * screens it never draws, so they say what they are rather than borrowing copy meant for unlocking.
 */
internal fun lockHint(step: LockStep, pinLength: Int, wrong: Boolean): String = when {
    wrong && step == LockStep.Choose -> "Those did not match. Pick a PIN"
    wrong -> "That is not your PIN"
    pinLength > 0 -> "Keep going"
    step == LockStep.Choose -> "Pick a PIN"
    step == LockStep.Confirm -> "Enter it once more"
    else -> "Enter your PIN"
}
