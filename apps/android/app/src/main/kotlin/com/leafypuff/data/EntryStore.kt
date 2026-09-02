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

    suspend fun openForEdit(id: String): StoredEntry? = client.entryById(id)?.let {
        StoredEntry(draft = it.toUiDraft(), photoIds = it.photoIds())
    }

    suspend fun save(draft: EntryDraft, photoIds: List<String>): Entry =
        client.save(draft.toCoreDraft(photoIds)).toEntry()

    suspend fun statistics(range: StatRange, today: LocalDate): StatsSummary =
        client.statistics(range.toCore(), today).toSummary()

    suspend fun export(destination: String): String = client.exportDiary(destination)

    suspend fun sync(baseUrl: String, accessToken: String?): SyncSummary? =
        accessToken?.let { client.syncNow(baseUrl, it) }

    suspend fun forgetPhoto(baseUrl: String, accessToken: String?, photoId: String) {
        accessToken?.let { client.forgetPhoto(baseUrl, it, photoId) }
    }

    suspend fun deleteAll() {
        client.deleteAll()
    }

    companion object {
        private const val DefaultLimit: UInt = 200u

        suspend fun open(databasePath: String): EntryStore = EntryStore(CoreClient.open(databasePath))
    }
}
