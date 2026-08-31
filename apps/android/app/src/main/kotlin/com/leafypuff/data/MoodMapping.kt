package com.leafypuff.data

import com.leafypuff.core.Mood as CoreMood
import com.leafypuff.domain.Mood

fun CoreMood.toDomain(): Mood = when (this) {
    CoreMood.HAPPY -> Mood.Happy
    CoreMood.CALM -> Mood.Calm
    CoreMood.GRATEFUL -> Mood.Grateful
    CoreMood.EXCITED -> Mood.Excited
    CoreMood.OKAY -> Mood.Okay
    CoreMood.TIRED -> Mood.Tired
    CoreMood.ANXIOUS -> Mood.Anxious
    CoreMood.SAD -> Mood.Sad
    CoreMood.ANGRY -> Mood.Angry
    CoreMood.SICK -> Mood.Sick
    CoreMood.LONELY -> Mood.Lonely
    CoreMood.LOVED -> Mood.Loved
}

fun Mood.toCore(): CoreMood = when (this) {
    Mood.Happy -> CoreMood.HAPPY
    Mood.Calm -> CoreMood.CALM
    Mood.Grateful -> CoreMood.GRATEFUL
    Mood.Excited -> CoreMood.EXCITED
    Mood.Okay -> CoreMood.OKAY
    Mood.Tired -> CoreMood.TIRED
    Mood.Anxious -> CoreMood.ANXIOUS
    Mood.Sad -> CoreMood.SAD
    Mood.Angry -> CoreMood.ANGRY
    Mood.Sick -> CoreMood.SICK
    Mood.Lonely -> CoreMood.LONELY
    Mood.Loved -> CoreMood.LOVED
}
