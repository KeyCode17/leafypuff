package com.leafypuff.data

import com.leafypuff.core.CoreClient
import com.leafypuff.core.SyncSummary
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

    /// No token, no exchange. A signed-out device keeps writing locally; it has nothing to
    /// push to and nobody to ask.
    suspend fun sync(baseUrl: String, accessToken: String?): SyncSummary? =
        accessToken?.let { client.syncNow(baseUrl, it) }

    suspend fun deleteAll() {
        client.deleteAll()
    }

    companion object {
        private const val DefaultLimit: UInt = 200u

        /// Opens the device database and nothing else. The vault is opened by the account
        /// password once the owner has signed in -- creating one here would mint a key that only
        /// this handset could ever use, which is what kept a diary from following its owner.
        suspend fun open(databasePath: String): EntryStore = EntryStore(CoreClient.open(databasePath))
    }
}
