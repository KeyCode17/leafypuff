package com.leafypuff.ui.calendar

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.KeyboardArrowLeft
import androidx.compose.material.icons.automirrored.filled.KeyboardArrowRight
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.common.formatShortMonth
import com.leafypuff.ui.popups.PopupShell
import kotlinx.datetime.LocalDate

private const val MonthsInYear = 12
private const val MonthsPerRow = 3
private val YearRowPaddingH = 4.dp
private val YearRowPaddingBottom = 10.dp
private val PillGap = 8.dp
private val PillPaddingV = 10.dp

@Composable
fun MonthJumpPopup(
    visibleMonth: LocalDate,
    onJump: (LocalDate) -> Unit,
    onDismiss: () -> Unit,
) {
    val colors = LocalLeafyColors.current
    var year by remember { mutableIntStateOf(visibleMonth.year) }

    PopupShell(onDismiss = onDismiss) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(start = YearRowPaddingH, end = YearRowPaddingH, bottom = YearRowPaddingBottom),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically,
        ) {
            NavButton(
                glyph = Icons.AutoMirrored.Filled.KeyboardArrowLeft,
                label = "Previous year",
                onClick = { year -= 1 },
            )
            Text(
                text = year.toString(),
                style = LocalLeafyTypography.current.monthLabel,
                color = colors.ink,
            )
            NavButton(
                glyph = Icons.AutoMirrored.Filled.KeyboardArrowRight,
                label = "Next year",
                onClick = { year += 1 },
            )
        }
        Column(verticalArrangement = Arrangement.spacedBy(PillGap)) {
            (1..MonthsInYear).chunked(MonthsPerRow).forEach { row ->
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.spacedBy(PillGap),
                ) {
                    row.forEach { month ->
                        MonthPill(
                            month = month,
                            chosen = year == visibleMonth.year && month == visibleMonth.monthNumber,
                            onClick = { onJump(LocalDate(year, month, 1)) },
                            modifier = Modifier.weight(1f),
                        )
                    }
                }
            }
        }
    }
}

@Composable
private fun MonthPill(month: Int, chosen: Boolean, onClick: () -> Unit, modifier: Modifier) {
    val colors = LocalLeafyColors.current

    Text(
        text = formatShortMonth(month),
        style = LocalLeafyTypography.current.chipLabel,
        color = if (chosen) colors.onAccent else colors.ink,
        textAlign = TextAlign.Center,
        modifier = modifier
            .clip(LeafyShapes.pill)
            .background(if (chosen) colors.accent else colors.soft2)
            .clickable(onClick = onClick)
            .padding(vertical = PillPaddingV),
    )
}
