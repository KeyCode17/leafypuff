package com.leafypuff.ui.editor

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Close
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

private val HeaderTopPadding = 16.dp
private val HeaderBottomPadding = 20.dp
private val CloseButtonSize = 36.dp
private val CloseGlyphSize = 18.dp
private val Hairline = 0.5.dp
private val HeaderTitleSize = 14.sp

@Composable
fun EntryEditorHeader(
    title: String,
    onClose: () -> Unit,
    onSave: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Row(
        modifier = modifier
            .fillMaxWidth()
            .padding(top = HeaderTopPadding, bottom = HeaderBottomPadding),
        horizontalArrangement = Arrangement.SpaceBetween,
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Box(
            modifier = Modifier
                .size(CloseButtonSize)
                .clip(LeafyShapes.pill)
                .background(colors.surface)
                .border(Hairline, colors.line, LeafyShapes.pill)
                .clickable(onClick = onClose),
            contentAlignment = Alignment.Center,
        ) {
            Icon(
                imageVector = Icons.Filled.Close,
                contentDescription = "Close entry editor",
                tint = colors.ink,
                modifier = Modifier.size(CloseGlyphSize),
            )
        }

        Text(
            text = title,
            style = typography.cardTitle.copy(fontSize = HeaderTitleSize),
            color = colors.ink,
        )

        Text(
            text = "Save".uppercase(),
            style = typography.buttonLabel,
            color = colors.accentDeep,
            modifier = Modifier
                .clip(LeafyShapes.pill)
                .clickable(onClick = onSave)
                .padding(horizontal = 4.dp, vertical = 4.dp),
        )
    }
}
