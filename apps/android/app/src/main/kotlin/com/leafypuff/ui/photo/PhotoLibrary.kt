package com.leafypuff.ui.photo

import android.graphics.Bitmap
import android.graphics.BitmapFactory
import android.graphics.Matrix
import androidx.compose.ui.graphics.ImageBitmap
import androidx.compose.ui.graphics.asImageBitmap
import androidx.exifinterface.media.ExifInterface
import com.leafypuff.core.CoreClient
import java.io.ByteArrayInputStream

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
        val decoded = BitmapFactory.decodeByteArray(bytes, 0, bytes.size, options) ?: return null
        return decoded.stoodUp(bytes).asImageBitmap()
    }
}

private fun Bitmap.stoodUp(encoded: ByteArray): Bitmap {
    val tag = runCatching { ExifInterface(ByteArrayInputStream(encoded)) }.getOrNull()
        ?: return this
    val turn = Matrix()
    if (tag.isFlipped) {
        turn.preScale(-1f, 1f)
    }
    turn.postRotate(tag.rotationDegrees.toFloat())
    if (turn.isIdentity) {
        return this
    }
    return Bitmap.createBitmap(this, 0, 0, width, height, turn, true)
}
