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

fun formatShortDate(date: LocalDate): String =
    "${date.dayOfMonth} ${ShortMonths[date.monthNumber - 1]} ${date.year}"

fun formatEntryDate(date: LocalDate): String =
    "${DayNames[date.dayOfWeek.ordinal]}, ${formatShortDate(date)}"

fun formatMonthYear(date: LocalDate): String = "${FullMonths[date.monthNumber - 1]} ${date.year}"

fun formatShortMonth(month: Int): String = ShortMonths[month - 1]
