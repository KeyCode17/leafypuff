package com.leafypuff

import android.os.Bundle
import androidx.activity.compose.setContent
import androidx.fragment.app.FragmentActivity
import com.leafypuff.ui.shell.LeafyApp
import java.io.File

class MainActivity : FragmentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val databasePath = File(applicationContext.filesDir, "leafypuff.db").absolutePath
        setContent {
            LeafyApp(
                databasePath = databasePath,
                versionName = BuildConfig.VERSION_NAME,
                apiBaseUrl = BuildConfig.API_BASE_URL,
            )
        }
    }
}
