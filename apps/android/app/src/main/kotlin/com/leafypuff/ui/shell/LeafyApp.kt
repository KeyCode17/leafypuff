package com.leafypuff.ui.shell

import android.content.Context
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts.RequestPermission
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.platform.LocalContext
import com.leafypuff.data.DeviceKey
import com.leafypuff.data.EntryStore
import com.leafypuff.data.PinLock
import com.leafypuff.data.PreferenceStore
import com.leafypuff.data.SessionStore
import com.leafypuff.data.VaultAccess
import com.leafypuff.domain.Entry
import com.leafypuff.notify.NotificationPermission
import com.leafypuff.notify.ReminderScheduler
import com.leafypuff.notify.needsNotificationConsent
import com.leafypuff.theme.LeafyTheme
import com.leafypuff.theme.LeafyTypeScale
import com.leafypuff.theme.LeafyTypeScaleLarge
import com.leafypuff.theme.LeafyTypeScaleMedium
import com.leafypuff.theme.LeafyTypeScaleSmall
import com.leafypuff.ui.auth.AuthGate
import com.leafypuff.ui.auth.SignedIn
import com.leafypuff.ui.editor.OpenedEntry
import com.leafypuff.ui.lock.LockGate
import com.leafypuff.ui.photo.CorePhotoLibrary
import com.leafypuff.ui.photo.EntryPhoto
import com.leafypuff.ui.photo.NoPhotoLibrary
import com.leafypuff.ui.settings.TextSize
import com.leafypuff.ui.vault.VaultGate
import kotlinx.coroutines.launch
import kotlinx.datetime.Clock
import kotlinx.datetime.LocalDate
import kotlinx.datetime.TimeZone
import kotlinx.datetime.todayIn
import java.io.File

@Composable
fun LeafyApp(databasePath: String, versionName: String, apiBaseUrl: String) {
    val context = LocalContext.current
    val scope = rememberCoroutineScope()
    val today = remember { Clock.System.todayIn(TimeZone.currentSystemDefault()) }
    val settings = remember(context) { PreferenceStore(context) }
    val session = remember(context) { SessionStore(context) }
    val reminders = remember(context) { ReminderScheduler(context) }
    val deviceKey = remember(context) { DeviceKey(context) }
    val pin = remember(context) { PinLock(context) }
    val askToNotify = rememberLauncherForActivityResult(RequestPermission()) { }

    var store by remember { mutableStateOf<EntryStore?>(null) }
    var entries by remember { mutableStateOf(emptyList<Entry>()) }
    var preferences by remember { mutableStateOf(settings.load()) }
    var signedIn by remember { mutableStateOf(session.signedIn()) }
    var credentials by remember { mutableStateOf<SignedIn?>(null) }
    val askForPin = remember(settings) { settings.lockEnabled() }

    LaunchedEffect(databasePath) {
        store = EntryStore.open(databasePath)
    }

    val library = remember(store) {
        store?.let { CorePhotoLibrary(it.client) } ?: NoPhotoLibrary
    }
    val vault = remember(store) { store?.let { VaultAccess(it.client, deviceKey) } }

    LeafyTheme(
        darkOverride = preferences.darkMode,
        typeScale = typeScaleFor(preferences.textSize),
    ) {
        AuthGate(
            apiBaseUrl = apiBaseUrl,
            client = store?.client,
            signedIn = signedIn,
            onSignedIn = {
                credentials = it
                signedIn = true
                if (it.name.isNotBlank()) {
                    val named = preferences.copy(name = it.name)
                    preferences = named
                    settings.save(named)
                }
            },
        ) {
            VaultGate(
                access = vault,
                signedIn = credentials,
                apiBaseUrl = apiBaseUrl,
                onSignedOut = {
                    session.clear()
                    credentials = null
                    signedIn = false
                    entries = emptyList()
                },
                onOpened = {
                    scope.launch {
                        entries = store?.list().orEmpty()
                        store?.sync(apiBaseUrl, session.accessToken())
                        entries = store?.list().orEmpty()
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
                            if (preferences.lockEnabled && !it.lockEnabled) {
                                pin.clear()
                            }
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
                        onSync = { store?.sync(apiBaseUrl, session.accessToken()) != null },
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
}

private fun exportPath(context: Context, today: LocalDate): String {
    val directory = context.getExternalFilesDir(null) ?: context.filesDir
    return File(directory, "leafypuff-$today.zip").absolutePath
}

private fun typeScaleFor(size: TextSize): LeafyTypeScale = when (size) {
    TextSize.Small -> LeafyTypeScaleSmall
    TextSize.Medium -> LeafyTypeScaleMedium
    TextSize.Large -> LeafyTypeScaleLarge
}
