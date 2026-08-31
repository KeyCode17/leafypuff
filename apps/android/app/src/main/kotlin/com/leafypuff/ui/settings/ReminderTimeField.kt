package com.leafypuff.ui.settings

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.SolidColor
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import kotlinx.datetime.LocalTime

private val StepperSize = 40.dp
private val FieldGap = 12.dp
private val DigitGap = 2.dp
private val DigitWidth = 46.dp
private val MeridiemGap = 6.dp
private val MeridiemPaddingH = 11.dp
private val MeridiemPaddingV = 7.dp
private val MeridiemFontSize = 14.sp
private const val MaxDigits = 2

@Composable
internal fun ReminderTimeField(
    time: LocalTime,
    hourText: String,
    minuteText: String,
    onHourText: (String) -> Unit,
    onMinuteText: (String) -> Unit,
    onTypedTime: (LocalTime) -> Unit,
    onSteppedTime: (LocalTime) -> Unit,
) {
    val colors = LocalLeafyColors.current

    Row(
        horizontalArrangement = Arrangement.spacedBy(FieldGap),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Stepper(glyph = "−") { onSteppedTime(shiftTime(time, -StepMinutes)) }
        Row(
            modifier = Modifier.weight(1f),
            horizontalArrangement = Arrangement.spacedBy(DigitGap, Alignment.CenterHorizontally),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            DigitInput(value = hourText, align = TextAlign.End) { digits ->
                onHourText(digits)
                digits.toIntOrNull()?.takeIf { it in 1..12 }?.let {
                    onTypedTime(composeTime(it, time.minute, isPm(time)))
                }
            }
            Text(
                text = ":",
                style = LocalLeafyTypography.current.lockTitle,
                color = colors.ink3,
            )
            DigitInput(value = minuteText, align = TextAlign.Start) { digits ->
                onMinuteText(digits)
                digits.toIntOrNull()?.takeIf { it in 0..59 }?.let {
                    onTypedTime(composeTime(hour12(time), it, isPm(time)))
                }
            }
            MeridiemPill(label = meridiemLabel(time)) {
                onSteppedTime(composeTime(hour12(time), time.minute, !isPm(time)))
            }
        }
        Stepper(glyph = "+") { onSteppedTime(shiftTime(time, StepMinutes)) }
    }
}

@Composable
private fun DigitInput(value: String, align: TextAlign, onDigits: (String) -> Unit) {
    val colors = LocalLeafyColors.current

    BasicTextField(
        value = value,
        onValueChange = { raw -> onDigits(raw.filter { it.isDigit() }.take(MaxDigits)) },
        singleLine = true,
        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number),
        textStyle = LocalLeafyTypography.current.lockTitle.copy(
            color = colors.ink,
            textAlign = align,
        ),
        cursorBrush = SolidColor(colors.accent),
        modifier = Modifier.width(DigitWidth),
    )
}

@Composable
private fun MeridiemPill(label: String, onClick: () -> Unit) {
    val colors = LocalLeafyColors.current

    Text(
        text = label,
        style = LocalLeafyTypography.current.buttonLabel.copy(fontSize = MeridiemFontSize),
        color = colors.accentDeep,
        modifier = Modifier
            .padding(start = MeridiemGap)
            .clip(LeafyShapes.pill)
            .background(colors.soft2)
            .clickable(onClick = onClick)
            .padding(horizontal = MeridiemPaddingH, vertical = MeridiemPaddingV),
    )
}

@Composable
private fun Stepper(glyph: String, onClick: () -> Unit) {
    val colors = LocalLeafyColors.current

    Box(
        modifier = Modifier
            .size(StepperSize)
            .clip(LeafyShapes.pill)
            .background(colors.soft2)
            .clickable(onClick = onClick),
        contentAlignment = Alignment.Center,
    ) {
        Text(
            text = glyph,
            style = LocalLeafyTypography.current.monthLabel.copy(fontWeight = FontWeight.W500),
            color = colors.ink,
        )
    }
}
