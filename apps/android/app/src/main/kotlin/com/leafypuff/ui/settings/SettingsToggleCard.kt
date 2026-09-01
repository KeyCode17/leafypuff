package com.leafypuff.ui.settings

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.RowScope
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.KeyboardArrowDown
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.leafypuff.theme.LeafyShapes
import com.leafypuff.theme.LocalLeafyColors
import com.leafypuff.theme.LocalLeafyTypography
import kotlinx.datetime.LocalTime

private val SubLabelGap = 2.dp
private val PillPaddingH = 14.dp
private val PillPaddingV = 8.dp
private val PillGap = 8.dp
private val ChevronSize = 14.dp
private val SubLabelFontSize = 12.sp
private val PillFontSize = 14.sp

@Composable
internal fun SettingsToggleCard(
    darkMode: Boolean,
    reminderEnabled: Boolean,
    reminderTime: LocalTime,
    lockEnabled: Boolean,
    onToggleDark: (Boolean) -> Unit,
    onToggleReminder: (Boolean) -> Unit,
    onReminderTimeClick: () -> Unit,
    onToggleLock: (Boolean) -> Unit,
    modifier: Modifier = Modifier,
) {
    SettingsCard(padding = ListCardPadding, modifier = modifier) {
        ToggleRow(
            label = "Dark mode",
            subLabel = null,
            checked = darkMode,
            onToggle = onToggleDark,
        )
        SettingsDivider()
        ToggleRow(
            label = "Daily writing reminder",
            subLabel = if (reminderEnabled) "Every day at ${formatTime12(reminderTime)}" else "Off",
            checked = reminderEnabled,
            onToggle = onToggleReminder,
        )
        if (reminderEnabled) {
            ReminderAtRow(time = reminderTime, onClick = onReminderTimeClick)
        }
        SettingsDivider()
        ToggleRow(
            label = "PIN / Face ID lock",
            // The design gives the reminder row a sub-line that changes with its switch; the lock
            // row earns the same, because "off" and "asks on every open" are worth telling apart.
            subLabel = if (lockEnabled) "Ask when opening leafyPuff" else "Off",
            checked = lockEnabled,
            onToggle = onToggleLock,
        )
    }
}

@Composable
private fun ToggleRow(
    label: String,
    subLabel: String?,
    checked: Boolean,
    onToggle: (Boolean) -> Unit,
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .clickable { onToggle(!checked) }
            .padding(vertical = CardRowPaddingV),
        horizontalArrangement = Arrangement.spacedBy(CardContentGap),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        RowLabel(label = label, subLabel = subLabel)
        SettingsSwitch(checked = checked)
    }
}

@Composable
private fun RowScope.RowLabel(label: String, subLabel: String?) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Column(
        modifier = Modifier.weight(1f),
        verticalArrangement = Arrangement.spacedBy(SubLabelGap),
    ) {
        Text(
            text = label,
            style = typography.body.copy(fontWeight = FontWeight.W500),
            color = colors.ink,
        )
        if (subLabel != null) {
            Text(
                text = subLabel,
                style = typography.chipLabel.copy(
                    fontSize = SubLabelFontSize,
                    fontWeight = FontWeight.W400,
                ),
                color = colors.ink3,
            )
        }
    }
}

@Composable
private fun ReminderAtRow(time: LocalTime, onClick: () -> Unit) {
    val colors = LocalLeafyColors.current
    val typography = LocalLeafyTypography.current

    Row(
        modifier = Modifier
            .fillMaxWidth()
            .padding(bottom = CardRowPaddingV),
        horizontalArrangement = Arrangement.spacedBy(CardContentGap),
        verticalAlignment = Alignment.CenterVertically,
    ) {
        Text(
            text = "Remind me at",
            style = typography.chipLabel.copy(
                fontSize = SubLabelFontSize,
                fontWeight = FontWeight.W400,
            ),
            color = colors.ink3,
            modifier = Modifier.weight(1f),
        )
        Row(
            modifier = Modifier
                .clip(LeafyShapes.pill)
                .background(colors.soft2)
                .clickable(onClick = onClick)
                .padding(horizontal = PillPaddingH, vertical = PillPaddingV),
            horizontalArrangement = Arrangement.spacedBy(PillGap),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Text(
                text = formatTime12(time),
                style = typography.noteTitleInput.copy(fontSize = PillFontSize),
                color = colors.ink,
            )
            Icon(
                imageVector = Icons.Filled.KeyboardArrowDown,
                contentDescription = "Change the reminder time",
                tint = colors.accentDeep,
                modifier = Modifier.size(ChevronSize),
            )
        }
    }
}
