package com.leafypuff.ui.shell

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.statusBarsPadding
import androidx.compose.foundation.pager.HorizontalPager
import androidx.compose.foundation.pager.rememberPagerState
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
import com.leafypuff.domain.Mood
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.ui.common.ToastOverlay
import com.leafypuff.ui.common.ToastRequest
import com.leafypuff.ui.popups.ConfirmPopup
import com.leafypuff.ui.common.deleteEntryToast
import com.leafypuff.ui.common.plainToast
import com.leafypuff.ui.common.saveToast
import com.leafypuff.ui.editor.EntryComposer
import com.leafypuff.ui.editor.EntryDraft
import com.leafypuff.ui.editor.OpenedEntry
import com.leafypuff.ui.photo.EntryPhoto
import com.leafypuff.ui.photo.PhotoCropped
import com.leafypuff.ui.photo.PhotoCover
import com.leafypuff.ui.photo.PhotoLibrary
import com.leafypuff.ui.stats.StatRange
import com.leafypuff.ui.stats.StatsSummary
import kotlinx.coroutines.launch
import kotlinx.datetime.LocalDate

private const val SignOutTitle = "Log out?"
private const val SignOutBody =
    "Your diary stays on this phone. Log in again whenever you want to keep writing."
private const val SignOutAccept = "Log out"
private const val SignOutReject = "Stay"

@Composable
fun LeafyHome(
    library: PhotoLibrary,
    today: LocalDate,
    entries: List<Entry>,
    preferences: AppPreferences,
    versionName: String,
    onPreferencesChange: (AppPreferences) -> Unit,
    onToggleLock: (Boolean) -> Unit,
    onToggleBiometric: (Boolean) -> Unit,
    onChangePin: () -> Unit,
    onSignOut: () -> Unit,
    onEditProfile: () -> Unit,
    onFramePhoto: (String) -> Unit,
    onCropPlaced: (String) -> Unit,
    refreshedCover: PhotoCover?,
    refreshedPlacement: PhotoCropped?,
    coversEpoch: Int,
    avatar: ImageBitmap?,
    onOpenEntry: suspend (Entry) -> OpenedEntry?,
    onStatistics: suspend (StatRange) -> StatsSummary?,
    onExport: suspend () -> String?,
    onSync: suspend () -> Boolean,
    onSave: (EntryDraft, List<EntryPhoto>, () -> Unit, (String) -> Unit) -> Unit,
    onDelete: (String, () -> Unit) -> Unit,
    onForgetPhotos: (List<String>) -> Unit,
    onDeleteAll: () -> Unit,
) {
    val colors = LocalLeafyColors.current
    val scope = rememberCoroutineScope()

    var current by remember { mutableStateOf(Destination.Diary) }
    val pager = rememberPagerState(pageCount = { Destination.entries.size })
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
    var pendingDelete by remember { mutableStateOf<String?>(null) }
    var confirmingSignOut by remember { mutableStateOf(false) }
    var lastSynced by remember { mutableStateOf("Never") }

    LaunchedEffect(current) {
        if (pager.targetPage != current.ordinal) {
            pager.animateScrollToPage(current.ordinal)
        }
    }

    LaunchedEffect(pager.settledPage) {
        current = Destination.entries[pager.settledPage]
    }

    LaunchedEffect(entries, coversEpoch) {
        covers = loadCovers(library, entries, emptyMap())
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
            HorizontalPager(
                state = pager,
                modifier = Modifier.fillMaxSize(),
                beyondViewportPageCount = 0,
            ) { page ->
                DestinationHost(
                    destination = Destination.entries[page],
                    entries = entries,
                    today = today,
                    selected = selected,
                    visibleMonth = visibleMonth,
                    covers = covers,
                    statistics = statistics,
                    range = range,
                    preferences = preferences,
                    versionName = versionName,
                    onToggleLock = onToggleLock,
                    onToggleBiometric = onToggleBiometric,
                    onChangePin = onChangePin,
                    onSignOut = { confirmingSignOut = true },
                    avatar = avatar,
                    onEditProfile = onEditProfile,
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
                    lastSynced = lastSynced,
                    onSync = {
                        scope.launch {
                            val moved = onSync()
                            lastSynced = syncLabel(moved)
                            toast = plainToast(syncMessage(moved))
                        }
                    },
                    onExport = {
                        scope.launch {
                            val written = onExport()
                            toast = plainToast(exportMessage(written))
                        }
                    },
                    onPreferencesChange = onPreferencesChange,
                    onDeleteAll = onDeleteAll,
                )
            }

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



        EntryComposer(
            open = composing,
            today = selected,
            library = library,
            existing = editing,
            existingPhotos = editingPhotos,
            stickerPack = preferences.stickerPack,
            onClose = { composing = false },
            onDelete = { id ->
                pendingDelete = id
                toast = deleteEntryToast()
            },
            onFramePhoto = onFramePhoto,
            onCropPlaced = onCropPlaced,
            refreshedCover = refreshedCover,
            refreshedPlacement = refreshedPlacement,
            onSave = { draft, kept ->
                val reopened = !draft.fresh
                val dropped = editingPhotos.map { it.id } - kept.map { it.id }.toSet()
                onSave(
                    draft,
                    kept,
                    {
                        onForgetPhotos(dropped)
                        composing = false
                        selected = draft.date
                        current = Destination.Diary
                        toast = saveToast(reopened)
                        scope.launch { if (onSync()) lastSynced = syncLabel(true) }
                    },
                    { problem -> toast = plainToast(problem) },
                )
            },
            modifier = Modifier.fillMaxSize(),
        )

        ToastOverlay(
            visible = toast != null,
            request = toast,
            onAccept = {
                toast = null
                val doomed = pendingDelete
                pendingDelete = null
                if (doomed != null) {
                    onDelete(doomed) {
                        composing = false
                        current = Destination.Diary
                        toast = plainToast("Entry deleted.")
                        scope.launch { if (onSync()) lastSynced = syncLabel(true) }
                    }
                }
            },
            onDismiss = { toast = null },
        )

        if (confirmingSignOut) {
            ConfirmPopup(
                face = Mood.Sad,
                title = SignOutTitle,
                body = SignOutBody,
                accept = SignOutAccept,
                reject = SignOutReject,
                onAccept = {
                    confirmingSignOut = false
                    onSignOut()
                },
                onDismiss = { confirmingSignOut = false },
            )
        }
    }
}

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

private fun syncLabel(moved: Boolean): String = when {
    moved -> "Just now"
    else -> "Not signed in"
}

private fun syncMessage(moved: Boolean): String = when {
    moved -> "Diary synced."
    else -> "Sign in to sync."
}
