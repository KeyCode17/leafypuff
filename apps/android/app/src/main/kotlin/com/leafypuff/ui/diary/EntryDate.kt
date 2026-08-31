package com.leafypuff.ui.diary

import kotlinx.datetime.LocalDate

private val Weekdays = listOf("Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun")
private val Months = listOf(
    "Jan", "Feb", "Mar", "Apr", "May", "Jun",
    "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
)

fun formatEntryDate(date: LocalDate): String {
    val weekday = Weekdays[date.dayOfWeek.ordinal]
    val month = Months[date.monthNumber - 1]
    return "$weekday, ${date.dayOfMonth} $month ${date.year}"
}

fun formatMonthYear(date: LocalDate): String = "${Months[date.monthNumber - 1]} ${date.year}"
