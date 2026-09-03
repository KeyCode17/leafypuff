package com.leafypuff.ui.calendar

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.leafypuff.domain.Entry
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.common.BunnyFace

private val RowPadding = 12.dp
private val RowGap = 12.dp
private val ThumbSize = 56.dp
private val ThumbShape = RoundedCornerShape(12.dp)
private val TextGap = 3.dp
private val FaceSize = 26.dp

@Composable
fun CalendarEntryRow(
    entry: Entry,
    cover: ImageBitmap?,
    onOpen: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Row(
        modifier = modifier
            .fillMaxWidth()
            .clip(LeafyShapes.card)
            .background(colors.sheet)
            .clickable(onClick = onOpen)
            .padding(RowPadding),
        horizontalArrangement = Arrangement.spacedBy(RowGap),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Box(
            modifier = Modifier
                .size(ThumbSize)
                .clip(ThumbShape)
                .background(colors.soft2),
        ) {
            if (cover != null) {
                Image(
                    bitmap = cover,
                    contentDescription = null,
                    contentScale = ContentScale.Crop,
                    modifier = Modifier.fillMaxSize(),
                )
            }
        }
        Column(
            modifier = Modifier.weight(1f),
            verticalArrangement = Arrangement.spacedBy(TextGap),
        ) {
            Text(
                text = entry.title,
                style = typography.cardTitle,
                color = colors.ink,
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
            )
            Text(
                text = entry.body,
                style = typography.body,
                color = colors.ink2,
                maxLines = 1,
                overflow = TextOverflow.Ellipsis,
            )
        }
        BunnyFace(mood = entry.mood, modifier = Modifier.size(FaceSize))
    }
}
