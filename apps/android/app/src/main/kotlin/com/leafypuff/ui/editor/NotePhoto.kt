package com.leafypuff.ui.editor

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

private val PhotoHeight = 190.dp
private val PhotoShape = RoundedCornerShape(16.dp)
private val BadgeOffset = 10.dp
private val BadgePaddingX = 9.dp
private val BadgePaddingY = 4.dp
private val BadgeTextSize = 9.sp
private val BadgeScrim = Color(0x8C242D35)

@Composable
fun NotePhoto(photoId: String, isCover: Boolean, modifier: Modifier = Modifier) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Box(
        modifier = modifier
            .fillMaxWidth()
            .height(PhotoHeight)
            .clip(PhotoShape)
            .background(colors.soft2),
    ) {
        Text(
            text = photoId,
            style = typography.metaLabel,
            color = colors.ink3,
            modifier = Modifier.align(Alignment.Center),
        )

        if (isCover) {
            Text(
                text = "Diary thumbnail".uppercase(),
                style = typography.metaLabel.copy(fontSize = BadgeTextSize),
                color = Color.White,
                modifier = Modifier
                    .align(Alignment.TopStart)
                    .padding(BadgeOffset)
                    .clip(LeafyShapes.pill)
                    .background(BadgeScrim)
                    .padding(horizontal = BadgePaddingX, vertical = BadgePaddingY),
            )
        }
    }
}
