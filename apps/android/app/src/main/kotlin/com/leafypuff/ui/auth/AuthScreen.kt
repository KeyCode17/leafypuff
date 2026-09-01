package com.leafypuff.ui.auth

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.unit.dp
import com.leafypuff.R
import com.leafypuff.theme.Destructive
import com.leafypuff.theme.LeafyElevation
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.theme.MarkPlate

private val PaddingTop = 76.dp
private val PaddingSide = 32.dp
private val PaddingBottom = 40.dp
private val PlateSize = 84.dp
private val MarkSize = 68.dp
private val PlateRadius = 28.dp
private val BlockGap = 18.dp
private val FieldGap = 14.dp
private val TitleGap = 6.dp

@Composable
fun AuthScreen(
    state: AuthFormState,
    pending: Boolean,
    onChange: (AuthFormState) -> Unit,
    onSubmit: () -> Unit,
    onSwitchMode: () -> Unit,
    onForgotPassword: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Column(
        modifier = modifier
            .fillMaxSize()
            .background(colors.bg)
            .verticalScroll(rememberScrollState())
            .padding(start = PaddingSide, top = PaddingTop, end = PaddingSide, bottom = PaddingBottom),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(BlockGap),
    ) {
        Box(
            modifier = Modifier
                .size(PlateSize)
                .shadow(LeafyElevation.card, RoundedCornerShape(PlateRadius))
                .clip(RoundedCornerShape(PlateRadius))
                .background(MarkPlate),
            contentAlignment = Alignment.Center,
        ) {
            Image(
                painter = painterResource(R.drawable.leafypuff_mark),
                contentDescription = null,
                modifier = Modifier.size(MarkSize),
            )
        }

        Column(
            modifier = Modifier.fillMaxWidth(),
            verticalArrangement = Arrangement.spacedBy(TitleGap),
        ) {
            Text(text = state.mode.title, style = typography.authTitle, color = colors.ink)
            Text(text = state.mode.subtitle, style = typography.body, color = colors.ink2)
        }

        Column(
            modifier = Modifier.fillMaxWidth(),
            verticalArrangement = Arrangement.spacedBy(FieldGap),
        ) {
            AuthFields(state = state, onChange = onChange)

            if (state.error != null) {
                Text(text = state.error, style = typography.chipLabel, color = Destructive)
            }

            if (state.mode == AuthMode.Login) {
                Text(
                    text = "Forgot password?",
                    style = typography.chipLabel,
                    color = colors.accentDeep,
                    modifier = Modifier
                        .align(Alignment.End)
                        .clickable(onClick = onForgotPassword),
                )
            }
        }

        PrimaryCta(
            label = if (pending) state.mode.working else state.mode.cta,
            enabled = !pending,
            onClick = onSubmit,
        )

        if (!state.mode.verifying) {
            AuthFooterSwitch(mode = state.mode, onSwitch = onSwitchMode)
        }
    }
}
