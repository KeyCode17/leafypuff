package com.leafypuff.core

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.datetime.LocalDate

data class SyncSummary(val pushed: UInt, val pulled: UInt, val cursor: Long)

data class IssuedSession(
    val accessToken: String,
    val refreshToken: String,
    val expiresInSeconds: Long,
)

data class MailedChallenge(val expiresInSeconds: Long)

data class PhotoDraft(val id: String, val path: String, val ordinal: Int)

data class ImportedPhoto(val id: String, val path: String, val takenOn: LocalDate?)

data class StickerDraft(
    val key: String,
    val sticker: Sticker,
    val x: Float,
    val y: Float,
    val size: Float,
    val rotation: Float,
)

data class EntryDraft(
    val id: String,
    val date: LocalDate,
    val mood: Mood,
    val title: String,
    val body: String,
    val tags: List<String>,
    val weather: Weather?,
    val location: Location?,
    val photos: List<PhotoDraft>,
    val stickers: List<StickerDraft>,
)

class CoreClient private constructor(private val core: LeafyPuffCore) {

    suspend fun save(draft: EntryDraft): EntryDraft = withContext(Dispatchers.IO) {
        core.saveEntry(draft.toRecord()).toDraft()
    }

    suspend fun entryById(id: String): EntryDraft? = withContext(Dispatchers.IO) {
        core.entryById(id)?.toDraft()
    }

    suspend fun list(limit: UInt): List<EntryDraft> = withContext(Dispatchers.IO) {
        core.listEntries(limit).map { it.toDraft() }
    }

    suspend fun deleteAll() = withContext(Dispatchers.IO) {
        core.deleteAllEntries()
    }

    suspend fun hasVault(): Boolean = withContext(Dispatchers.IO) {
        core.hasVault()
    }

    suspend fun createVault(passphrase: String): String = withContext(Dispatchers.IO) {
        core.createVault(passphrase)
    }

    suspend fun unlock(passphrase: String) = withContext(Dispatchers.IO) {
        core.unlock(passphrase)
    }

    suspend fun unlockWithRecoveryCode(code: String) = withContext(Dispatchers.IO) {
        core.unlockWithRecoveryCode(code)
    }

    fun isUnlocked(): Boolean = core.isUnlocked()

    fun lock() {
        core.lock()
    }

    suspend fun register(
        baseUrl: String,
        email: String,
        password: String,
        displayName: String,
    ): MailedChallenge = withContext(Dispatchers.IO) {
        core.register(baseUrl, email, password, displayName.ifBlank { null }).toChallenge()
    }

    suspend fun verifyEmail(baseUrl: String, email: String, code: String) =
        withContext(Dispatchers.IO) {
            core.verifyEmail(baseUrl, email, code)
        }

    suspend fun signIn(baseUrl: String, email: String, password: String): MailedChallenge =
        withContext(Dispatchers.IO) {
            core.signIn(baseUrl, email, password).toChallenge()
        }

    suspend fun verifySignIn(baseUrl: String, email: String, code: String): IssuedSession =
        withContext(Dispatchers.IO) {
            core.verifySignIn(baseUrl, email, code).toIssued()
        }

    suspend fun statistics(range: FfiStatsRange, today: LocalDate): FfiStats =
        withContext(Dispatchers.IO) {
            core.statistics(range, today.toString())
        }

    suspend fun exportDiary(destination: String): String = withContext(Dispatchers.IO) {
        core.exportDiary(destination)
    }

    suspend fun deviceId(): String = withContext(Dispatchers.IO) {
        core.deviceId()
    }

    /// Uploads the ciphertext this device already holds and stores what comes back without
    /// opening it, so an exchange works whether or not the vault is unlocked.
    suspend fun syncNow(baseUrl: String, accessToken: String): SyncSummary =
        withContext(Dispatchers.IO) {
            core.syncNow(baseUrl, accessToken).let {
                SyncSummary(pushed = it.pushed, pulled = it.pulled, cursor = it.cursor)
            }
        }

    suspend fun importPhoto(bytes: ByteArray): ImportedPhoto = withContext(Dispatchers.IO) {
        core.importPhoto(bytes).toImported()
    }

    suspend fun photoTakenOn(bytes: ByteArray): LocalDate? = withContext(Dispatchers.IO) {
        core.photoTakenOn(bytes)?.let(LocalDate::parse)
    }

    suspend fun coverThumbnail(photoId: String): ByteArray = withContext(Dispatchers.IO) {
        core.coverThumbnail(photoId)
    }

    companion object {
        suspend fun open(dbPath: String): CoreClient = withContext(Dispatchers.IO) {
            CoreClient(LeafyPuffCore.open(dbPath))
        }
    }
}
