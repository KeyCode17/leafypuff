package com.leafypuff

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import com.leafypuff.theme.LeafyTheme
import com.leafypuff.ui.shell.LeafyApp

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            LeafyTheme {
                LeafyApp()
            }
        }
    }
}
