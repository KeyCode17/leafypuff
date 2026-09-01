package com.leafypuff.ui.common

import androidx.compose.runtime.Immutable
import kotlinx.datetime.LocalDate

@Immutable
data class ToastPrompt(val accept: String, val reject: String)

@Immutable
data class ToastRequest(val text: String, val prompt: ToastPrompt? = null)

private val AcceptOrReject = ToastPrompt(accept = "Accept", reject = "Reject")

fun plainToast(text: String): ToastRequest = ToastRequest(text)

fun saveToast(reopened: Boolean): ToastRequest = when {
    reopened -> plainToast("Entry updated.")
    else -> plainToast("Entry saved.")
}

fun exifPromptToast(day: LocalDate): ToastRequest =
    ToastRequest(text = exifPromptText(day), prompt = AcceptOrReject)

fun exifPromptText(day: LocalDate): String =
    "This photo was taken on ${formatShortDate(day)}. Use that as the entry date?"
