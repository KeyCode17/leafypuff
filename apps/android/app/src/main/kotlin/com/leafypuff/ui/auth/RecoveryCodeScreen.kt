package com.leafypuff.ui.auth

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.platform.LocalClipboardManager
import androidx.compose.ui.text.AnnotatedString
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LeafyStroke
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

private val PaddingTop = 76.dp
private val PaddingSide = 32.dp
private val PaddingBottom = 40.dp
private val BlockGap = 18.dp
private val CodePadding = 18.dp
private val CheckSize = 22.dp
private val CheckGap = 12.dp
private val CheckRadius = 6.dp

private const val Title = "Write this down"
private const val Body =
    "This code is the only way back into your diary if you forget your password. " +
        "It is not stored anywhere and cannot be shown again."
private const val Acknowledgement = "I have written it down somewhere safe"
private const val Copy = "COPY"
private const val Continue = "CONTINUE"

@Composable
fun RecoveryCodeScreen(
    code: String,
    acknowledged: Boolean,
    onAcknowledge: (Boolean) -> Unit,
    onContinue: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current
    val clipboard = LocalClipboardManager.current

    Column(
        modifier = modifier
            .fillMaxSize()
            .background(colors.bg)
            .verticalScroll(rememberScrollState())
            .padding(start = PaddingSide, top = PaddingTop, end = PaddingSide, bottom = PaddingBottom),
        verticalArrangement = Arrangement.spacedBy(BlockGap),
    ) {
        Text(text = Title, style = typography.authTitle, color = colors.ink)
        Text(text = Body, style = typography.body, color = colors.ink2)

        Box(
            modifier = Modifier
                .fillMaxWidth()
                .clip(LeafyShapes.input)
                .background(colors.surface)
                .border(LeafyStroke.border, colors.line, LeafyShapes.input)
                .padding(CodePadding),
        ) {
            Text(text = code, style = typography.noteTitleInput, color = colors.ink)
        }

        Text(
            text = Copy,
            style = typography.fieldToggle,
            color = colors.accentDeep,
            modifier = Modifier.clickable { clipboard.setText(AnnotatedString(code)) },
        )

        Acknowledgement(acknowledged = acknowledged, onAcknowledge = onAcknowledge)

        PrimaryCta(label = Continue, enabled = acknowledged, onClick = onContinue)
    }
}

@Composable
private fun Acknowledgement(acknowledged: Boolean, onAcknowledge: (Boolean) -> Unit) {
    val colors = LocalLeafyColors.current

    Row(
        modifier = Modifier
            .fillMaxWidth()
            .clickable { onAcknowledge(!acknowledged) },
        horizontalArrangement = Arrangement.spacedBy(CheckGap),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Box(
            modifier = Modifier
                .size(CheckSize)
                .clip(RoundedCornerShape(CheckRadius))
                .background(if (acknowledged) colors.accent else colors.surface)
                .border(LeafyStroke.border, colors.line, RoundedCornerShape(CheckRadius)),
            contentAlignment = Alignment.Center,
        ) {
            if (acknowledged) {
                Icon(
                    imageVector = Icons.Filled.Check,
                    contentDescription = null,
                    tint = colors.onAccent,
                    modifier = Modifier.size(CheckSize / 2),
                )
            }
        }
        Text(
            text = Acknowledgement,
            style = LocalLeafyTypography.current.chipLabel,
            color = colors.ink2,
        )
    }
}
