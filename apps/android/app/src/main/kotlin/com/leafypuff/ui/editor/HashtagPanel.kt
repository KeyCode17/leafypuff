package com.leafypuff.ui.editor

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.SolidColor
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

val HashtagPresets = listOf(
    "#slowday", "#home", "#work", "#coffee", "#rain", "#family", "#walk", "#food", "#rest",
)

private val ChipGap = 6.dp
private val ChipPaddingX = 12.dp
private val ChipPaddingY = 6.dp
private val InputPaddingX = 16.dp
private val InputPaddingY = 12.dp
private val InputBorder = 1.dp

@OptIn(ExperimentalLayoutApi::class)
@Composable
fun HashtagPanel(
    selected: List<String>,
    onAddTag: (String) -> Unit,
    modifier: Modifier = Modifier,
) {
    var input by remember { mutableStateOf("") }

    EditorDrawer(modifier = modifier) {
        FlowRow(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(ChipGap),
            verticalArrangement = Arrangement.spacedBy(ChipGap),
        ) {
            HashtagPresets.filterNot { it in selected }.forEach { preset ->
                PresetChip(tag = preset, onClick = { onAddTag(preset) })
            }
        }

        TagInput(
            value = input,
            onValueChange = { input = it },
            onSubmit = {
                val raw = input.trim().trimStart('#')
                if (raw.isNotEmpty()) {
                    onAddTag("#$raw")
                }
                input = ""
            },
        )
    }
}

@Composable
private fun PresetChip(tag: String, onClick: () -> Unit) {
    val colors = LocalLeafyColors.current

    Text(
        text = tag,
        style = LocalLeafyTypography.current.chipLabel,
        color = colors.accentDeep,
        modifier = Modifier
            .clip(LeafyShapes.chip)
            .background(colors.soft)
            .clickable(onClick = onClick)
            .padding(horizontal = ChipPaddingX, vertical = ChipPaddingY),
    )
}

@Composable
private fun TagInput(value: String, onValueChange: (String) -> Unit, onSubmit: () -> Unit) {
    val colors = LocalLeafyColors.current
    val textStyle = LocalLeafyTypography.current.body.copy(color = colors.ink)

    BasicTextField(
        value = value,
        onValueChange = onValueChange,
        modifier = Modifier
            .fillMaxWidth()
            .clip(LeafyShapes.input)
            .background(colors.sheet)
            .border(InputBorder, colors.line, LeafyShapes.input)
            .padding(horizontal = InputPaddingX, vertical = InputPaddingY),
        textStyle = textStyle,
        singleLine = true,
        cursorBrush = SolidColor(colors.ink),
        keyboardOptions = KeyboardOptions(imeAction = ImeAction.Done),
        keyboardActions = KeyboardActions(onDone = { onSubmit() }),
        decorationBox = { inner ->
            Box {
                if (value.isEmpty()) {
                    Text(
                        text = "Type a tag, press Enter",
                        style = textStyle,
                        color = colors.ink3,
                    )
                }
                inner()
            }
        },
    )
}
