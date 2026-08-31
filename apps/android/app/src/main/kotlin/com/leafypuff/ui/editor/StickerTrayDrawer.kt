package com.leafypuff.ui.editor

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

private val TrayHeight = 52.dp
private val Hairline = 0.5.dp

@Composable
fun StickerTrayDrawer(modifier: Modifier = Modifier) {
    val colors = LocalLeafyColors.current

    EditorDrawer(modifier = modifier) {
        Text(
            text = "Stickers land in a later slice.",
            style = LocalLeafyTypography.current.chipLabel,
            color = colors.ink3,
        )
        Box(
            modifier = Modifier
                .fillMaxWidth()
                .height(TrayHeight)
                .clip(LeafyShapes.stickerTrayTile)
                .background(colors.sheet)
                .border(Hairline, colors.line, LeafyShapes.stickerTrayTile),
        )
    }
}
