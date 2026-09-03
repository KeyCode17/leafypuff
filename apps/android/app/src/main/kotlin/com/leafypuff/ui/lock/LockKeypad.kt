package com.leafypuff.ui.lock

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.size
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Fingerprint
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LeafyStroke
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.Inter
import com.leafypuff.theme.Rubik

private const val BackspaceGlyph = "⌫"
private const val BlankKey = ""
private const val BiometricLabel = "Unlock with your fingerprint"

private val KeypadRows = listOf(
    listOf("1", "2", "3"),
    listOf("4", "5", "6"),
    listOf("7", "8", "9"),
    listOf(BlankKey, "0", BackspaceGlyph),
)

private val KeyWidth = 74.dp
private val KeyHeight = 64.dp
private val KeyColumnGap = 22.dp
private val KeyRowGap = 14.dp
private val FingerprintSize = 30.dp

private val KeyLabelStyle = TextStyle(
    fontFamily = Rubik,
    fontWeight = FontWeight.W500,
    fontSize = 26.sp,
)

@Composable
internal fun LockKeypad(
    onDigit: (Char) -> Unit,
    onBackspace: () -> Unit,
    onBiometric: (() -> Unit)?,
    modifier: Modifier = Modifier,
) {
    Column(
        modifier = modifier,
        verticalArrangement = Arrangement.spacedBy(KeyRowGap),
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        KeypadRows.forEach { row ->
            Row(horizontalArrangement = Arrangement.spacedBy(KeyColumnGap)) {
                row.forEach { label ->
                    when {
                        label == BlankKey && onBiometric != null -> FingerprintKey(onBiometric)
                        label == BlankKey -> Spacer(Modifier.size(KeyWidth, KeyHeight))
                        label == BackspaceGlyph -> KeypadKey(label, onBackspace)
                        else -> KeypadKey(label) { onDigit(label.first()) }
                    }
                }
            }
        }
    }
}

@Composable
private fun KeypadKey(label: String, onClick: () -> Unit) {
    val colors = LocalLeafyColors.current

    KeyFace(onClick = onClick) {
        val style = if (label == BackspaceGlyph) {
            KeyLabelStyle.copy(fontFamily = Inter)
        } else {
            KeyLabelStyle
        }
        Text(text = label, style = style, color = colors.ink)
    }
}

@Composable
private fun FingerprintKey(onClick: () -> Unit) {
    KeyFace(onClick = onClick) {
        Icon(
            imageVector = Icons.Filled.Fingerprint,
            contentDescription = BiometricLabel,
            tint = LocalLeafyColors.current.accentDeep,
            modifier = Modifier.size(FingerprintSize),
        )
    }
}

@Composable
private fun KeyFace(onClick: () -> Unit, content: @Composable () -> Unit) {
    val colors = LocalLeafyColors.current

    Box(
        modifier = Modifier
            .size(KeyWidth, KeyHeight)
            .clip(LeafyShapes.pill)
            .background(colors.key)
            .border(LeafyStroke.border, colors.keyLine, LeafyShapes.pill)
            .clickable(onClick = onClick),
        contentAlignment = Alignment.Center,
    ) {
        content()
    }
}
