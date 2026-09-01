package com.leafypuff.ui.photo

import android.graphics.BitmapFactory
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.asImageBitmap
import com.leafypuff.core.CoreClient

interface PhotoLibrary {
    suspend fun import(bytes: ByteArray): EntryPhoto?

    suspend fun thumbnail(photoId: String): ImageBitmap?
}

object NoPhotoLibrary : PhotoLibrary {
    override suspend fun import(bytes: ByteArray): EntryPhoto? = null

    override suspend fun thumbnail(photoId: String): ImageBitmap? = null
}

/**
 * Photos go through the same core handle the entries do. A second handle would mean a second
 * database and a second vault, and a photo sealed under one of them is unreadable from the other.
 */
class CorePhotoLibrary(private val client: CoreClient) : PhotoLibrary {

    override suspend fun import(bytes: ByteArray): EntryPhoto? {
        val imported = runCatching { client.importPhoto(bytes) }.getOrNull() ?: return null
        val cover = thumbnail(imported.id) ?: return null
        return EntryPhoto(imported.id, cover, imported.takenOn)
    }

    override suspend fun thumbnail(photoId: String): ImageBitmap? {
        val bytes = runCatching { client.coverThumbnail(photoId) }.getOrNull() ?: return null
        return BitmapFactory.decodeByteArray(bytes, 0, bytes.size)?.asImageBitmap()
    }
}
