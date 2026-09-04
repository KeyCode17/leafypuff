package com.leafypuff.domain

import com.leafypuff.core.Location
import com.leafypuff.core.Weather
import kotlinx.datetime.LocalDate

data class Entry(
    val id: String,
    val date: LocalDate,
    val mood: Mood,
    val title: String,
    val body: String,
    val tags: List<String>,
    val coverPhotoId: String? = null,
    val weather: Weather? = null,
    val location: Location? = null,
)
