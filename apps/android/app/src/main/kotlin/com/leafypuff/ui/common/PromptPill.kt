package com.leafypuff.ui.common

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.em
import androidx.compose.ui.unit.sp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.Rubik

private val PillPaddingY = 9.dp

private val PillTextStyle = TextStyle(
    fontFamily = Rubik,
    fontSize = 12.sp,
    letterSpacing = 0.04.em,
)

@Composable
fun PromptPill(
    label: String,
    fill: Color,
    ink: Color,
    weight: FontWeight,
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
) {
    Text(
        text = label.uppercase(),
        style = PillTextStyle.copy(fontWeight = weight),
        color = ink,
        textAlign = TextAlign.Center,
        modifier = modifier
            .clip(LeafyShapes.pill)
            .background(fill)
            .clickable(onClick = onClick)
            .padding(vertical = PillPaddingY),
    )
}
