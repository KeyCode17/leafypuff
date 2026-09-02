package com.leafypuff.ui.settings

import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.layout.ContentScale
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.text.BasicTextField
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Edit
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.SolidColor
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.leafypuff.domain.Mood
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.common.BunnyFace
import kotlinx.datetime.LocalDate
import com.leafypuff.ui.common.formatMonthYear

private val AvatarSize = 56.dp
private val AvatarBunnySize = 44.dp
private val NameColumnGap = 4.dp
private val EditGlyphSize = 20.dp
private val NameFontSize = 18.sp
private val SinceFontSize = 12.sp
internal fun formatWritingSince(date: LocalDate): String = "Writing since ${formatMonthYear(date)}"

@Composable
internal fun SettingsProfileCard(
    name: String,
    writingSince: LocalDate?,
    avatar: ImageBitmap?,
    onNameChange: (String) -> Unit,
    onEditProfile: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current

    SettingsCard(padding = BlockCardPadding, modifier = modifier) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(CardContentGap),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Box(
                modifier = Modifier
                    .size(AvatarSize)
                    .clip(LeafyShapes.pill)
                    .background(colors.soft2),
                contentAlignment = Alignment.Center,
            ) {
                if (avatar == null) {
                    BunnyFace(mood = Mood.Calm, modifier = Modifier.size(AvatarBunnySize))
                } else {
                    Image(
                        bitmap = avatar,
                        contentDescription = null,
                        contentScale = ContentScale.Crop,
                        modifier = Modifier.fillMaxSize(),
                    )
                }
            }
            NameBlock(name = name, writingSince = writingSince, onNameChange = onNameChange)
            Icon(
                imageVector = Icons.Filled.Edit,
                contentDescription = "Edit your profile",
                tint = colors.ink3,
                modifier = Modifier
                    .clickable(onClick = onEditProfile)
                    .size(EditGlyphSize),
            )
        }
    }
}

@Composable
private fun RowScope.NameBlock(
    name: String,
    writingSince: LocalDate?,
    onNameChange: (String) -> Unit,
) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Column(
        modifier = Modifier.weight(1f),
        verticalArrangement = Arrangement.spacedBy(NameColumnGap),
    ) {
        BasicTextField(
            value = name,
            onValueChange = onNameChange,
            singleLine = true,
            textStyle = typography.noteTitleInput.copy(
                fontSize = NameFontSize,
                color = colors.ink,
            ),
            cursorBrush = SolidColor(colors.accent),
            modifier = Modifier.fillMaxWidth(),
            decorationBox = { field ->
                if (name.isEmpty()) {
                    Text(
                        text = "Your name",
                        style = typography.noteTitleInput.copy(fontSize = NameFontSize),
                        color = colors.ink3,
                    )
                }
                field()
            },
        )
        if (writingSince != null) {
            Text(
                text = formatWritingSince(writingSince),
                style = typography.chipLabel.copy(
                    fontSize = SinceFontSize,
                    fontWeight = FontWeight.W400,
                ),
                color = colors.ink3,
            )
        }
    }
}
