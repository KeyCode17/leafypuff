package com.leafypuff.ui.auth

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

private val SwitchGap = 4.dp

@Composable
internal fun AuthFields(state: AuthFormState, onChange: (AuthFormState) -> Unit) {
    when (state.mode) {
        AuthMode.VerifyEmail, AuthMode.VerifySignIn -> AuthField(
            label = "Code",
            value = state.code,
            placeholder = "6 digits",
            keyboard = KeyboardType.NumberPassword,
            onChange = { onChange(state.copy(code = it, error = null)) },
        )

        else -> {
            if (state.mode == AuthMode.Signup) {
                AuthField(
                    label = "Your name",
                    value = state.name,
                    placeholder = "Kelinci",
                    keyboard = KeyboardType.Text,
                    onChange = { onChange(state.copy(name = it, error = null)) },
                    modifier = Modifier.fillMaxWidth(),
                )
            }
            AuthField(
                label = "Email",
                value = state.email,
                placeholder = "you@email.com",
                keyboard = KeyboardType.Email,
                onChange = { onChange(state.copy(email = it, error = null)) },
                modifier = Modifier.fillMaxWidth(),
            )
            AuthField(
                label = "Password",
                value = state.password,
                // The design writes "At least 8 characters"; the API refuses anything under twelve.
                // A placeholder that states a rule the server does not enforce is worse than the
                // one-word deviation, so it states the real floor.
                placeholder = "At least 12 characters",
                keyboard = KeyboardType.Password,
                masked = !state.passwordShown,
                onToggleMask = { onChange(state.copy(passwordShown = !state.passwordShown)) },
                onChange = { onChange(state.copy(password = it, error = null)) },
                modifier = Modifier.fillMaxWidth(),
            )
        }
    }
}

@Composable
internal fun AuthFooterSwitch(mode: AuthMode, onSwitch: () -> Unit) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current
    val lead = if (mode == AuthMode.Login) "Don't have an account?" else "Already writing?"
    val action = if (mode == AuthMode.Login) "Sign up" else "Log in"

    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.spacedBy(SwitchGap, Alignment.CenterHorizontally),
    ) {
        Text(text = lead, style = typography.chipLabel, color = colors.ink2)
        Text(
            text = action,
            style = typography.chipLabel,
            color = colors.accentDeep,
            modifier = Modifier.clickable(onClick = onSwitch),
        )
    }
}
