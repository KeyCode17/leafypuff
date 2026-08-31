package com.leafypuff.data

import com.leafypuff.domain.Entry
import com.leafypuff.domain.Mood
import kotlinx.datetime.LocalDate

val SampleEntries: List<Entry> = listOf(
    Entry(
        id = "e1",
        date = LocalDate(2026, 8, 31),
        mood = Mood.Calm,
        title = "Quiet morning",
        body = "Tea on the balcony before anyone else was awake. The street was still wet from last night and nothing needed doing yet.",
        tags = listOf("#slowday", "#home"),
    ),
    Entry(
        id = "e2",
        date = LocalDate(2026, 8, 30),
        mood = Mood.Grateful,
        title = "Long call with Rina",
        body = "Two hours that felt like twenty minutes. We talked about the move and she made it sound possible instead of frightening.",
        tags = listOf("#family"),
    ),
    Entry(
        id = "e3",
        date = LocalDate(2026, 8, 29),
        mood = Mood.Tired,
        title = "Deploy day",
        body = "Shipped it, then sat staring at the logs for an hour longer than I needed to. Slept badly.",
        tags = listOf("#work"),
    ),
    Entry(
        id = "e4",
        date = LocalDate(2026, 8, 28),
        mood = Mood.Happy,
        title = "Rain, then coffee",
        body = "Got caught in it walking back and gave up on staying dry. Ended up in the small place near the station.",
        tags = listOf("#rain", "#coffee", "#walk"),
    ),
    Entry(
        id = "e5",
        date = LocalDate(2026, 8, 27),
        mood = Mood.Okay,
        title = "Nothing much",
        body = "An ordinary Thursday. Writing it down anyway, because the ordinary ones disappear first.",
        tags = listOf("#rest"),
    ),
)
