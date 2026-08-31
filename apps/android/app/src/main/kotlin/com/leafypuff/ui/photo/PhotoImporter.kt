package com.leafypuff.ui.photo

interface PhotoImporter {
    suspend fun import(bytes: ByteArray): EntryPhoto?
}

object NoPhotoImporter : PhotoImporter {
    override suspend fun import(bytes: ByteArray): EntryPhoto? = null
}
