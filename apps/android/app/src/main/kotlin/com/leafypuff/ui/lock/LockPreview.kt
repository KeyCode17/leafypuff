package com.leafypuff.ui.lock

import androidx.compose.runtime.Composable
import androidx.compose.ui.tooling.preview.Preview
import com.leafypuff.theme.LeafyTheme

private const val FrameWidth = 375
private const val FrameHeight = 812
private const val PartialPin = 2

@Preview(name = "Lock empty on light", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun LockEmptyLightPreview() {
    LockFrame(dark = false, pinLength = 0)
}

@Preview(name = "Lock empty on dark", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun LockEmptyDarkPreview() {
    LockFrame(dark = true, pinLength = 0)
}

@Preview(name = "Lock partly entered on light", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun LockPartialLightPreview() {
    LockFrame(dark = false, pinLength = PartialPin)
}

@Preview(name = "Lock partly entered on dark", widthDp = FrameWidth, heightDp = FrameHeight)
@Composable
private fun LockPartialDarkPreview() {
    LockFrame(dark = true, pinLength = PartialPin)
}

@Composable
private fun LockFrame(dark: Boolean, pinLength: Int) {
    LeafyTheme(darkOverride = dark) {
        LockScreen(
            pinLength = pinLength,
            hint = lockHint(LockStep.Verify, pinLength, wrong = false),
            onDigit = { },
            onBackspace = { },
            onBiometric = { },
        )
    }
}
