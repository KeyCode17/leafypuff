package com.leafypuff.data

import com.leafypuff.core.Location
import com.leafypuff.core.Weather

fun Weather.label(): String = when (this) {
    Weather.SUNNY -> "Sunny"
    Weather.CLOUDY -> "Cloudy"
    Weather.RAINY -> "Rainy"
    Weather.WINDY -> "Windy"
}

fun weatherFromLabel(label: String?): Weather? = when (label) {
    "Sunny" -> Weather.SUNNY
    "Cloudy" -> Weather.CLOUDY
    "Rainy" -> Weather.RAINY
    "Windy" -> Weather.WINDY
    else -> null
}

fun Location.label(): String = when (this) {
    Location.HOME -> "Home"
    Location.CAFE -> "Cafe"
    Location.OFFICE -> "Office"
    Location.PARK -> "Park"
    Location.ON_THE_ROAD -> "On the road"
}

fun locationFromLabel(label: String?): Location? = when (label) {
    "Home" -> Location.HOME
    "Cafe" -> Location.CAFE
    "Office" -> Location.OFFICE
    "Park" -> Location.PARK
    "On the road" -> Location.ON_THE_ROAD
    else -> null
}
