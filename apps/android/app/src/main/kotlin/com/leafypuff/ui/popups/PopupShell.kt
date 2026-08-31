package com.leafypuff.ui.popups

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ColumnScope
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.SideEffect
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalView
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog
import androidx.compose.ui.window.DialogProperties
import androidx.compose.ui.window.DialogWindowProvider
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors

private val Scrim = Color(0x59242D35)
private val PopupGutter = 44.dp
private val PopupPadding = 16.dp
private val PopupGap = 2.dp
private val PopupElevation = 18.dp

@Composable
internal fun PopupShell(onDismiss: () -> Unit, content: @Composable ColumnScope.() -> Unit) {
    Dialog(
        onDismissRequest = onDismiss,
        properties = DialogProperties(usePlatformDefaultWidth = false),
    ) {
        val view = LocalView.current
        SideEffect { (view.parent as? DialogWindowProvider)?.window?.setDimAmount(0f) }

        Box(
            modifier = Modifier
                .fillMaxSize()
                .background(Scrim)
                .noRippleClick(onDismiss)
                .padding(horizontal = PopupGutter),
            contentAlignment = Alignment.Center,
        ) {
            PopupCard(content)
        }
    }
}

@Composable
private fun PopupCard(content: @Composable ColumnScope.() -> Unit) {
    val colors = LocalLeafyColors.current

    Column(
        modifier = Modifier
            .fillMaxWidth()
            .shadow(PopupElevation, LeafyShapes.popup)
            .clip(LeafyShapes.popup)
            .background(colors.sheet)
            .noRippleClick { }
            .padding(PopupPadding),
        verticalArrangement = Arrangement.spacedBy(PopupGap),
        content = content,
    )
}

@Composable
private fun Modifier.noRippleClick(onClick: () -> Unit): Modifier {
    val interactionSource = remember { MutableInteractionSource() }
    return clickable(interactionSource = interactionSource, indication = null, onClick = onClick)
}
