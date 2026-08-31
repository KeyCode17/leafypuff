package com.leafypuff.ui.diary

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import com.leafypuff.domain.Entry
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import com.leafypuff.ui.common.BunnyFace
import com.leafypuff.ui.common.formatEntryDate

@Composable
fun EntryCard(entry: Entry, modifier: Modifier = Modifier) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Column(
        modifier = modifier
            .fillMaxWidth()
            .clip(LeafyShapes.card)
            .background(colors.sheet)
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Text(
                text = formatEntryDate(entry.date).uppercase(),
                style = typography.metaLabel,
                color = colors.ink3,
            )
            MoodChip(entry)
        }

        Text(text = entry.title, style = typography.cardTitle, color = colors.ink)

        Text(
            text = entry.body,
            style = typography.body,
            color = colors.ink2,
            maxLines = 2,
            overflow = TextOverflow.Ellipsis,
        )

        Row(horizontalArrangement = Arrangement.spacedBy(6.dp)) {
            entry.tags.forEach { tag -> TagChip(tag) }
        }
    }
}

@Composable
private fun MoodChip(entry: Entry) {
    val colors = LocalLeafyColors.current

    Row(
        modifier = Modifier
            .clip(LeafyShapes.chip)
            .background(colors.soft2)
            .padding(horizontal = 10.dp, vertical = 4.dp),
        horizontalArrangement = Arrangement.spacedBy(6.dp),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        BunnyFace(mood = entry.mood, modifier = Modifier.size(22.dp))
        Text(
            text = entry.mood.label,
            style = LocalLeafyTypography.current.chipLabel,
            color = colors.ink2,
        )
    }
}

@Composable
private fun TagChip(tag: String) {
    val colors = LocalLeafyColors.current

    Text(
        text = tag,
        style = LocalLeafyTypography.current.chipLabel,
        color = colors.accentDeep,
        modifier = Modifier
            .clip(LeafyShapes.chip)
            .background(colors.soft)
            .padding(horizontal = 10.dp, vertical = 4.dp),
    )
}
