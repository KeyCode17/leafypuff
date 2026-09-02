package com.leafypuff.ui.shell

import android.content.Context
import androidx.activity.compose.BackHandler
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts.RequestPermission
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.getValue
import androidx.compose.foundation.clickable
import androidx.compose.foundation.interaction.MutableInteractionSource
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.platform.LocalContext
import android.graphics.BitmapFactory
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.asImageBitmap
import com.leafypuff.core.CoreClient
import com.leafypuff.core.LeafyPuffCoreException
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
import com.leafypuff.ui.lock.PinSetup
import com.leafypuff.ui.lock.PinSetupMode
import com.leafypuff.ui.photo.CorePhotoLibrary
import com.leafypuff.ui.photo.rememberPhotoPicker
import com.leafypuff.ui.profile.ProfileScreen
import com.leafypuff.ui.profile.ProfileState
import com.leafypuff.ui.crop.CropScreen
import com.leafypuff.ui.crop.PhotoFraming
import com.leafypuff.ui.profile.ProfileStep
import com.leafypuff.ui.photo.EntryPhoto
import com.leafypuff.ui.photo.PhotoCover
import com.leafypuff.ui.photo.PhotoPlacement
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
    var pinSetup by remember { mutableStateOf<PinSetupMode?>(null) }
    var pinAccepted by remember { mutableStateOf(false) }
    var profile by remember { mutableStateOf<ProfileState?>(null) }
    var framingPhoto by remember { mutableStateOf<String?>(null) }
    var framingImage by remember { mutableStateOf<ImageBitmap?>(null) }
    var framing by remember { mutableStateOf(PhotoFraming()) }
    var framingPending by remember { mutableStateOf(false) }
    var coversEpoch by remember { mutableIntStateOf(0) }
    var framings by remember { mutableStateOf(mapOf<String, PhotoFraming>()) }
    var refreshedCover by remember { mutableStateOf<PhotoCover?>(null) }
    var vaultEpoch by remember { mutableIntStateOf(0) }

    LaunchedEffect(databasePath) {
        store = EntryStore.open(databasePath)
    }

    var avatar by remember { mutableStateOf<ImageBitmap?>(null) }

    val library = remember(store) {
        store?.let { CorePhotoLibrary(it.client) } ?: NoPhotoLibrary
    }

    var avatarFraming by remember { mutableStateOf<String?>(null) }

    val pickAvatar = rememberPhotoPicker { bytes ->
        scope.launch {
            val imported = library.import(bytes) ?: return@launch
            val worn = preferences.copy(avatarPhotoId = imported.id)
            preferences = worn
            settings.save(worn)
            avatar = imported.cover
            framing = PhotoFraming()
            framingImage = null
            avatarFraming = imported.id
        }
    }
    val vault = remember(store) { store?.let { VaultAccess(it.client, deviceKey) } }
    val syncing = remember(store) { store?.let { SyncRunner(it, session, apiBaseUrl) } }


    val listed: suspend () -> List<Entry> = {
        runCatching { store?.list().orEmpty() }.getOrDefault(emptyList())
    }

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
                        entries = listed()
                        vaultEpoch += 1
                        if (syncing?.run() == SyncOutcome.SignedOut) {
                            forgetSession()
                            return@launch
                        }
                        entries = listed()
                        vaultEpoch += 1
                    }
                },
            ) {
                LockGate(
                    enabled = preferences.lockEnabled,
                    biometricEnabled = preferences.biometricEnabled,
                    unlocked = pinAccepted,
                    onUnlocked = { pinAccepted = true },
                ) {
                    LeafyHome(
                        library = library,
                        today = today,
                        entries = entries,
                        preferences = preferences,
                        versionName = versionName,
                        onToggleLock = { wanted ->
                            if (wanted) {
                                pinSetup = PinSetupMode.Create
                            } else {
                                pin.clear()
                                val without = preferences.copy(lockEnabled = false)
                                preferences = without
                                settings.save(without)
                            }
                        },
                        onToggleBiometric = { wanted ->
                            val marked = preferences.copy(biometricEnabled = wanted)
                            preferences = marked
                            settings.save(marked)
                        },
                        onChangePin = { pinSetup = PinSetupMode.Change },
                        avatar = avatar,
                        coversEpoch = coversEpoch,
                        refreshedCover = refreshedCover,
                        onFramePhoto = { id ->
                            framing = PhotoFraming()
                            framingImage = null
                            framingPhoto = id
                        },
                        onEditProfile = {
                            profile = ProfileState(
                                name = preferences.name,
                                email = session.email(),
                            )
                        },
                        onSignOut = {
                            pinAccepted = false
                            forgetSession()
                        },
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
                                        library.thumbnail(photoId)?.let { cover ->
                                            EntryPhoto(
                                                id = photoId,
                                                cover = cover,
                                                takenOn = null,
                                                place = runCatching {
                                                    store?.client?.photoPlacement(photoId)
                                                }
                                                    .getOrNull()
                                                    ?.takeIf { held -> held.size == 4 }
                                                    ?.let { held ->
                                                        PhotoPlacement(
                                                            x = held[0].toFloat(),
                                                            y = held[1].toFloat(),
                                                            size = held[2].toFloat(),
                                                            rotation = held[3].toFloat(),
                                                        )
                                                    },
                                            )
                                        }
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
                        onForgetPhotos = { dropped ->
                            scope.launch {
                                dropped.forEach { id ->
                                    runCatching {
                                        store?.forgetPhoto(apiBaseUrl, session.accessToken(), id)
                                    }
                                }
                            }
                        },
                        onSave = { draft, kept, onDone, onProblem ->
                            scope.launch {
                                val photoIds = kept.map { it.id }
                                runCatching { store?.save(draft, photoIds) }.fold(
                                    onSuccess = {
                                        kept.forEach { photo ->
                                            val held = framings[photo.id]
                                            if (held != null) {
                                                runCatching {
                                                    store?.client?.framePhoto(
                                                        photo.id,
                                                        held.x,
                                                        held.y,
                                                        held.width,
                                                    )
                                                }
                                            }
                                            val placed = photo.place ?: return@forEach
                                            runCatching {
                                                store?.client?.placePhoto(
                                                    photo.id,
                                                    placed.x.toDouble(),
                                                    placed.y.toDouble(),
                                                    placed.size.toDouble(),
                                                    placed.rotation.toDouble(),
                                                )
                                            }
                                        }
                                        framings = framings - photoIds.toSet()
                                        entries = listed()
                                        onDone()
                                    },
                                    onFailure = { failure -> onProblem(unsaved(failure)) },
                                )
                            }
                        },
                        onDeleteAll = {
                            scope.launch {
                                runCatching { store?.deleteAll() }
                                entries = listed()
                            }
                        },
                    )
                }
            }
        }

        LaunchedEffect(preferences.avatarPhotoId, store, vaultEpoch) {
            avatar = preferences.avatarPhotoId?.let { library.thumbnail(it) }
        }

        val sealedOff = Modifier.clickable(
            interactionSource = remember { MutableInteractionSource() },
            indication = null,
            onClick = { },
        )

        val framed = framingPhoto
        if (framed != null) {
            BackHandler { framingPhoto = null }
            LaunchedEffect(framed) {
                val held = framings[framed]
                    ?: runCatching { store?.client?.photoFraming(framed) }
                        .getOrNull()
                        ?.takeIf { it.size == 3 }
                        ?.let { PhotoFraming(it[0], it[1], it[2]) }
                if (held != null) {
                    framing = held
                }
                framingImage = runCatching { store?.client?.originalPhoto(framed) }
                    .getOrNull()
                    ?.let { bytes ->
                        BitmapFactory.decodeByteArray(bytes, 0, bytes.size)?.asImageBitmap()
                    }
            }
            CropScreen(
                photo = framingImage,
                framing = framing,
                pending = framingPending,
                onFramingChange = { framing = it },
                onSubmit = {
                    if (!framingPending) {
                        framingPending = true
                        scope.launch {
                            runCatching {
                                store?.client?.framePhoto(
                                    framed,
                                    framing.x,
                                    framing.y,
                                    framing.width,
                                )
                            }
                            framings = framings + (framed to framing)
                            library.thumbnail(framed)?.let { fresh ->
                                refreshedCover = PhotoCover(framed, fresh)
                            }
                            framingPending = false
                            framingPhoto = null
                            coversEpoch += 1
                        }
                    }
                },
                onBack = { framingPhoto = null },
                modifier = sealedOff,
            )
        }

        val editing = profile
        if (editing != null) {
            BackHandler {
                profile = when (editing.step) {
                    ProfileStep.ConfirmEmail -> editing.copy(step = ProfileStep.Details)
                    else -> null
                }
            }
            ProfileScreen(
                state = editing,
                avatar = avatar,
                onStateChange = { profile = it },
                onPickAvatar = pickAvatar,
                onClearAvatar = {
                    val held = preferences.avatarPhotoId
                    val bare = preferences.copy(avatarPhotoId = null)
                    preferences = bare
                    settings.save(bare)
                    avatar = null
                    if (held != null) {
                        scope.launch {
                            runCatching {
                                store?.forgetPhoto(apiBaseUrl, session.accessToken(), held)
                            }
                        }
                    }
                },
                onSubmit = {
                    if (!editing.pending) {
                        profile = editing.copy(pending = true, error = null)
                        scope.launch {
                            profile = submitProfile(
                                state = editing,
                                apiBaseUrl = apiBaseUrl,
                                session = session,
                                client = store?.client,
                                onNameChange = { chosen ->
                                    val named = preferences.copy(name = chosen)
                                    preferences = named
                                    settings.save(named)
                                },
                            )
                        }
                    }
                },
                onBack = { profile = null },
                modifier = sealedOff,
            )
        }

        val rounding = avatarFraming
        if (rounding != null) {
            BackHandler { avatarFraming = null }
            LaunchedEffect(rounding) {
                framingImage = runCatching { store?.client?.originalPhoto(rounding) }
                    .getOrNull()
                    ?.let { bytes ->
                        BitmapFactory.decodeByteArray(bytes, 0, bytes.size)?.asImageBitmap()
                    }
            }
            CropScreen(
                photo = framingImage,
                framing = framing,
                pending = framingPending,
                title = "Frame your photo",
                blurb = "Drag to move it, pinch to change how much it holds.",
                ratio = PhotoFraming.SquareTallness,
                round = true,
                onFramingChange = { framing = it },
                onSubmit = {
                    if (!framingPending) {
                        framingPending = true
                        scope.launch {
                            runCatching {
                                store?.client?.frameAvatar(
                                    rounding,
                                    framing.x,
                                    framing.y,
                                    framing.width,
                                )
                            }
                            avatar = library.thumbnail(rounding)
                            framingPending = false
                            avatarFraming = null
                        }
                    }
                },
                onBack = { avatarFraming = null },
                modifier = sealedOff,
            )
        }

        val setup = pinSetup
        if (setup != null) {
            BackHandler { pinSetup = null }
            PinSetup(
                mode = setup,
                onDone = {
                    if (setup == PinSetupMode.Create) {
                        val guarded = preferences.copy(lockEnabled = true)
                        preferences = guarded
                        settings.save(guarded)
                    }
                    pinAccepted = true
                    pinSetup = null
                },
                onCancel = { pinSetup = null },
                modifier = sealedOff,
            )
        }
    }
}

