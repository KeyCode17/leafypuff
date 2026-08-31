package com.leafypuff.ui.common

import kotlinx.datetime.LocalDate

private val DayNames = listOf(
    "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday",
)
private val ShortMonths = listOf(
    "Jan", "Feb", "Mar", "Apr", "May", "Jun",
    "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
)
private val FullMonths = listOf(
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December",
)

fun formatEntryDate(date: LocalDate): String {
    val weekday = DayNames[date.dayOfWeek.ordinal]
    val month = ShortMonths[date.monthNumber - 1]
    return "$weekday, ${date.dayOfMonth} $month ${date.year}"
}

fun formatMonthYear(date: LocalDate): String = "${FullMonths[date.monthNumber - 1]} ${date.year}"
