package com.leafypuff.ui.mood

import androidx.compose.runtime.Composable
import androidx.compose.ui.tooling.preview.Preview
import com.leafypuff.theme.LeafyTheme
import kotlinx.datetime.LocalDate

private const val FrameWidth = 375
private const val FrameHeight = 812
private val PreviewDate = LocalDate(2026, 9, 1)

@Preview(name = "Mood picker on light", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun MoodPickerLightPreview() {
    MoodPickerFrame(dark = false)
}

@Preview(name = "Mood picker on dark", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun MoodPickerDarkPreview() {
    MoodPickerFrame(dark = true)
}

@Composable
private fun MoodPickerFrame(dark: Boolean) {
    LeafyTheme(darkOverride = dark) {
        MoodPicker(
            entryDate = PreviewDate,
            onPick = { },
            onClose = { },
            onDateClick = { },
        )
    }
}
