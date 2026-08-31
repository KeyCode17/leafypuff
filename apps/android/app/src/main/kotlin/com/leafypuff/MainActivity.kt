package com.leafypuff

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.runtime.remember
import com.leafypuff.ui.photo.CorePhotoImporter
import com.leafypuff.ui.shell.LeafyApp
import java.io.File

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val databasePath = File(applicationContext.filesDir, "leafypuff.db").absolutePath
        setContent {
            LeafyApp(
                importer = remember { CorePhotoImporter(applicationContext) },
                databasePath = databasePath,
                versionName = BuildConfig.VERSION_NAME,
            )
        }
    }
}
