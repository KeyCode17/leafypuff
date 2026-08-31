package com.leafypuff.ui.common

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.leafypuff.theme.LeafyTheme
import com.leafypuff.theme.LocalLeafyColors
import kotlinx.datetime.LocalDate

private const val FrameWidth = 375
private const val FrameHeight = 812
private val PromptDay = LocalDate(2026, 7, 4)

@Preview(name = "Exif prompt on light", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun ExifPromptLightPreview() {
    ToastFrame(dark = false, request = exifPromptToast(PromptDay))
}

@Preview(name = "Exif prompt on dark", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun ExifPromptDarkPreview() {
    ToastFrame(dark = true, request = exifPromptToast(PromptDay))
}

@Preview(name = "Plain toast on light", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun PlainToastLightPreview() {
    ToastFrame(dark = false, request = plainToast("Entry saved."))
}

@Composable
private fun ToastFrame(dark: Boolean, request: ToastRequest) {
    LeafyTheme(darkOverride = dark) {
        Box(
            modifier = Modifier
                .fillMaxSize()
                .background(LocalLeafyColors.current.bg),
        ) {
            ToastOverlay(
                visible = true,
                request = request,
                onAccept = { },
                onDismiss = { },
            )
        }
    }
}
