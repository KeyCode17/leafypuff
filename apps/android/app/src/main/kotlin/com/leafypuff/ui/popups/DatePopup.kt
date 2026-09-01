package com.leafypuff.ui.popups

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.KeyboardArrowLeft
import androidx.compose.material.icons.automirrored.filled.KeyboardArrowRight
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.common.formatMonthYear
import kotlinx.datetime.DateTimeUnit
import kotlinx.datetime.LocalDate
import kotlinx.datetime.minus
import kotlinx.datetime.plus

private val HeaderPaddingH = 2.dp
private val HeaderPaddingBottom = 10.dp
private val NavButtonSize = 30.dp
private val NavGlyphSize = 14.dp
private val MonthLabelSize = 15.sp

@Composable
fun DatePopup(
    selected: LocalDate,
    onSelect: (LocalDate) -> Unit,
    onDismiss: () -> Unit,
) {
    var visibleMonth by remember {
        mutableStateOf(LocalDate(selected.year, selected.monthNumber, 1))
    }

    PopupShell(onDismiss = onDismiss) {
        DateHeader(
            visibleMonth = visibleMonth,
            onPrevious = { visibleMonth = visibleMonth.minus(1, DateTimeUnit.MONTH) },
            onNext = { visibleMonth = visibleMonth.plus(1, DateTimeUnit.MONTH) },
        )
        DateWeekdayRow()
        DateGrid(visibleMonth = visibleMonth, selected = selected, onSelect = onSelect)
    }
}

@Composable
private fun DateHeader(visibleMonth: LocalDate, onPrevious: () -> Unit, onNext: () -> Unit) {
    val colors = LocalLeafyColors.current

    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(
                start = HeaderPaddingH,
                end = HeaderPaddingH,
                bottom = HeaderPaddingBottom,
            ),
        horizontalArrangement = Arrangement.SpaceBetween,
        verticalAlignment = Alignment.CenterVertically,
    ) {
        NavButton(
            glyph = Icons.AutoMirrored.Filled.KeyboardArrowLeft,
            label = "Previous month",
            onClick = onPrevious,
        )
        Text(
            text = formatMonthYear(visibleMonth),
            style = LocalLeafyTypography.current.monthLabel.copy(fontSize = MonthLabelSize),
            color = colors.ink,
        )
        NavButton(
            glyph = Icons.AutoMirrored.Filled.KeyboardArrowRight,
            label = "Next month",
            onClick = onNext,
        )
    }
}

@Composable
private fun NavButton(glyph: ImageVector, label: String, onClick: () -> Unit) {
    val colors = LocalLeafyColors.current

    Box(
        modifier = Modifier
            .size(NavButtonSize)
            .clip(LeafyShapes.pill)
            .background(colors.soft2)
            .clickable(onClick = onClick),
        contentAlignment = Alignment.Center,
    ) {
        Icon(
            imageVector = glyph,
            contentDescription = label,
            tint = colors.ink,
            modifier = Modifier.size(NavGlyphSize),
        )
    }
}
