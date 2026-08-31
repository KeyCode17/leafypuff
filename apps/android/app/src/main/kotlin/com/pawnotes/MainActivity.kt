package com.pawnotes

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import com.pawnotes.theme.PawTheme
import com.pawnotes.ui.shell.PawApp

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            PawTheme {
                PawApp()
            }
        }
    }
}
