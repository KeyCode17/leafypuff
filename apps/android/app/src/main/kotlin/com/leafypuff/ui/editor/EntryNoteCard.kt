package com.leafypuff.ui.editor

import androidx.compose.foundation.ExperimentalFoundationApi
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
import androidx.compose.foundation.relocation.BringIntoViewRequester
import androidx.compose.foundation.relocation.bringIntoViewRequester
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.focus.onFocusChanged
import androidx.compose.ui.graphics.SolidColor
import androidx.compose.ui.text.TextLayoutResult
import androidx.compose.ui.text.TextRange
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.TextFieldValue
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.photo.EntryPhoto
import com.leafypuff.ui.photo.flowing

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
    onRemovePhoto: ((String) -> Unit)?,
    onFramePhoto: ((String) -> Unit)?,
    onMakeCover: ((String) -> Unit)?,
    onPlaceFreely: ((String) -> Unit)?,
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

        photos.flowing().forEachIndexed { index, photo ->
            NotePhoto(
                photo = photo,
                isCover = index == 0,
                onRemove = onRemovePhoto?.let { remove -> { remove(photo.id) } },
                onFrame = onFramePhoto?.let { frame -> { frame(photo.id) } },
                onMakeCover = onMakeCover
                    ?.takeIf { index > 0 }
                    ?.let { promote -> { promote(photo.id) } },
                onPlaceFreely = onPlaceFreely?.let { place -> { place(photo.id) } },
            )
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

@OptIn(ExperimentalFoundationApi::class)
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
    val requester = remember { BringIntoViewRequester() }
    var field by remember { mutableStateOf(TextFieldValue(value, TextRange(value.length))) }
    var layout by remember { mutableStateOf<TextLayoutResult?>(null) }
    var focused by remember { mutableStateOf(false) }

    if (field.text != value) {
        field = TextFieldValue(value, TextRange(value.length))
    }

    LaunchedEffect(field, focused, layout) {
        val drawn = layout
        if (!focused || drawn == null) {
            return@LaunchedEffect
        }
        val laid = drawn.layoutInput.text.length
        val caret = field.selection.end.coerceIn(0, laid)
        requester.bringIntoView(drawn.getCursorRect(caret))
    }

    BasicTextField(
        value = field,
        onValueChange = { next ->
            field = next
            if (next.text != value) {
                onValueChange(next.text)
            }
        },
        modifier = modifier
            .fillMaxWidth()
            .bringIntoViewRequester(requester)
            .onFocusChanged { focused = it.isFocused },
        textStyle = textStyle,
        singleLine = singleLine,
        cursorBrush = SolidColor(colors.ink),
        onTextLayout = { layout = it },
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
