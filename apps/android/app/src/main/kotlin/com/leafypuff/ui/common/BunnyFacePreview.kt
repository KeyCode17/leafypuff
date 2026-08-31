package com.leafypuff.ui.common

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.leafypuff.domain.Mood

private const val ColumnsPerRow = 3
private val CellWidth = 84.dp
private val PreviewFaceSize = 56.dp

@Preview(name = "Moods on light")
@Composable
private fun BunnyFaceLightPreview() {
    MoodGrid(background = Color(0xFFF6F8EC), labelColor = Color(0xFF242D35))
}

@Preview(name = "Moods on dark")
@Composable
private fun BunnyFaceDarkPreview() {
    MoodGrid(background = Color(0xFF191D14), labelColor = Color(0xFFF1F4E6))
}

@Composable
private fun MoodGrid(background: Color, labelColor: Color) {
    Column(
        modifier = Modifier
            .background(background)
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        Mood.entries.chunked(ColumnsPerRow).forEach { row ->
            Row(horizontalArrangement = Arrangement.spacedBy(8.dp)) {
                row.forEach { MoodCell(it, labelColor) }
            }
        }
    }
}

@Composable
private fun MoodCell(mood: Mood, labelColor: Color) {
    Column(
        modifier = Modifier.width(CellWidth),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.spacedBy(4.dp),
    ) {
        BunnyFace(mood, Modifier.size(PreviewFaceSize))
        Text(
            text = mood.label,
            color = labelColor,
            fontSize = 11.sp,
            textAlign = TextAlign.Center,
        )
    }
}
