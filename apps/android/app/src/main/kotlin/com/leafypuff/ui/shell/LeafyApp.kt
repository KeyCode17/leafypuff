package com.leafypuff.ui.shell

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import com.leafypuff.data.SampleEntries
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.ui.diary.DiaryScreen

@Composable
fun LeafyApp() {
    val colors = LocalLeafyColors.current
    var current by remember { mutableStateOf(Destination.Diary) }

    Box(
        modifier = Modifier
            .fillMaxSize()
            .background(colors.bg)
            .statusBarsPadding(),
    ) {
        when (current) {
            Destination.Diary -> DiaryScreen(SampleEntries)
            else -> PlaceholderScreen(current)
        }

        BottomNav(
            current = current,
            onSelect = { current = it },
            onCompose = { },
            modifier = Modifier.align(Alignment.BottomCenter),
        )
    }
}
