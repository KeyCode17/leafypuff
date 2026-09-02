package com.leafypuff.ui.lock

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyElevation
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

private val ScreenPaddingTop = 88.dp
private val ScreenPaddingSide = 40.dp
private val ScreenPaddingBottom = 40.dp

private val TitleGap = 22.dp
private val HintGap = 6.dp
private val DotsGapAbove = 26.dp
private val DotsGapBelow = 32.dp
private val BiometricGap = 26.dp

private const val BiometricLabel = "Use Face ID"
private const val CancelLabel = "Cancel"
private val CancelGap = 18.dp
private val BiometricPaddingH = 24.dp
private val BiometricPaddingV = 13.dp

@Composable
fun LockScreen(
    pinLength: Int,
    hint: String,
    onDigit: (Char) -> Unit,
    onBackspace: () -> Unit,
    onBiometric: (() -> Unit)?,
    onCancel: (() -> Unit)?,
    modifier: Modifier = Modifier,
) {
    Column(
        modifier = modifier
            .fillMaxSize()
            .background(LocalLeafyColors.current.bg)
            .padding(
                start = ScreenPaddingSide,
                top = ScreenPaddingTop,
                end = ScreenPaddingSide,
                bottom = ScreenPaddingBottom,
            ),
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        LockMarkPlate()
        Spacer(Modifier.height(TitleGap))
        LockTitle()
        Spacer(Modifier.height(HintGap))
        LockHint(hint)
        Spacer(Modifier.height(DotsGapAbove))
        PinDots(pinLength)
        Spacer(Modifier.height(DotsGapBelow))
        LockKeypad(onDigit = onDigit, onBackspace = onBackspace)
        if (onBiometric != null) {
            Spacer(Modifier.height(BiometricGap))
            BiometricButton(onBiometric)
        }
        if (onCancel != null) {
            Spacer(Modifier.height(CancelGap))
            CancelButton(onCancel)
        }
    }
}

@Composable
private fun CancelButton(onCancel: () -> Unit) {
    Text(
        text = CancelLabel,
        style = LocalLeafyTypography.current.chipLabel,
        color = LocalLeafyColors.current.ink2,
        modifier = Modifier.clickable(onClick = onCancel),
    )
}

@Composable
private fun BiometricButton(onBiometric: () -> Unit) {
    val colors = LocalLeafyColors.current

    Text(
        text = BiometricLabel.uppercase(),
        style = LocalLeafyTypography.current.buttonLabel,
        color = colors.onAccent,
        modifier = Modifier
            .shadow(
                elevation = LeafyElevation.glow,
                shape = LeafyShapes.pill,
                ambientColor = colors.accent,
                spotColor = colors.accent,
            )
            .clip(LeafyShapes.pill)
            .background(colors.accent)
            .clickable(onClick = onBiometric)
            .padding(horizontal = BiometricPaddingH, vertical = BiometricPaddingV),
    )
}
