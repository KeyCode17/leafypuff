package com.leafypuff.data

import com.leafypuff.core.CoreClient
import com.leafypuff.domain.Entry
import com.leafypuff.ui.editor.EntryDraft

class EntryStore(private val client: CoreClient) {

    suspend fun list(limit: UInt = DefaultLimit): List<Entry> =
        client.list(limit).map { it.toEntry() }

    suspend fun draftById(id: String): EntryDraft? = client.entryById(id)?.toUiDraft()

    suspend fun save(draft: EntryDraft, photoIds: List<String>): Entry =
        client.save(draft.toCoreDraft(photoIds)).toEntry()

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
