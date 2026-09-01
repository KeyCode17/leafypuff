package com.leafypuff.ui.shell

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyBrush
import com.leafypuff.theme.LeafyElevation
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

private val BarHeight = 84.dp
private val FabSize = 68.dp
private val CentreGap = 80.dp
private val GlyphSize = 22.dp
private val FabGlyphSize = 30.dp
private val GlyphLabelGap = 4.dp

@Composable
fun BottomNav(
    current: Destination,
    onSelect: (Destination) -> Unit,
    onCompose: () -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current

    Box(modifier = modifier.fillMaxWidth()) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .height(BarHeight)
                .align(Alignment.BottomCenter)
                .shadow(LeafyElevation.nav)
                .background(colors.surface),
            horizontalArrangement = Arrangement.Center,
            verticalAlignment = Alignment.CenterVertically,
        ) {
            NavSlot(Destination.Diary, current, onSelect, Modifier.weight(1f))
            NavSlot(Destination.Calendar, current, onSelect, Modifier.weight(1f))
            Spacer(Modifier.width(CentreGap))
            NavSlot(Destination.Statistics, current, onSelect, Modifier.weight(1f))
            NavSlot(Destination.Settings, current, onSelect, Modifier.weight(1f))
        }

        Box(
            modifier = Modifier
                .align(Alignment.BottomCenter)
                .padding(bottom = BarHeight - FabSize / 2)
                .size(FabSize)
                .shadow(
                    elevation = LeafyElevation.glow,
                    shape = CircleShape,
                    ambientColor = colors.accent,
                    spotColor = colors.accent,
                )
                .clip(CircleShape)
                .background(LeafyBrush.fab(colors.accent))
                .clickable(onClick = onCompose),
            contentAlignment = Alignment.Center,
        ) {
            Icon(
                imageVector = Icons.Filled.Add,
                contentDescription = "Write a new entry",
                tint = colors.onAccent,
                modifier = Modifier.size(FabGlyphSize),
            )
        }
    }
}

@Composable
private fun NavSlot(
    destination: Destination,
    current: Destination,
    onSelect: (Destination) -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current
    val active = destination == current

    val tint = if (active) colors.accent else colors.ink3

    Column(
        modifier = modifier
            .clickable { onSelect(destination) }
            .padding(vertical = 8.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(GlyphLabelGap),
    ) {
        Icon(
            imageVector = destination.glyph,
            contentDescription = null,
            tint = tint,
            modifier = Modifier.size(GlyphSize),
        )
        Text(
            text = destination.label.uppercase(),
            style = LocalLeafyTypography.current.metaLabel,
            color = tint,
            textAlign = TextAlign.Center,
        )
    }
}
