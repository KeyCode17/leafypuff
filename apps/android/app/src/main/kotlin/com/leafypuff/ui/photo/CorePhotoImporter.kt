package com.leafypuff.ui.photo

import android.content.Context
import android.graphics.BitmapFactory
import androidx.compose.ui.graphics.asImageBitmap
import com.leafypuff.core.CoreClient
import java.io.File
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock

private const val DatabaseName = "diary.sqlite"

class CorePhotoImporter(private val context: Context) : PhotoImporter {
    private val gate = Mutex()
    private var opened: CoreClient? = null

    override suspend fun import(bytes: ByteArray): EntryPhoto? {
        val core = runCatching { open() }.getOrNull() ?: return null
        val imported = runCatching { core.importPhoto(bytes) }.getOrNull() ?: return null
        val cover = runCatching { core.coverThumbnail(imported.id) }.getOrNull() ?: return null
        val drawn = BitmapFactory.decodeByteArray(cover, 0, cover.size) ?: return null
        return EntryPhoto(imported.id, drawn.asImageBitmap(), imported.takenOn)
    }

    private suspend fun open(): CoreClient = gate.withLock {
        opened ?: CoreClient.open(databasePath()).also { opened = it }
    }

    private fun databasePath(): String = File(context.filesDir, DatabaseName).absolutePath
}
