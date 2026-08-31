package com.leafypuff.core

import java.io.ByteArrayOutputStream
import java.nio.file.Files
import java.util.zip.CRC32
import java.util.zip.Deflater
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertNull
import kotlin.test.assertTrue
import kotlinx.coroutines.runBlocking
import kotlinx.datetime.LocalDate

private const val VaultPassphrase = "a passphrase only this test uses"

class PhotoRoundTripTest {

    @Test
    fun anImportedPhotoComesBackAsAThreeByTwoCover() = runBlocking {
        val client = openClient()

        val imported = client.importPhoto(bandedPng(300, 400, 100))
        val cover = client.coverThumbnail(imported.id)

        val (width, height) = jpegSize(cover)
        assertEquals(300, width)
        assertEquals(200, height)
        assertEquals(width * 2, height * 3)
        assertTrue(imported.path.endsWith(imported.id))
    }

    @Test
    fun anUntaggedPhotoImportsWithoutACaptureDay() = runBlocking {
        val client = openClient()
        val picked = bandedPng(300, 400, 100)

        assertNull(client.importPhoto(picked).takenOn)
        assertNull(client.photoTakenOn(picked))
    }

    @Test
    fun aTaggedPhotoReportsTheDayItWasTaken() = runBlocking {
        val client = openClient()
        val picked = bandedPng(300, 400, 100, exifBlob("2026:07:04 09:12:33"))

        assertEquals(LocalDate(2026, 7, 4), client.photoTakenOn(picked))
        assertEquals(LocalDate(2026, 7, 4), assertNotNull(client.importPhoto(picked).takenOn))
    }

    @Test
    fun aPayloadThatIsNotAnImageIsRefused() = runBlocking {
        val client = openClient()

        val refused = runCatching { client.importPhoto("not a photograph".toByteArray()) }
        assertTrue(refused.exceptionOrNull() is LeafyPuffCoreException.Photo)
    }

    @Test
    fun aCoverThatWasNeverImportedIsRefused() = runBlocking {
        val client = openClient()

        val refused = runCatching { client.coverThumbnail("00000000-0000-4000-8000-000000000000") }
        assertTrue(refused.exceptionOrNull() is LeafyPuffCoreException.Photo)
    }
}

private val TopBand = intArrayOf(214, 38, 38)
private val BottomBand = intArrayOf(38, 62, 214)
private val PngSignature =
    byteArrayOf(0x89.toByte(), 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A)

private suspend fun openClient(): CoreClient {
    val dir = Files.createTempDirectory("leafypuff-photos")
    return CoreClient.open(dir.resolve("diary.sqlite").toString()).also { it.createVault(VaultPassphrase) }
}

private fun bandedPng(
    width: Int,
    height: Int,
    topRows: Int,
    exif: ByteArray? = null,
): ByteArray {
    val scanlines = ByteArrayOutputStream()
    for (row in 0 until height) {
        scanlines.write(0)
        val band = if (row < topRows) TopBand else BottomBand
        for (column in 0 until width) {
            band.forEach(scanlines::write)
        }
    }

    val out = ByteArrayOutputStream()
    out.write(PngSignature)
    out.write(chunk("IHDR", beInt(width) + beInt(height) + byteArrayOf(8, 2, 0, 0, 0)))
    if (exif != null) {
        out.write(chunk("eXIf", exif))
    }
    out.write(chunk("IDAT", deflate(scanlines.toByteArray())))
    out.write(chunk("IEND", ByteArray(0)))
    return out.toByteArray()
}

private fun chunk(type: String, data: ByteArray): ByteArray {
    val body = type.toByteArray() + data
    val crc = CRC32().apply { update(body) }.value
    return beInt(data.size) + body + beInt(crc.toInt())
}

private fun beInt(value: Int): ByteArray = byteArrayOf(
    (value ushr 24).toByte(),
    (value ushr 16).toByte(),
    (value ushr 8).toByte(),
    value.toByte(),
)

private fun deflate(data: ByteArray): ByteArray {
    val deflater = Deflater()
    deflater.setInput(data)
    deflater.finish()
    val out = ByteArrayOutputStream()
    val buffer = ByteArray(8192)
    while (!deflater.finished()) {
        out.write(buffer, 0, deflater.deflate(buffer))
    }
    deflater.end()
    return out.toByteArray()
}

private fun exifBlob(stamp: String): ByteArray {
    val ascii = stamp.toByteArray() + 0
    val out = ByteArrayOutputStream()
    out.write("MM".toByteArray())
    out.write(byteArrayOf(0, 42, 0, 0, 0, 8))
    out.write(byteArrayOf(0, 1))
    out.write(byteArrayOf(0x87.toByte(), 0x69, 0, 4, 0, 0, 0, 1, 0, 0, 0, 26))
    out.write(byteArrayOf(0, 0, 0, 0))
    out.write(byteArrayOf(0, 1))
    out.write(byteArrayOf(0x90.toByte(), 3, 0, 2, 0, 0, 0, ascii.size.toByte(), 0, 0, 0, 44))
    out.write(byteArrayOf(0, 0, 0, 0))
    out.write(ascii)
    return out.toByteArray()
}

private fun jpegSize(jpeg: ByteArray): Pair<Int, Int> {
    var at = 2
    while (at + 9 < jpeg.size) {
        if (jpeg[at] != 0xFF.toByte()) {
            at += 1
            continue
        }
        val marker = jpeg[at + 1].toInt() and 0xFF
        val length = ((jpeg[at + 2].toInt() and 0xFF) shl 8) or (jpeg[at + 3].toInt() and 0xFF)
        if (marker in 0xC0..0xCF && marker != 0xC4 && marker != 0xC8 && marker != 0xCC) {
            val height = ((jpeg[at + 5].toInt() and 0xFF) shl 8) or (jpeg[at + 6].toInt() and 0xFF)
            val width = ((jpeg[at + 7].toInt() and 0xFF) shl 8) or (jpeg[at + 8].toInt() and 0xFF)
            return width to height
        }
        at += 2 + length
    }
    return 0 to 0
}
