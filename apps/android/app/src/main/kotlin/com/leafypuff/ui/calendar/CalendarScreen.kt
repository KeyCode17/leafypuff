package com.leafypuff.ui.calendar

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.leafypuff.domain.Entry
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.diary.EntryCard
import com.leafypuff.ui.diary.formatEntryDate
import kotlinx.datetime.Clock
import kotlinx.datetime.LocalDate
import kotlinx.datetime.TimeZone
import kotlinx.datetime.todayIn

private val ScreenGutter = 24.dp
private val ScrollBottomPadding = 130.dp
private val SelectedBlockTopPadding = 28.dp
private val SelectedBlockGap = 14.dp

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
    hasPhoto: (Entry) -> Boolean = { false },
) {
    val today = remember { Clock.System.todayIn(TimeZone.currentSystemDefault()) }
    val entriesByDate = remember(entries) { entries.groupBy { it.date } }

    Column(
        modifier = modifier
            .fillMaxSize()
            .verticalScroll(rememberScrollState())
            .padding(horizontal = ScreenGutter)
            .padding(bottom = ScrollBottomPadding),
    ) {
        CalendarHeader(
            visibleMonth = visibleMonth,
            onMonthChange = onMonthChange,
            onToday = onToday,
        )
        WeekdayRow()
        CalendarGrid(
            visibleMonth = visibleMonth,
            entriesByDate = entriesByDate,
            today = today,
            selected = selected,
            hasPhoto = hasPhoto,
            onSelect = onSelect,
        )
        SelectedDayBlock(
            selected = selected,
            selectedEntries = entriesByDate[selected].orEmpty(),
            onCreateForSelected = onCreateForSelected,
        )
    }
}

@Composable
private fun SelectedDayBlock(
    selected: LocalDate,
    selectedEntries: List<Entry>,
    onCreateForSelected: () -> Unit,
) {
    val colors = LocalLeafyColors.current

    Column(
        modifier = Modifier
            .fillMaxWidth()
            .padding(top = SelectedBlockTopPadding),
        verticalArrangement = Arrangement.spacedBy(SelectedBlockGap),
    ) {
        Text(
            text = formatEntryDate(selected).uppercase(),
            style = LocalLeafyTypography.current.metaLabel,
            color = colors.ink3,
        )
        if (selectedEntries.isEmpty()) {
            CalendarEmptyState(onCreate = onCreateForSelected)
        } else {
            selectedEntries.forEach { entry -> EntryCard(entry) }
        }
    }
}
