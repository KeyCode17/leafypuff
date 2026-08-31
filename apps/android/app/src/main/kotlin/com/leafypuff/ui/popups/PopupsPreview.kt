package com.leafypuff.ui.popups

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.leafypuff.theme.LeafyTheme
import com.leafypuff.theme.LocalLeafyColors
import kotlinx.datetime.LocalDate

private const val FrameWidth = 375
private const val FrameHeight = 812
private val PreviewDate = LocalDate(2026, 9, 1)

@Preview(name = "Weather popup on light", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun WeatherPopupPreview() {
    PopupFrame(dark = false) {
        var selected by remember { mutableStateOf<String?>("Cloudy") }
        OptionPopup(
            title = WeatherTitle,
            options = WeatherOptions,
            selected = selected,
            onSelect = { selected = it },
            onDismiss = { },
        )
    }
}

@Preview(name = "Location popup on dark", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun LocationPopupPreview() {
    PopupFrame(dark = true) {
        var selected by remember { mutableStateOf<String?>("Cafe") }
        OptionPopup(
            title = LocationTitle,
            options = LocationOptions,
            selected = selected,
            onSelect = { selected = it },
            onDismiss = { },
        )
    }
}

@Preview(name = "Date popup on light", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun DatePopupPreview() {
    PopupFrame(dark = false) {
        var selected by remember { mutableStateOf(PreviewDate) }
        DatePopup(selected = selected, onSelect = { selected = it }, onDismiss = { })
    }
}

@Composable
private fun PopupFrame(dark: Boolean, content: @Composable () -> Unit) {
    LeafyTheme(darkOverride = dark) {
        Box(modifier = Modifier.fillMaxSize().background(LocalLeafyColors.current.bg)) {
            content()
        }
    }
}
