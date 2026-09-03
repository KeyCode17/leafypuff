package com.leafypuff.ui.popups

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Air
import androidx.compose.material.icons.filled.Check
import androidx.compose.material.icons.filled.Cloud
import androidx.compose.material.icons.filled.DirectionsCar
import androidx.compose.material.icons.filled.Home
import androidx.compose.material.icons.filled.LocalCafe
import androidx.compose.material.icons.filled.Park
import androidx.compose.material.icons.filled.Umbrella
import androidx.compose.material.icons.filled.WbSunny
import androidx.compose.material.icons.filled.Work
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.unit.dp
import com.leafypuff.core.Location
import com.leafypuff.core.Weather
import com.leafypuff.data.label
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

private val TitlePaddingH = 12.dp
private val TitlePaddingTop = 2.dp
private val TitlePaddingBottom = 8.dp
private val RowPaddingH = 12.dp
private val RowPaddingV = 11.dp
private val RowGap = 10.dp
private val GlyphSize = 18.dp
private val CheckSize = 16.dp

const val WeatherTitle = "Weather"
const val LocationTitle = "Location"

data class PopupOption(val label: String, val glyph: ImageVector)

val WeatherOptions: List<PopupOption> = Weather.entries.map { PopupOption(it.label(), it.glyph()) }
val LocationOptions: List<PopupOption> =
    Location.entries.map { PopupOption(it.label(), it.glyph()) }

fun List<PopupOption>.glyphOf(label: String?): ImageVector? =
    firstOrNull { it.label == label }?.glyph

private fun Weather.glyph(): ImageVector = when (this) {
    Weather.SUNNY -> Icons.Filled.WbSunny
    Weather.CLOUDY -> Icons.Filled.Cloud
    Weather.RAINY -> Icons.Filled.Umbrella
    Weather.WINDY -> Icons.Filled.Air
}

private fun Location.glyph(): ImageVector = when (this) {
    Location.HOME -> Icons.Filled.Home
    Location.CAFE -> Icons.Filled.LocalCafe
    Location.OFFICE -> Icons.Filled.Work
    Location.PARK -> Icons.Filled.Park
    Location.ON_THE_ROAD -> Icons.Filled.DirectionsCar
}

@Composable
fun OptionPopup(
    title: String,
    options: List<PopupOption>,
    selected: String?,
    onSelect: (String) -> Unit,
    onDismiss: () -> Unit,
) {
    val colors = LocalLeafyColors.current

    PopupShell(onDismiss = onDismiss) {
        Text(
            text = title.uppercase(),
            style = LocalLeafyTypography.current.metaLabel,
            color = colors.ink3,
            modifier = Modifier.padding(
                start = TitlePaddingH,
                end = TitlePaddingH,
                top = TitlePaddingTop,
                bottom = TitlePaddingBottom,
            ),
        )
        options.forEach { option ->
            OptionRow(
                option = option,
                checked = option.label == selected,
                onClick = { onSelect(option.label) },
            )
        }
    }
}

@Composable
private fun OptionRow(option: PopupOption, checked: Boolean, onClick: () -> Unit) {
    val colors = LocalLeafyColors.current

    Row(
        modifier = Modifier
            .fillMaxWidth()
            .clip(LeafyShapes.input)
            .clickable(onClick = onClick)
            .padding(horizontal = RowPaddingH, vertical = RowPaddingV),
        horizontalArrangement = Arrangement.spacedBy(RowGap),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Icon(
            imageVector = option.glyph,
            contentDescription = null,
            tint = if (checked) colors.accentDeep else colors.ink2,
            modifier = Modifier.size(GlyphSize),
        )
        Text(
            text = option.label,
            style = LocalLeafyTypography.current.body,
            color = colors.ink,
            modifier = Modifier.weight(1f),
        )
        if (checked) {
            Icon(
                imageVector = Icons.Filled.Check,
                contentDescription = null,
                tint = colors.accentDeep,
                modifier = Modifier.size(CheckSize),
            )
        }
    }
}
