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
import com.leafypuff.data.SyncOutcome
import com.leafypuff.data.SyncRunner
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

private const val UnsavedEmpty = "An entry needs a title or a few words."
private const val UnsavedUnknown = "That entry would not save. Try again."

private fun unsaved(failure: Throwable): String = when {
    failure.message?.contains("title or a body") == true -> UnsavedEmpty
    else -> UnsavedUnknown
}

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
    val syncing = remember(store) { store?.let { SyncRunner(it, session, apiBaseUrl) } }

    val forgetSession = {
        scope.launch { vault?.signOut() }
        session.clear()
        credentials = null
        signedIn = false
        entries = emptyList()
    }

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
            onPasswordReset = { forgetSession() },
        ) {
            VaultGate(
                access = vault,
                signedIn = credentials,
                apiBaseUrl = apiBaseUrl,
                onSignedOut = { forgetSession() },
                onOpened = {
                    scope.launch {
                        entries = store?.list().orEmpty()
                        if (syncing?.run() == SyncOutcome.SignedOut) {
                            forgetSession()
                        }
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
                        onSync = {
                            when (syncing?.run()) {
                                SyncOutcome.Moved -> true
                                SyncOutcome.SignedOut -> {
                                    forgetSession()
                                    false
                                }

                                else -> false
                            }
                        },
                        onSave = { draft, photoIds, onDone, onProblem ->
                            scope.launch {
                                runCatching { store?.save(draft, photoIds) }.fold(
                                    onSuccess = {
                                        entries = store?.list().orEmpty()
                                        onDone()
                                    },
                                    onFailure = { failure -> onProblem(unsaved(failure)) },
                                )
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
