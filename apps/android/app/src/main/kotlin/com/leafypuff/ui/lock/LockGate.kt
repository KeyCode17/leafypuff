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

private enum class Step { Choose, Confirm, Verify }

/**
 * The screen the app opens behind. On a device with no PIN it asks for one twice; after that it
 * asks for it once. The 240 ms settle is what makes the fourth digit visible before the screen
 * changes, rather than the keypad appearing to swallow it.
 */
@Composable
fun LockGate(modifier: Modifier = Modifier, content: @Composable () -> Unit) {
    val context = LocalContext.current
    val lock = remember { PinLock(context) }
    var step by remember { mutableStateOf(if (lock.isSet()) Step.Verify else Step.Choose) }
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
        delay(UnlockDelayMillis)
        val entered = digits
        digits = ""
        settling = false
        when (step) {
            Step.Choose -> {
                chosen = entered
                step = Step.Confirm
            }

            Step.Confirm -> if (entered == chosen) {
                lock.set(entered)
                unlocked = true
            } else {
                chosen = ""
                step = Step.Choose
            }

            Step.Verify -> if (lock.matches(entered)) {
                unlocked = true
            }
        }
    }

    if (unlocked) {
        content()
        return
    }

    LockScreen(
        pinLength = digits.length,
        onDigit = { digit ->
            if (!settling && digits.length < PinLength) {
                digits += digit
            }
        },
        onBackspace = { digits = digits.dropLast(1) },
        onBiometric = {
            if (step == Step.Verify) {
                unlockWithBiometric(context) { unlocked = true }
            }
        },
        modifier = modifier,
    )
}
