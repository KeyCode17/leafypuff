package com.leafypuff.ui.auth

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.SolidColor
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.text.input.VisualTransformation
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LeafyStroke
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

private val FieldPaddingHorizontal = 16.dp
private val FieldPaddingVertical = 14.dp
private val LabelGap = 6.dp
private val ToggleGap = 12.dp

@Composable
internal fun AuthField(
    label: String,
    value: String,
    placeholder: String,
    keyboard: KeyboardType,
    onChange: (String) -> Unit,
    modifier: Modifier = Modifier,
    masked: Boolean = false,
    onToggleMask: (() -> Unit)? = null,
) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Column(modifier = modifier, verticalArrangement = Arrangement.spacedBy(LabelGap)) {
        Text(
            text = label.uppercase(),
            style = typography.metaLabel,
            color = colors.ink3,
        )
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .background(colors.surface, LeafyShapes.input)
                .border(LeafyStroke.border, colors.line, LeafyShapes.input)
                .padding(horizontal = FieldPaddingHorizontal, vertical = FieldPaddingVertical),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Box(modifier = Modifier.weight(1f)) {
                BasicTextField(
                    value = value,
                    onValueChange = onChange,
                    singleLine = true,
                    textStyle = typography.body.copy(color = colors.ink),
                    cursorBrush = SolidColor(colors.accent),
                    keyboardOptions = KeyboardOptions(keyboardType = keyboard),
                    visualTransformation = if (masked) {
                        PasswordVisualTransformation()
                    } else {
                        VisualTransformation.None
                    },
                    modifier = Modifier.fillMaxWidth(),
                )
                if (value.isEmpty()) {
                    Text(text = placeholder, style = typography.body, color = colors.ink3)
                }
            }
            if (onToggleMask != null) {
                Text(
                    text = if (masked) "SHOW" else "HIDE",
                    style = typography.fieldToggle,
                    color = colors.accentDeep,
                    modifier = Modifier
                        .padding(start = ToggleGap)
                        .clickable(onClick = onToggleMask),
                )
            }
        }
    }
}
