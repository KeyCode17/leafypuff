package com.leafypuff.ui.photo

import android.content.Context
import android.content.pm.PackageManager
import android.net.Uri
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.PickVisualMediaRequest
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.PhotoCamera
import androidx.compose.material.icons.filled.PhotoLibrary
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.platform.LocalContext
import androidx.core.content.FileProvider
import com.leafypuff.ui.popups.OptionPopup
import com.leafypuff.ui.popups.PopupOption
import java.io.File

private const val SourceTitle = "Add a photo"
private const val CameraLabel = "Take a photo"
private const val GalleryLabel = "Choose from gallery"
private const val ShotDirectory = "camera"
private const val ShotPrefix = "shot-"
private const val ShotSuffix = ".jpg"
private const val ProviderSuffix = ".photos"

private val PhotoSources = listOf(
    PopupOption(CameraLabel, Icons.Filled.PhotoCamera),
    PopupOption(GalleryLabel, Icons.Filled.PhotoLibrary),
)

@Composable
fun rememberPhotoPicker(onPicked: (ByteArray) -> Unit): () -> Unit {
    val context = LocalContext.current
    val hasCamera = remember(context) {
        context.packageManager.hasSystemFeature(PackageManager.FEATURE_CAMERA_ANY)
    }
    var choosing by remember { mutableStateOf(false) }
    var shot by remember { mutableStateOf<File?>(null) }

    val gallery = rememberLauncherForActivityResult(
        contract = ActivityResultContracts.PickVisualMedia(),
    ) { picked -> picked?.let { readPicked(context, it) }?.let(onPicked) }

    val camera = rememberLauncherForActivityResult(
        contract = ActivityResultContracts.TakePicture(),
    ) { taken ->
        val file = shot
        shot = null
        if (taken && file != null) {
            readShot(file)?.let(onPicked)
        }
        file?.delete()
    }

    val openGallery = {
        gallery.launch(
            PickVisualMediaRequest(ActivityResultContracts.PickVisualMedia.ImageOnly),
        )
    }

    val openCamera = {
        val file = shotFile(context)
        shot = file
        camera.launch(sharedUri(context, file))
    }

    if (choosing) {
        OptionPopup(
            title = SourceTitle,
            options = PhotoSources,
            selected = null,
            onSelect = { chosen ->
                choosing = false
                if (chosen == CameraLabel) openCamera() else openGallery()
            },
            onDismiss = { choosing = false },
        )
    }

    return {
        if (hasCamera) {
            choosing = true
        } else {
            openGallery()
        }
    }
}

private fun shotFile(context: Context): File {
    val dir = File(context.cacheDir, ShotDirectory)
    dir.mkdirs()
    return File(dir, "$ShotPrefix${System.currentTimeMillis()}$ShotSuffix")
}

private fun sharedUri(context: Context, file: File): Uri =
    FileProvider.getUriForFile(context, "${context.packageName}$ProviderSuffix", file)

private fun readShot(file: File): ByteArray? =
    runCatching { file.readBytes() }.getOrNull()?.takeIf { it.isNotEmpty() }

private fun readPicked(context: Context, uri: Uri): ByteArray? =
    runCatching { context.contentResolver.openInputStream(uri)?.use { it.readBytes() } }
        .getOrNull()
