package com.leafypuff.ui.stats

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ColumnScope
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

internal val CardPadding = 18.dp
internal val CardGap = 14.dp
internal val BlockGap = 18.dp

@Composable
fun StatsCard(
    modifier: Modifier = Modifier,
    gap: Dp = CardGap,
    content: @Composable ColumnScope.() -> Unit,
) {
    Column(
        modifier = modifier
            .fillMaxWidth()
            .clip(LeafyShapes.card)
            .background(LocalLeafyColors.current.sheet)
            .padding(CardPadding),
        verticalArrangement = Arrangement.spacedBy(gap),
        content = content,
    )
}

@Composable
fun StatsCardLabel(text: String) {
    Text(
        text = text.uppercase(),
        style = LocalLeafyTypography.current.metaLabel,
        color = LocalLeafyColors.current.ink3,
    )
}
