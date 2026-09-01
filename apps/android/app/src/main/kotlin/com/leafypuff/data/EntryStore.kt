package com.leafypuff.data

import com.leafypuff.core.CoreClient
import com.leafypuff.domain.Entry
import com.leafypuff.ui.stats.StatRange
import com.leafypuff.ui.stats.StatsSummary
import com.leafypuff.ui.editor.EntryDraft
import kotlinx.datetime.LocalDate

data class StoredEntry(val draft: EntryDraft, val photoIds: List<String>)

class EntryStore(internal val client: CoreClient) {

    suspend fun list(limit: UInt = DefaultLimit): List<Entry> =
        client.list(limit).map { it.toEntry() }

    /// Draft and photo ids in one read. The editor needs both, and the entry row already carries
    /// them, so asking twice would decrypt the same record twice.
    suspend fun openForEdit(id: String): StoredEntry? = client.entryById(id)?.let {
        StoredEntry(draft = it.toUiDraft(), photoIds = it.photoIds())
    }

    suspend fun save(draft: EntryDraft, photoIds: List<String>): Entry =
        client.save(draft.toCoreDraft(photoIds)).toEntry()

    suspend fun statistics(range: StatRange, today: LocalDate): StatsSummary =
        client.statistics(range.toCore(), today).toSummary()

    suspend fun export(destination: String): String = client.exportDiary(destination)

    suspend fun deleteAll() {
        client.deleteAll()
    }

    companion object {
        private const val DefaultLimit: UInt = 200u

        /// Opens the device database and unlocks its vault, creating one on first run. Every
        /// entry field is sealed at rest, so a store handed out locked would fail on the first
        /// write rather than here.
        suspend fun open(databasePath: String, passphrase: String): EntryStore {
            val client = CoreClient.open(databasePath)
            if (client.hasVault()) {
                client.unlock(passphrase)
            } else {
                client.createVault(passphrase)
            }
            return EntryStore(client)
        }
    }
}
