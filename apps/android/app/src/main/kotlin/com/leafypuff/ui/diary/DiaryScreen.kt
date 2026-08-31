package com.leafypuff.ui.diary

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.leafypuff.domain.Entry
import com.leafypuff.domain.Mood
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.common.BunnyFace

@Composable
fun DiaryScreen(entries: List<Entry>, modifier: Modifier = Modifier) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    LazyColumn(
        modifier = modifier.fillMaxSize(),
        contentPadding = PaddingValues(start = 24.dp, end = 24.dp, bottom = 130.dp),
        verticalArrangement = Arrangement.spacedBy(20.dp),
    ) {
        item {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(top = 20.dp, bottom = 4.dp),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.Top,
            ) {
                Column(verticalArrangement = Arrangement.spacedBy(4.dp)) {
                    Text("leafyPuff", style = typography.screenTitle, color = colors.ink)
                    Text(
                        text = headerMeta(entries).uppercase(),
                        style = typography.metaLabel,
                        color = colors.ink3,
                    )
                }
                BunnyFace(mood = Mood.Happy, modifier = Modifier.size(44.dp))
            }
        }

        items(entries, key = { it.id }) { entry -> EntryCard(entry) }

        item {
            Text(
                text = "That's everything so far.",
                style = typography.body,
                color = colors.ink3,
                modifier = Modifier.padding(top = 8.dp),
            )
        }
    }
}

private fun headerMeta(entries: List<Entry>): String {
    val count = entries.size
    val noun = if (count == 1) "entry" else "entries"
    val month = entries.firstOrNull()?.let { formatMonthYear(it.date) }.orEmpty()
    return listOf("$count $noun", month).filter { it.isNotBlank() }.joinToString(" · ")
}
