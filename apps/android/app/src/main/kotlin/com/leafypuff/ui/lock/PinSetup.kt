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

private const val SettleMillis = 240L

enum class PinSetupMode { Create, Change }

internal enum class SetupStepView { Current, Choose, Confirm }

@Composable
fun PinSetup(
    mode: PinSetupMode,
    onDone: () -> Unit,
    onCancel: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val context = LocalContext.current
    val lock = remember { PinLock(context) }
    val opening = if (mode == PinSetupMode.Change) SetupStepView.Current else SetupStepView.Choose

    var step by remember { mutableStateOf(opening) }
    var chosen by remember { mutableStateOf("") }
    var digits by remember { mutableStateOf("") }
    var wrong by remember { mutableStateOf(false) }
    var settling by remember { mutableStateOf(false) }

    LaunchedEffect(digits, step) {
        if (digits.length < PinLength) {
            settling = false
            return@LaunchedEffect
        }
        settling = true
        wrong = false
        delay(SettleMillis)
        val entered = digits
        digits = ""
        settling = false
        when (step) {
            SetupStepView.Current -> if (lock.matches(entered)) {
                step = SetupStepView.Choose
            } else {
                wrong = true
            }

            SetupStepView.Choose -> {
                chosen = entered
                step = SetupStepView.Confirm
            }

            SetupStepView.Confirm -> if (entered == chosen) {
                lock.set(entered)
                onDone()
            } else {
                chosen = ""
                wrong = true
                step = SetupStepView.Choose
            }
        }
    }

    LockScreen(
        pinLength = digits.length,
        hint = setupHint(step, mode, digits.length, wrong),
        onDigit = { digit ->
            if (!settling && digits.length < PinLength) {
                digits += digit
            }
        },
        onBackspace = { digits = digits.dropLast(1) },
        onBiometric = null,
        onCancel = onCancel,
        modifier = modifier,
    )
}

internal fun setupHint(
    step: SetupStepView,
    mode: PinSetupMode,
    pinLength: Int,
    wrong: Boolean,
): String = when {
    wrong && step == SetupStepView.Current -> "That is not your PIN"
    wrong -> "Those did not match. Pick a PIN"
    pinLength > 0 -> "Keep going"
    step == SetupStepView.Current -> "Enter your current PIN"
    step == SetupStepView.Confirm -> "Enter it once more"
    mode == PinSetupMode.Change -> "Pick a new PIN"
    else -> "Pick a PIN"
}
