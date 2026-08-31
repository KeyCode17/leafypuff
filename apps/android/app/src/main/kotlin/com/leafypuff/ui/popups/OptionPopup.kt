package com.leafypuff.ui.popups

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography

private val TitlePaddingH = 12.dp
private val TitlePaddingTop = 2.dp
private val TitlePaddingBottom = 8.dp
private val RowPaddingH = 12.dp
private val RowPaddingV = 11.dp
private val RowGap = 10.dp
private val CheckSize = 16.dp

const val WeatherTitle = "Weather"
const val LocationTitle = "Where were you?"

val WeatherOptions = listOf("Sunny", "Cloudy", "Rainy", "Windy")
val LocationOptions = listOf("Home", "Cafe", "Office", "Park", "On the road")

@Composable
fun OptionPopup(
    title: String,
    options: List<String>,
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
                label = option,
                checked = option == selected,
                onClick = { onSelect(option) },
            )
        }
    }
}

@Composable
private fun OptionRow(label: String, checked: Boolean, onClick: () -> Unit) {
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
        Text(
            text = label,
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
