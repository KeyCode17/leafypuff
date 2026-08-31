package com.leafypuff

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.runtime.remember
import com.leafypuff.theme.LeafyTheme
import com.leafypuff.ui.photo.CorePhotoImporter
import com.leafypuff.ui.shell.LeafyApp

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            LeafyTheme {
                LeafyApp(importer = remember { CorePhotoImporter(applicationContext) })
            }
        }
    }
}
