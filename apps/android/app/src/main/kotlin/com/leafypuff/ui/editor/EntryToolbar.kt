package com.leafypuff.ui.editor

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Star
import androidx.compose.material.icons.outlined.Star
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyStroke
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

enum class EditorTool { Sticker, Hashtag }

private val ToolbarHeight = 78.dp
private val ToolbarPaddingX = 20.dp
private val ToolbarPaddingTop = 14.dp
private val SlotWidth = 44.dp
private val SlotHeight = 40.dp
private val CameraSize = 22.dp
private val StarSize = 23.dp

@Composable
fun EntryToolbar(
    tool: EditorTool?,
    onToggleTool: (EditorTool) -> Unit,
    onAddPhoto: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current

    Box(
        modifier = modifier
            .fillMaxWidth()
            .height(ToolbarHeight)
            .background(colors.surface),
    ) {
        Box(
            modifier = Modifier
                .fillMaxWidth()
                .height(LeafyStroke.hairline)
                .background(colors.line),
        )

        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(start = ToolbarPaddingX, end = ToolbarPaddingX, top = ToolbarPaddingTop),
            horizontalArrangement = Arrangement.SpaceAround,
            verticalAlignment = Alignment.Top,
        ) {
            ToolSlot(onClick = onAddPhoto) {
                CameraGlyph(tint = colors.ink2, modifier = Modifier.size(CameraSize))
            }

            ToolSlot(onClick = { onToggleTool(EditorTool.Sticker) }) {
                val active = tool == EditorTool.Sticker
                Icon(
                    imageVector = if (active) Icons.Filled.Star else Icons.Outlined.Star,
                    contentDescription = "Sticker tray",
                    tint = if (active) colors.accent else colors.ink2,
                    modifier = Modifier.size(StarSize),
                )
            }

            ToolSlot(onClick = { onToggleTool(EditorTool.Hashtag) }) {
                val active = tool == EditorTool.Hashtag
                Text(
                    text = "#",
                    style = LocalLeafyTypography.current.noteTitleInput,
                    color = if (active) colors.accentDeep else colors.ink2,
                )
            }
        }
    }
}

@Composable
private fun ToolSlot(onClick: () -> Unit, content: @Composable () -> Unit) {
    Box(
        modifier = Modifier
            .width(SlotWidth)
            .height(SlotHeight)
            .clickable(onClick = onClick),
        contentAlignment = Alignment.Center,
    ) {
        content()
    }
}
