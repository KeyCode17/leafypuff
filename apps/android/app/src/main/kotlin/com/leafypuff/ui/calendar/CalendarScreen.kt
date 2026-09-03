package com.leafypuff.ui.calendar

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.unit.dp
import com.leafypuff.domain.Entry
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.common.formatEntryDate
import kotlinx.datetime.Clock
import kotlinx.datetime.LocalDate
import kotlinx.datetime.TimeZone
import kotlinx.datetime.todayIn

private val ScreenGutter = 24.dp
private val ScrollBottomPadding = 130.dp
private val SelectedBlockTopPadding = 28.dp
private val SelectedBlockGap = 14.dp
private const val GridShare = 0.625f

@Composable
fun CalendarScreen(
    entries: List<Entry>,
    visibleMonth: LocalDate,
    selected: LocalDate,
    onMonthChange: (LocalDate) -> Unit,
    onSelect: (LocalDate) -> Unit,
    onToday: () -> Unit,
    onCreateForSelected: () -> Unit,
    modifier: Modifier = Modifier,
    covers: Map<String, ImageBitmap> = emptyMap(),
    onOpen: (Entry) -> Unit = { },
) {
    val today = remember { Clock.System.todayIn(TimeZone.currentSystemDefault()) }
    val entriesByDate = remember(entries) { entries.groupBy { it.date } }
    var jumping by remember { mutableStateOf(false) }

    Column(
        modifier = modifier
            .fillMaxSize()
            .padding(horizontal = ScreenGutter),
    ) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .fillMaxHeight(GridShare),
        ) {
            CalendarHeader(
                visibleMonth = visibleMonth,
                onMonthChange = onMonthChange,
                onToday = onToday,
                onJump = { jumping = true },
            )
            WeekdayRow()
            CalendarGrid(
                visibleMonth = visibleMonth,
                entriesByDate = entriesByDate,
                today = today,
                selected = selected,
                covers = covers,
                onSelect = onSelect,
                modifier = Modifier.weight(1f),
            )
        }
        SelectedDayList(
            selected = selected,
            selectedEntries = entriesByDate[selected].orEmpty(),
            covers = covers,
            onCreateForSelected = onCreateForSelected,
            onOpen = onOpen,
            modifier = Modifier.weight(1f),
        )
    }

    if (jumping) {
        MonthJumpPopup(
            visibleMonth = visibleMonth,
            onJump = {
                onMonthChange(it)
                jumping = false
            },
            onDismiss = { jumping = false },
        )
    }
}

@Composable
private fun SelectedDayList(
    selected: LocalDate,
    selectedEntries: List<Entry>,
    covers: Map<String, ImageBitmap>,
    onCreateForSelected: () -> Unit,
    onOpen: (Entry) -> Unit,
    modifier: Modifier = Modifier,
) {
    val colors = LocalLeafyColors.current

    LazyColumn(
        modifier = modifier.fillMaxWidth(),
        contentPadding = PaddingValues(top = SelectedBlockTopPadding, bottom = ScrollBottomPadding),
        verticalArrangement = Arrangement.spacedBy(SelectedBlockGap),
    ) {
        item {
            Text(
                text = formatEntryDate(selected).uppercase(),
                style = LocalLeafyTypography.current.metaLabel,
                color = colors.ink3,
            )
        }
        if (selectedEntries.isEmpty()) {
            item { CalendarEmptyState(onCreate = onCreateForSelected) }
        } else {
            items(selectedEntries, key = { it.id }) { entry ->
                CalendarEntryRow(
                    entry = entry,
                    cover = covers[entry.id],
                    onOpen = { onOpen(entry) },
                )
            }
        }
    }
}
