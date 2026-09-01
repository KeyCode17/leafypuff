package com.leafypuff.ui.shell

import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.BarChart
import androidx.compose.material.icons.filled.CalendarMonth
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material.icons.automirrored.filled.EventNote
import androidx.compose.ui.graphics.vector.ImageVector

enum class Destination(val label: String, val glyph: ImageVector) {
    Diary("Diary", Icons.AutoMirrored.Filled.EventNote),
    Calendar("Calendar", Icons.Filled.CalendarMonth),
    Statistics("Statistics", Icons.Filled.BarChart),
    Settings("Settings", Icons.Filled.Settings),
}
