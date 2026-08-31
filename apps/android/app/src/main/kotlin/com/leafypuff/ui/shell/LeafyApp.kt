package com.leafypuff.ui.shell

import androidx.compose.foundation.background
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import com.leafypuff.data.AppPreferences
import com.leafypuff.data.EntryStore
import com.leafypuff.domain.Entry
import com.leafypuff.theme.LeafyTheme
import com.leafypuff.theme.LeafyTypeScale
import com.leafypuff.theme.LeafyTypeScaleLarge
import com.leafypuff.theme.LeafyTypeScaleMedium
import com.leafypuff.theme.LeafyTypeScaleSmall
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.ui.editor.EntryComposer
import com.leafypuff.ui.photo.PhotoImporter
import com.leafypuff.ui.settings.TextSize
import kotlinx.coroutines.launch
import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.todayIn

@Composable
fun LeafyApp(
    importer: PhotoImporter,
    databasePath: String,
    versionName: String,
) {
    val scope = rememberCoroutineScope()
    val today = remember { Clock.System.todayIn(TimeZone.currentSystemDefault()) }
    val systemDark = isSystemInDarkTheme()

    var store by remember { mutableStateOf<EntryStore?>(null) }
    var entries by remember { mutableStateOf(emptyList<Entry>()) }
    var current by remember { mutableStateOf(Destination.Diary) }
    var composing by remember { mutableStateOf(false) }
    var selected by remember { mutableStateOf(today) }
    var visibleMonth by remember { mutableStateOf(today) }
    var preferences by remember { mutableStateOf(AppPreferences(darkMode = systemDark)) }

    LaunchedEffect(databasePath) {
        val opened = EntryStore.open(databasePath)
        store = opened
        entries = opened.list()
    }

    LeafyTheme(
        darkOverride = preferences.darkMode,
        typeScale = typeScaleFor(preferences.textSize),
    ) {
        val colors = LocalLeafyColors.current

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
                DestinationHost(
                    destination = current,
                    entries = entries,
                    today = today,
                    selected = selected,
                    visibleMonth = visibleMonth,
                    preferences = preferences,
                    versionName = versionName,
                    onSelectDay = { selected = it },
                    onMonthChange = { visibleMonth = it },
                    onToday = {
                        selected = today
                        visibleMonth = today
                    },
                    onCompose = { composing = true },
                    onPreferencesChange = { preferences = it },
                    onDeleteAll = {
                        scope.launch {
                            store?.deleteAll()
                            entries = store?.list().orEmpty()
                        }
                    },
                )

                BottomNav(
                    current = current,
                    onSelect = { current = it },
                    onCompose = { composing = true },
                    modifier = Modifier.align(Alignment.BottomCenter),
                )
            }

            EntryComposer(
                open = composing,
                today = selected,
                importer = importer,
                onClose = { composing = false },
                onSave = { draft, photoIds ->
                    scope.launch {
                        store?.save(draft, photoIds)
                        entries = store?.list().orEmpty()
                        composing = false
                        selected = draft.date
                    }
                },
                modifier = Modifier.fillMaxSize(),
            )
        }
    }
}

private fun typeScaleFor(size: TextSize): LeafyTypeScale = when (size) {
    TextSize.Small -> LeafyTypeScaleSmall
    TextSize.Medium -> LeafyTypeScaleMedium
    TextSize.Large -> LeafyTypeScaleLarge
}
