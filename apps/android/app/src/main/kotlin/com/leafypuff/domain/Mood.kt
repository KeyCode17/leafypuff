package com.leafypuff.domain

enum class MoodGroup { Positive, Neutral, Negative }

enum class EyeStyle { Arc, Closed, Wide, Dot, Angry }

enum class MouthStyle { Smile, Flat, Open, Wavy, Frown }

enum class FaceProp { Blush, Tear, SleepZ, Plaster, Heart }

enum class EarTilt(val degrees: Float) { Upright(12f), Drooped(22f) }

enum class Mood(
    val label: String,
    val dotArgb: Long,
    val eyes: EyeStyle,
    val mouth: MouthStyle,
    val props: List<FaceProp> = emptyList(),
    val earTilt: EarTilt = EarTilt.Upright,
) {
    Happy("Happy", 0xFF8B9A5F, EyeStyle.Arc, MouthStyle.Smile, listOf(FaceProp.Blush)),
    Calm("Calm", 0xFFA8B98B, EyeStyle.Closed, MouthStyle.Smile),
    Grateful("Grateful", 0xFFC3CE9E, EyeStyle.Closed, MouthStyle.Smile, listOf(FaceProp.Blush)),
    Excited("Excited", 0xFFE3C766, EyeStyle.Wide, MouthStyle.Open),
    Okay("Okay", 0xFFC3C8B4, EyeStyle.Dot, MouthStyle.Flat),
    Tired("Tired", 0xFFA3A896, EyeStyle.Closed, MouthStyle.Flat, listOf(FaceProp.SleepZ)),
    Anxious("Anxious", 0xFFC9A87A, EyeStyle.Dot, MouthStyle.Wavy),
    Sad("Sad", 0xFF8FA0A8, EyeStyle.Dot, MouthStyle.Frown, listOf(FaceProp.Tear)),
    Angry("Angry", 0xFFEF4E4E, EyeStyle.Angry, MouthStyle.Frown),
    Sick("Sick", 0xFF9CB49A, EyeStyle.Closed, MouthStyle.Wavy, listOf(FaceProp.Plaster)),
    Lonely("Lonely", 0xFFA6A2AE, EyeStyle.Dot, MouthStyle.Flat, emptyList(), EarTilt.Drooped),
    Loved("Loved", 0xFFE0909A, EyeStyle.Closed, MouthStyle.Smile, listOf(FaceProp.Blush, FaceProp.Heart)),
    ;

    val group: MoodGroup
        get() = when (this) {
            Happy, Calm, Grateful, Excited, Loved -> MoodGroup.Positive
            Okay, Tired -> MoodGroup.Neutral
            Anxious, Sad, Angry, Sick, Lonely -> MoodGroup.Negative
        }
}
