package com.leafypuff.ui.photo

import android.graphics.BitmapFactory
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.asImageBitmap
import com.leafypuff.core.CoreClient

interface PhotoLibrary {
    suspend fun import(bytes: ByteArray): EntryPhoto?

    suspend fun thumbnail(photoId: String): ImageBitmap?

    suspend fun original(photoId: String): ImageBitmap?
}

object NoPhotoLibrary : PhotoLibrary {
    override suspend fun import(bytes: ByteArray): EntryPhoto? = null

    override suspend fun thumbnail(photoId: String): ImageBitmap? = null

    override suspend fun original(photoId: String): ImageBitmap? = null
}

private const val LongestEdgeForPlacing = 1200

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

    override suspend fun original(photoId: String): ImageBitmap? {
        val bytes = runCatching { client.originalPhoto(photoId) }.getOrNull() ?: return null
        val bounds = BitmapFactory.Options().apply { inJustDecodeBounds = true }
        BitmapFactory.decodeByteArray(bytes, 0, bytes.size, bounds)
        val longest = maxOf(bounds.outWidth, bounds.outHeight)
        var sample = 1
        while (longest / (sample * 2) >= LongestEdgeForPlacing) {
            sample *= 2
        }
        val options = BitmapFactory.Options().apply { inSampleSize = sample }
        return BitmapFactory.decodeByteArray(bytes, 0, bytes.size, options)?.asImageBitmap()
    }
}
