package com.leafypuff.ui.shell

import android.content.Context
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts.RequestPermission
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.platform.LocalContext
import com.leafypuff.data.EntryStore
import com.leafypuff.data.PreferenceStore
import com.leafypuff.notify.NotificationPermission
import com.leafypuff.notify.ReminderScheduler
import com.leafypuff.notify.needsNotificationConsent
import com.leafypuff.domain.Entry
import com.leafypuff.theme.LeafyTheme
import com.leafypuff.theme.LeafyTypeScale
import com.leafypuff.theme.LeafyTypeScaleLarge
import com.leafypuff.theme.LeafyTypeScaleMedium
import com.leafypuff.theme.LeafyTypeScaleSmall
import com.leafypuff.ui.auth.AuthGate
import com.leafypuff.ui.editor.OpenedEntry
import com.leafypuff.ui.lock.LockGate
import com.leafypuff.ui.photo.CorePhotoLibrary
import com.leafypuff.ui.photo.EntryPhoto
import com.leafypuff.ui.photo.NoPhotoLibrary
import com.leafypuff.ui.settings.TextSize
import kotlinx.coroutines.launch
import java.io.File
import kotlinx.datetime.Clock
import kotlinx.datetime.LocalDate
import kotlinx.datetime.TimeZone
import kotlinx.datetime.todayIn

@Composable
fun LeafyApp(
    databasePath: String,
    passphrase: String,
    versionName: String,
    apiBaseUrl: String,
) {
    val context = LocalContext.current
    val scope = rememberCoroutineScope()
    val today = remember { Clock.System.todayIn(TimeZone.currentSystemDefault()) }
    val systemDark = isSystemInDarkTheme()
    val settings = remember(context) { PreferenceStore(context) }

    var store by remember { mutableStateOf<EntryStore?>(null) }
    var entries by remember { mutableStateOf(emptyList<Entry>()) }
    var preferences by remember { mutableStateOf(settings.load(systemDark)) }
    // Read once. The design words the setting "Ask when opening PawNotes", so a toggle flipped
    // mid-session takes effect the next time the app opens, not under the owner's hands.
    val askForPin = remember(settings) { settings.lockEnabled() }

    LaunchedEffect(databasePath) {
        val opened = EntryStore.open(databasePath, passphrase)
        store = opened
        entries = opened.list()
    }

    val reminders = remember(context) { ReminderScheduler(context) }
    val askToNotify = rememberLauncherForActivityResult(RequestPermission()) { }

    val library = remember(store) {
        store?.let { CorePhotoLibrary(it.client) } ?: NoPhotoLibrary
    }

    LeafyTheme(
        darkOverride = preferences.darkMode,
        typeScale = typeScaleFor(preferences.textSize),
    ) {
        AuthGate(
            apiBaseUrl = apiBaseUrl,
            client = store?.client,
            onSignedIn = { name ->
                if (name.isNotBlank()) {
                    val named = preferences.copy(name = name)
                    preferences = named
                    settings.save(named)
                }
            },
        ) {
            LockGate(enabled = askForPin) {
                LeafyHome(
                    library = library,
                    today = today,
                    entries = entries,
                    preferences = preferences,
                    versionName = versionName,
                    onPreferencesChange = {
                        preferences = it
                        settings.save(it)
                        reminders.apply(it.reminderEnabled, it.reminderTime)
                        if (it.reminderEnabled && needsNotificationConsent(context)) {
                            askToNotify.launch(NotificationPermission)
                        }
                    },
                    onOpenEntry = { entry ->
                        val stored = store?.openForEdit(entry.id)
                        stored?.let {
                            OpenedEntry(
                                draft = it.draft,
                                photos = it.photoIds.mapNotNull { photoId ->
                                    library.thumbnail(photoId)
                                        ?.let { cover -> EntryPhoto(photoId, cover, null) }
                                },
                            )
                        }
                    },
                    onStatistics = { picked -> store?.statistics(picked, today) },
                    onExport = { store?.export(exportPath(context, today)) },
                    onSave = { draft, photoIds, onDone ->
                        scope.launch {
                            store?.save(draft, photoIds)
                            entries = store?.list().orEmpty()
                            onDone()
                        }
                    },
                    onDeleteAll = {
                        scope.launch {
                            store?.deleteAll()
                            entries = store?.list().orEmpty()
                        }
                    },
                )
            }
        }
    }
}

/**
 * The archive lands in the app's external files directory, which needs no permission and is what
 * a file manager shows. Stamping the date keeps a second export from overwriting the first.
 */
private fun exportPath(context: Context, today: LocalDate): String {
    val directory = context.getExternalFilesDir(null) ?: context.filesDir
    return File(directory, "leafypuff-$today.zip").absolutePath
}

private fun typeScaleFor(size: TextSize): LeafyTypeScale = when (size) {
    TextSize.Small -> LeafyTypeScaleSmall
    TextSize.Medium -> LeafyTypeScaleMedium
    TextSize.Large -> LeafyTypeScaleLarge
}