private suspend fun submitProfile(
    state: ProfileState,
    apiBaseUrl: String,
    session: SessionStore,
    client: CoreClient?,
    onNameChange: (String) -> Unit,
): ProfileState? {
    val token = session.accessToken()
    if (client == null || token == null) {
        return state.copy(pending = false, error = "Sign in again to change these.")
    }
    return when (state.step) {
        ProfileStep.Details -> {
            val chosen = state.name.trim()
            if (chosen.isNotBlank()) {
                onNameChange(chosen)
            }
            if (state.email.trim().equals(session.email(), ignoreCase = true)) {
                null
            } else {
                runCatching { client.changeEmail(apiBaseUrl, token, state.email.trim()) }.fold(
                    onSuccess = {
                        state.copy(pending = false, step = ProfileStep.ConfirmEmail, code = "")
                    },
                    onFailure = { failure ->
                        state.copy(pending = false, error = profileProblem(failure))
                    },
                )
            }
        }

        ProfileStep.ConfirmEmail ->
            runCatching { client.confirmEmail(apiBaseUrl, token, state.code) }.fold(
                onSuccess = { adopted ->
                    session.rename(adopted)
                    null
                },
                onFailure = { failure ->
                    state.copy(pending = false, error = profileProblem(failure))
                },
            )
    }
}

private fun profileProblem(failure: Throwable): String = when (failure) {
    is LeafyPuffCoreException.EmailTaken -> "That address already has an account."
    is LeafyPuffCoreException.InvalidCredentials -> "That code is wrong or has expired."
    is LeafyPuffCoreException.TooManyAttempts -> "Too many attempts. Wait a moment."
    is LeafyPuffCoreException.MailUnavailable -> "We could not send the code. Try again shortly."
    is LeafyPuffCoreException.Storage -> "No connection. Check your network."
    else -> "That did not work. Try again."
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
