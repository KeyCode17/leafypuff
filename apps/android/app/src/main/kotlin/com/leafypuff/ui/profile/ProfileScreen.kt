package com.leafypuff.ui.profile

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import com.leafypuff.domain.Mood
import com.leafypuff.theme.Destructive
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.auth.AuthField
import com.leafypuff.ui.auth.PrimaryCta
import com.leafypuff.ui.common.BunnyFace

private val TopPadding = 68.dp
private val SidePadding = 32.dp
private val BlockGap = 18.dp
private val AvatarSize = 96.dp
private val AvatarBunnySize = 58.dp

@Composable
fun ProfileScreen(
    state: ProfileState,
    avatar: ImageBitmap?,
    onStateChange: (ProfileState) -> Unit,
    onPickAvatar: () -> Unit,
    onClearAvatar: () -> Unit,
    onSubmit: () -> Unit,
    onBack: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Column(
        modifier = modifier
            .fillMaxSize()
            .background(colors.bg)
            .verticalScroll(rememberScrollState())
            .padding(start = SidePadding, top = TopPadding, end = SidePadding),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(BlockGap),
    ) {
        Box(
            modifier = Modifier
                .size(AvatarSize)
                .clip(LeafyShapes.pill)
                .background(colors.soft2)
                .clickable(onClick = onPickAvatar),
            contentAlignment = Alignment.Center,
        ) {
            if (avatar == null) {
                BunnyFace(mood = Mood.Calm, modifier = Modifier.size(AvatarBunnySize))
            } else {
                Image(
                    bitmap = avatar,
                    contentDescription = null,
                    contentScale = ContentScale.Crop,
                    modifier = Modifier.fillMaxSize(),
                )
            }
        }
        Row(horizontalArrangement = Arrangement.spacedBy(BlockGap)) {
            Text(
                text = "Change photo",
                style = typography.chipLabel,
                color = colors.accentDeep,
                modifier = Modifier.clickable(onClick = onPickAvatar),
            )
            if (avatar != null) {
                Text(
                    text = "Remove photo",
                    style = typography.chipLabel,
                    color = Destructive,
                    modifier = Modifier.clickable(onClick = onClearAvatar),
                )
            }
        }

        Text(
            text = state.step.title,
            style = typography.authTitle,
            color = colors.ink,
            modifier = Modifier.fillMaxWidth(),
        )
        Text(
            text = state.step.subtitle(state.email),
            style = typography.body,
            color = colors.ink2,
            modifier = Modifier.fillMaxWidth(),
        )

        ProfileFields(state = state, onStateChange = onStateChange)

        if (state.error != null) {
            Text(
                text = state.error,
                style = typography.chipLabel,
                color = Destructive,
                modifier = Modifier.fillMaxWidth(),
            )
        }

        PrimaryCta(
            label = if (state.pending) state.step.working else state.step.cta,
            enabled = !state.pending,
            onClick = onSubmit,
        )
        Text(
            text = "Back",
            style = typography.chipLabel,
            color = colors.ink2,
            modifier = Modifier.clickable(onClick = onBack),
        )
    }
}

@Composable
private fun ProfileFields(state: ProfileState, onStateChange: (ProfileState) -> Unit) {
    when (state.step) {
        ProfileStep.Details -> {
            AuthField(
                label = "Your name",
                value = state.name,
                placeholder = "Kelinci",
                keyboard = KeyboardType.Text,
                onChange = { onStateChange(state.copy(name = it, error = null)) },
                modifier = Modifier.fillMaxWidth(),
            )
            AuthField(
                label = "Email",
                value = state.email,
                placeholder = "you@email.com",
                keyboard = KeyboardType.Email,
                onChange = { onStateChange(state.copy(email = it, error = null)) },
                modifier = Modifier.fillMaxWidth(),
            )
        }

        ProfileStep.ConfirmEmail -> AuthField(
            label = "Code",
            value = state.code,
            placeholder = "6 digits",
            keyboard = KeyboardType.NumberPassword,
            onChange = { onStateChange(state.copy(code = it, error = null)) },
            modifier = Modifier.fillMaxWidth(),
        )
    }
}
