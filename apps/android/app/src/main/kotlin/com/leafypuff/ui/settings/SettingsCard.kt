package com.leafypuff.ui.settings

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ColumnScope
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors

internal val CardSpacing = 18.dp
internal val CardRowPaddingV = 16.dp
internal val CardContentGap = 14.dp
internal val ListCardPadding = PaddingValues(horizontal = 18.dp, vertical = 6.dp)
internal val BlockCardPadding = PaddingValues(18.dp)

private val CardElevation = 8.dp
private val Hairline = 0.5.dp

@Composable
internal fun SettingsCard(
    padding: PaddingValues,
    modifier: Modifier = Modifier,
    verticalArrangement: Arrangement.Vertical = Arrangement.Top,
    content: @Composable ColumnScope.() -> Unit,
) {
    val colors = LocalLeafyColors.current

    Column(
        modifier = modifier
            .fillMaxWidth()
            .shadow(CardElevation, LeafyShapes.card)
            .clip(LeafyShapes.card)
            .background(colors.sheet)
            .padding(padding),
        verticalArrangement = verticalArrangement,
        content = content,
    )
}

@Composable
internal fun SettingsDivider(modifier: Modifier = Modifier) {
    Box(
        modifier = modifier
            .fillMaxWidth()
            .height(Hairline)
            .background(LocalLeafyColors.current.line),
    )
}
