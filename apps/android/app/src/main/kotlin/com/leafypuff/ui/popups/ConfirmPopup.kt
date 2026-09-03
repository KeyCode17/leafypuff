package com.leafypuff.ui.popups

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import com.leafypuff.domain.Mood
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.common.BunnyFace
import com.leafypuff.ui.common.PromptPill

private val FaceSize = 64.dp
private val ConfirmPadding = 8.dp
private val ConfirmGap = 10.dp
private val ButtonsTopPadding = 8.dp
private val ButtonGap = 10.dp

@Composable
fun ConfirmPopup(
    face: Mood,
    title: String,
    body: String,
    accept: String,
    reject: String,
    onAccept: () -> Unit,
    onDismiss: () -> Unit,
) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    PopupShell(onDismiss = onDismiss) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(ConfirmPadding),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.spacedBy(ConfirmGap),
        ) {
            BunnyFace(mood = face, modifier = Modifier.size(FaceSize))
            Text(
                text = title,
                style = typography.cardTitle,
                color = colors.ink,
                textAlign = TextAlign.Center,
            )
            Text(
                text = body,
                style = typography.body,
                color = colors.ink2,
                textAlign = TextAlign.Center,
            )
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(top = ButtonsTopPadding),
                horizontalArrangement = Arrangement.spacedBy(ButtonGap),
            ) {
                PromptPill(
                    label = reject,
                    fill = colors.soft2,
                    ink = colors.ink,
                    weight = FontWeight.W500,
                    onClick = onDismiss,
                    modifier = Modifier.weight(1f),
                )
                PromptPill(
                    label = accept,
                    fill = colors.accent,
                    ink = colors.onAccent,
                    weight = FontWeight.W600,
                    onClick = onAccept,
                    modifier = Modifier.weight(1f),
                )
            }
        }
    }
}
