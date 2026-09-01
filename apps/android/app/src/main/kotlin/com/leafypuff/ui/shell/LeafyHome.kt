package com.leafypuff.ui.shell

import androidx.compose.foundation.background
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
import androidx.compose.ui.graphics.ImageBitmap
import com.leafypuff.data.AppPreferences
import com.leafypuff.domain.Entry
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.ui.common.ToastOverlay
import com.leafypuff.ui.common.ToastRequest
import com.leafypuff.ui.common.plainToast
import com.leafypuff.ui.common.saveToast
import com.leafypuff.ui.editor.EntryComposer
import com.leafypuff.ui.editor.EntryDraft
import com.leafypuff.ui.editor.OpenedEntry
import com.leafypuff.ui.photo.EntryPhoto
import com.leafypuff.ui.photo.PhotoLibrary
import com.leafypuff.ui.stats.StatRange
import com.leafypuff.ui.stats.StatsSummary
import kotlinx.coroutines.launch
import kotlinx.datetime.LocalDate

/**
 * The signed-in, unlocked app: the four destinations, the nav bar and the editor that floats over
 * them. Everything here assumes an account and an open vault, which is what the two gates in front
 * of it guarantee.
 */
@Composable
fun LeafyHome(
    library: PhotoLibrary,
    today: LocalDate,
    entries: List<Entry>,
    preferences: AppPreferences,
    versionName: String,
    onPreferencesChange: (AppPreferences) -> Unit,
    onOpenEntry: suspend (Entry) -> OpenedEntry?,
    onStatistics: suspend (StatRange) -> StatsSummary?,
    onExport: suspend () -> String?,
    onSave: (EntryDraft, List<String>, () -> Unit) -> Unit,
    onDeleteAll: () -> Unit,
) {
    val colors = LocalLeafyColors.current
    val scope = rememberCoroutineScope()

    var current by remember { mutableStateOf(Destination.Diary) }
    var composing by remember { mutableStateOf(false) }
    var editing by remember { mutableStateOf<EntryDraft?>(null) }
    var editingPhotos by remember { mutableStateOf(emptyList<EntryPhoto>()) }
    var opening by remember { mutableStateOf<Entry?>(null) }
    var covers by remember { mutableStateOf(emptyMap<String, ImageBitmap>()) }
    var selected by remember { mutableStateOf(today) }
    var visibleMonth by remember { mutableStateOf(today) }
    var range by remember { mutableStateOf(StatRange.SevenDays) }
    var statistics by remember { mutableStateOf(StatsSummary()) }
    var toast by remember { mutableStateOf<ToastRequest?>(null) }

    LaunchedEffect(entries) {
        covers = loadCovers(library, entries, covers)
    }

    LaunchedEffect(entries, range) {
        statistics = onStatistics(range) ?: statistics
    }

    LaunchedEffect(opening) {
        val entry = opening ?: return@LaunchedEffect
        val loaded = onOpenEntry(entry)
        opening = null
        if (loaded != null) {
            editing = loaded.draft
            editingPhotos = loaded.photos
            composing = true
        }
    }

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
                covers = covers,
                statistics = statistics,
                range = range,
                preferences = preferences,
                versionName = versionName,
                onSelectDay = { selected = it },
                onMonthChange = { visibleMonth = it },
                onToday = {
                    selected = today
                    visibleMonth = today
                },
                onCompose = {
                    editing = null
                    editingPhotos = emptyList()
                    composing = true
                },
                onOpenEntry = { opening = it },
                onRangeChange = { range = it },
                onExport = {
                    scope.launch {
                        val written = onExport()
                        toast = plainToast(exportMessage(written))
                    }
                },
                onPreferencesChange = onPreferencesChange,
                onDeleteAll = onDeleteAll,
            )

            BottomNav(
                current = current,
                onSelect = { current = it },
                onCompose = {
                    editing = null
                    editingPhotos = emptyList()
                    composing = true
                },
                modifier = Modifier.align(Alignment.BottomCenter),
            )
        }

        ToastOverlay(
            visible = toast != null,
            request = toast,
            onAccept = { toast = null },
            onDismiss = { toast = null },
        )

        EntryComposer(
            open = composing,
            today = selected,
            library = library,
            existing = editing,
            existingPhotos = editingPhotos,
            stickerPack = preferences.stickerPack,
            onClose = { composing = false },
            onSave = { draft, photoIds ->
                val reopened = draft.id != null
                onSave(draft, photoIds) {
                    composing = false
                    selected = draft.date
                    // The design ends every save on the Diary at that date, whichever tab the
                    // editor was opened from.
                    current = Destination.Diary
                    toast = saveToast(reopened)
                }
            },
            modifier = Modifier.fillMaxSize(),
        )
    }
}

/**
 * Only covers not already held are fetched. Redrawing the list after a save would otherwise decode
 * every thumbnail again for the one entry that changed.
 */
private suspend fun loadCovers(
    library: PhotoLibrary,
    entries: List<Entry>,
    held: Map<String, ImageBitmap>,
): Map<String, ImageBitmap> {
    val wanted = entries.mapNotNull { entry -> entry.coverPhotoId?.let { entry.id to it } }
    val fetched = wanted
        .filterNot { held.containsKey(it.first) }
        .mapNotNull { (entryId, photoId) ->
            library.thumbnail(photoId)?.let { entryId to it }
        }
    return held.filterKeys { key -> wanted.any { it.first == key } } + fetched
}

private fun exportMessage(path: String?): String = when (path) {
    null -> "The export could not be written."
    else -> "Diary exported to ${path.substringAfterLast('/')}."
}
