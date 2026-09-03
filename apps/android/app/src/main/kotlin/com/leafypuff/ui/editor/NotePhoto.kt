package com.leafypuff.ui.editor

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Close
import androidx.compose.material3.Icon
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.crop.PhotoFraming
import com.leafypuff.ui.photo.EntryPhoto

private val PhotoShape = RoundedCornerShape(16.dp)
private val RemoveGlyphSize = 18.dp
private val RemovePadding = 5.dp
private val BadgeOffset = 10.dp
private val BadgePaddingX = 9.dp
private val BadgePaddingY = 4.dp
private val BadgeTextSize = 9.sp
private val BadgeScrim = Color(0x8C242D35)

@Composable
fun NotePhoto(
    photo: EntryPhoto,
    isCover: Boolean,
    onRemove: (() -> Unit)?,
    onFrame: (() -> Unit)?,
    onMakeCover: (() -> Unit)?,
    onPlaceFreely: (() -> Unit)?,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Box(
        modifier = modifier
            .fillMaxWidth()
            .aspectRatio(PhotoFraming.CoverAspect)
            .clip(PhotoShape)
            .background(colors.soft2)
            .then(if (onFrame == null) Modifier else Modifier.clickable(onClick = onFrame)),
    ) {
        Image(
            bitmap = photo.cover,
            contentDescription = null,
            contentScale = ContentScale.Crop,
            modifier = Modifier.fillMaxSize(),
        )

        if (isCover || onMakeCover != null) {
            Text(
                text = (if (isCover) "Diary thumbnail" else "Make thumbnail").uppercase(),
                style = typography.metaLabel.copy(fontSize = BadgeTextSize),
                color = Color.White,
                modifier = Modifier
                    .align(Alignment.TopStart)
                    .padding(BadgeOffset)
                    .clip(LeafyShapes.pill)
                    .background(BadgeScrim)
                    .then(
                        if (onMakeCover == null) {
                            Modifier
                        } else {
                            Modifier.clickable(onClick = onMakeCover)
                        },
                    )
                    .padding(horizontal = BadgePaddingX, vertical = BadgePaddingY),
            )
        }

        if (onPlaceFreely != null) {
            Text(
                text = "Place freely".uppercase(),
                style = typography.metaLabel.copy(fontSize = BadgeTextSize),
                color = Color.White,
                modifier = Modifier
                    .align(Alignment.BottomStart)
                    .padding(BadgeOffset)
                    .clip(LeafyShapes.pill)
                    .background(BadgeScrim)
                    .clickable(onClick = onPlaceFreely)
                    .padding(horizontal = BadgePaddingX, vertical = BadgePaddingY),
            )
        }

        if (onRemove != null) {
            Icon(
                imageVector = Icons.Filled.Close,
                contentDescription = "Remove this photo",
                tint = Color.White,
                modifier = Modifier
                    .align(Alignment.TopEnd)
                    .padding(BadgeOffset)
                    .clip(CircleShape)
                    .background(BadgeScrim)
                    .clickable(onClick = onRemove)
                    .padding(RemovePadding)
                    .size(RemoveGlyphSize),
            )
        }
    }
}
