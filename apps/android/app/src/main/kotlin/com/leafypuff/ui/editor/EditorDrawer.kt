package com.leafypuff.ui.editor

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ColumnScope
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LocalLeafyColors

private val DrawerPadding = PaddingValues(start = 24.dp, top = 14.dp, end = 24.dp, bottom = 16.dp)
private val DrawerGap = 10.dp

@Composable
fun EditorDrawer(modifier: Modifier = Modifier, content: @Composable ColumnScope.() -> Unit) {
    Column(
        modifier = modifier
            .fillMaxWidth()
            .background(LocalLeafyColors.current.surface)
            .padding(DrawerPadding),
        verticalArrangement = Arrangement.spacedBy(DrawerGap),
        content = content,
    )
}
