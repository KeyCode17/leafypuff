package com.leafypuff.ui.settings

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.SideEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalView
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog
import androidx.compose.ui.window.DialogProperties
import androidx.compose.ui.window.DialogWindowProvider
import com.leafypuff.theme.LeafyElevation
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import kotlinx.datetime.LocalTime

private val Scrim = Color(0x59242D35)
private val PopupGutter = 40.dp
private val PopupPadding = 20.dp
private val PopupGap = 16.dp
private val DoneHeight = 44.dp

@Composable
fun ReminderTimePopup(
    time: LocalTime,
    onTimeChange: (LocalTime) -> Unit,
    onDismiss: () -> Unit,
) {
    Dialog(
        onDismissRequest = onDismiss,
        properties = DialogProperties(usePlatformDefaultWidth = false),
    ) {
        val view = LocalView.current
        SideEffect { (view.parent as? DialogWindowProvider)?.window?.setDimAmount(0f) }

        Box(
            modifier = Modifier
                .fillMaxSize()
                .background(Scrim)
                .noRippleClick(onDismiss)
                .padding(horizontal = PopupGutter),
            contentAlignment = Alignment.Center,
        ) {
            PopupCard(time = time, onTimeChange = onTimeChange, onDone = onDismiss)
        }
    }
}

@OptIn(ExperimentalLayoutApi::class)
@Composable
private fun PopupCard(time: LocalTime, onTimeChange: (LocalTime) -> Unit, onDone: () -> Unit) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current
    var hourText by remember { mutableStateOf(hour12(time).toString()) }
    var minuteText by remember { mutableStateOf(formatMinute(time)) }
    val commit: (LocalTime) -> Unit = { next ->
        hourText = hour12(next).toString()
        minuteText = formatMinute(next)
        onTimeChange(next)
    }

    Column(
        modifier = Modifier
            .fillMaxWidth()
            .shadow(LeafyElevation.popup, LeafyShapes.popup)
            .clip(LeafyShapes.popup)
            .background(colors.sheet)
            .noRippleClick { }
            .padding(PopupPadding),
        verticalArrangement = Arrangement.spacedBy(PopupGap),
    ) {
        Text(
            text = "Reminder time".uppercase(),
            style = typography.metaLabel,
            color = colors.ink3,
        )
        ReminderTimeField(
            time = time,
            hourText = hourText,
            minuteText = minuteText,
            onHourText = { hourText = it },
            onMinuteText = { minuteText = it },
            onTypedTime = onTimeChange,
            onSteppedTime = commit,
        )
        FlowRow(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(PresetGap),
            verticalArrangement = Arrangement.spacedBy(PresetGap),
        ) {
            ReminderPresets.forEach { preset ->
                PresetPill(
                    label = preset.label,
                    selected = preset.time == time,
                    onClick = { commit(preset.time) },
                )
            }
        }
        DoneButton(onDone)
    }
}

@Composable
private fun DoneButton(onDone: () -> Unit) {
    val colors = LocalLeafyColors.current

    Box(
        modifier = Modifier
            .fillMaxWidth()
            .height(DoneHeight)
            .shadow(
                elevation = LeafyElevation.glow,
                shape = LeafyShapes.input,
                ambientColor = colors.accent,
                spotColor = colors.accent,
            )
            .clip(LeafyShapes.input)
            .background(colors.accent)
            .clickable(onClick = onDone),
        contentAlignment = Alignment.Center,
    ) {
        Text(
            text = "Done".uppercase(),
            style = LocalLeafyTypography.current.buttonLabel,
            color = colors.onAccent,
            textAlign = TextAlign.Center,
        )
    }
}

@Composable
private fun Modifier.noRippleClick(onClick: () -> Unit): Modifier {
    val interactionSource = remember { MutableInteractionSource() }
    return clickable(interactionSource = interactionSource, indication = null, onClick = onClick)
}
