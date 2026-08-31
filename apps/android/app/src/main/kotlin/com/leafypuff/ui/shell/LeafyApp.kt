package com.leafypuff.ui.shell

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
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
import com.leafypuff.ui.editor.EntryComposer
import com.leafypuff.ui.photo.PhotoImporter
import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.todayIn

@Composable
fun LeafyApp(importer: PhotoImporter) {
    val colors = LocalLeafyColors.current
    var current by remember { mutableStateOf(Destination.Diary) }
    var composing by remember { mutableStateOf(false) }
    val today = remember { Clock.System.todayIn(TimeZone.currentSystemDefault()) }

    Box(
        modifier = Modifier
            .fillMaxSize()
            .background(colors.bg),
    ) {
        Box(
            modifier = Modifier
                .fillMaxSize()
                .statusBarsPadding(),
        ) {
            when (current) {
                Destination.Diary -> DiaryScreen(SampleEntries)
                else -> PlaceholderScreen(current)
            }

            BottomNav(
                current = current,
                onSelect = { current = it },
                onCompose = { composing = true },
                modifier = Modifier.align(Alignment.BottomCenter),
            )
        }

        EntryComposer(
            open = composing,
            today = today,
            importer = importer,
            onClose = { composing = false },
            modifier = Modifier.fillMaxSize(),
        )
    }
}
