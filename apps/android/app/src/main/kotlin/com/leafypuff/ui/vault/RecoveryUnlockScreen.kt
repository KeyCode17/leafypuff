package com.leafypuff.ui.vault

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.imePadding
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.Destructive
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.auth.AuthField
import com.leafypuff.ui.auth.PrimaryCta

private val TopPadding = 76.dp
private val SidePadding = 32.dp
private val BlockGap = 18.dp

private const val Title = "Use your recovery code"
private const val Body =
    "This is the code the app showed you once, when the diary was created. It opens your " +
        "entries and seals them again under the password you just signed in with."
private const val Action = "OPEN MY DIARY"
private const val Working = "OPENING…"

@Composable
internal fun RecoveryUnlockScreen(
    code: String,
    pending: Boolean,
    error: String?,
    onChange: (String) -> Unit,
    onSubmit: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Column(
        modifier = modifier
            .fillMaxSize()
            .background(colors.bg)
            .imePadding()
            .verticalScroll(rememberScrollState())
            .padding(start = SidePadding, top = TopPadding, end = SidePadding),
        verticalArrangement = Arrangement.spacedBy(BlockGap),
    ) {
        Text(text = Title, style = typography.authTitle, color = colors.ink)
        Text(text = Body, style = typography.body, color = colors.ink2)

        AuthField(
            label = "Recovery code",
            value = code,
            placeholder = "26 letters and digits",
            keyboard = KeyboardType.Text,
            onChange = onChange,
            modifier = Modifier.fillMaxWidth(),
        )

        if (error != null) {
            Text(text = error, style = typography.chipLabel, color = Destructive)
        }

        PrimaryCta(
            label = if (pending) Working else Action,
            enabled = !pending,
            onClick = onSubmit,
        )
    }
}
