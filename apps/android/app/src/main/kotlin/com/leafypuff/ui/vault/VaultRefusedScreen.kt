package com.leafypuff.ui.vault

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.auth.PrimaryCta

private val TopPadding = 76.dp
private val SidePadding = 32.dp
private val BlockGap = 18.dp

private const val Title = "This diary would not open"
private const val Action = "SIGN IN AGAIN"
private const val Recover = "Use my recovery code"

@Composable
internal fun VaultRefusedScreen(
    reason: String,
    onSignIn: () -> Unit,
    onRecover: (() -> Unit)?,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Column(
        modifier = modifier
            .fillMaxSize()
            .background(colors.bg)
            .padding(start = SidePadding, top = TopPadding, end = SidePadding),
        verticalArrangement = Arrangement.spacedBy(BlockGap),
    ) {
        Text(text = Title, style = typography.authTitle, color = colors.ink)
        Text(text = reason, style = typography.body, color = colors.ink2)
        PrimaryCta(label = Action, enabled = true, onClick = onSignIn)

        if (onRecover != null) {
            Text(
                text = Recover,
                style = typography.chipLabel,
                color = colors.accentDeep,
                modifier = Modifier
                    .fillMaxWidth()
                    .clickable(onClick = onRecover),
            )
        }
    }
}
