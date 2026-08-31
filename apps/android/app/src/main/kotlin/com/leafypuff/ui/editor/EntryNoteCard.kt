package com.leafypuff.ui.editor

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.defaultMinSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.SolidColor
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.photo.EntryPhoto

private val CardPadding = 18.dp
private val CardGap = 12.dp
private val BodyMinHeight = 120.dp
private val TagGap = 6.dp
private val TagRowTopPadding = 2.dp
private val TagPaddingX = 12.dp
private val TagPaddingY = 6.dp

@OptIn(ExperimentalLayoutApi::class)
@Composable
fun EntryNoteCard(
    title: String,
    body: String,
    tags: List<String>,
    photos: List<EntryPhoto>,
    onTitleChange: (String) -> Unit,
    onBodyChange: (String) -> Unit,
    onRemoveTag: (Int) -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Column(
        modifier = modifier
            .fillMaxWidth()
            .clip(LeafyShapes.card)
            .background(colors.sheet)
            .padding(CardPadding),
        verticalArrangement = Arrangement.spacedBy(CardGap),
    ) {
        NoteField(
            value = title,
            onValueChange = onTitleChange,
            placeholder = "Give it a title",
            textStyle = typography.noteTitleInput.copy(color = colors.ink),
            singleLine = true,
        )

        NoteField(
            value = body,
            onValueChange = onBodyChange,
            placeholder = "Tell your story…",
            textStyle = typography.body.copy(color = colors.ink),
            singleLine = false,
            modifier = Modifier.defaultMinSize(minHeight = BodyMinHeight),
        )

        photos.forEachIndexed { index, photo ->
            NotePhoto(photo = photo, isCover = index == 0)
        }

        if (tags.isNotEmpty()) {
            FlowRow(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(top = TagRowTopPadding),
                horizontalArrangement = Arrangement.spacedBy(TagGap),
                verticalArrangement = Arrangement.spacedBy(TagGap),
            ) {
                tags.forEachIndexed { index, tag ->
                    SelectedTagChip(tag = tag, onRemove = { onRemoveTag(index) })
                }
            }
        }
    }
}

@Composable
private fun NoteField(
    value: String,
    onValueChange: (String) -> Unit,
    placeholder: String,
    textStyle: TextStyle,
    singleLine: Boolean,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current

    BasicTextField(
        value = value,
        onValueChange = onValueChange,
        modifier = modifier.fillMaxWidth(),
        textStyle = textStyle,
        singleLine = singleLine,
        cursorBrush = SolidColor(colors.ink),
        decorationBox = { inner ->
            Box {
                if (value.isEmpty()) {
                    Text(text = placeholder, style = textStyle, color = colors.ink3)
                }
                inner()
            }
        },
    )
}

@Composable
private fun SelectedTagChip(tag: String, onRemove: () -> Unit) {
    val colors = LocalLeafyColors.current

    Text(
        text = tag,
        style = LocalLeafyTypography.current.chipLabel.copy(fontWeight = FontWeight.W600),
        color = colors.onAccent,
        modifier = Modifier
            .clip(LeafyShapes.chip)
            .background(colors.accent)
            .clickable(onClick = onRemove)
            .padding(horizontal = TagPaddingX, vertical = TagPaddingY),
    )
}
