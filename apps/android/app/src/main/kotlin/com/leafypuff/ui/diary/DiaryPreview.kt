package com.leafypuff.ui.diary

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.leafypuff.data.SampleEntries
import com.leafypuff.theme.LeafyTheme
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.ui.photo.bandedPhoto

private const val FrameWidth = 375
private const val CardFrameHeight = 420

@Preview(name = "Cover card on light", widthDp = FrameWidth, heightDp = CardFrameHeight)
@Composable
private fun CoverCardLightPreview() {
    CardFrame(dark = false)
}

@Preview(name = "Cover card on dark", widthDp = FrameWidth, heightDp = CardFrameHeight)
@Composable
private fun CoverCardDarkPreview() {
    CardFrame(dark = true)
}

@Composable
private fun CardFrame(dark: Boolean) {
    LeafyTheme(darkOverride = dark) {
        Box(
            modifier = Modifier
                .fillMaxSize()
                .background(LocalLeafyColors.current.bg)
                .padding(24.dp),
        ) {
            EntryCard(
                entry = SampleEntries.first(),
                cover = bandedPhoto(300, 200, 84, Color(0xFFBFCE94), Color(0xFF6F7C48)),
            )
        }
    }
}
