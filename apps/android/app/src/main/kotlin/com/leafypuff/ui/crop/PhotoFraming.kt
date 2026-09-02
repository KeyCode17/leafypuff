package com.leafypuff.ui.crop

data class PhotoFraming(
    val x: Double = 0.0,
    val y: Double = 0.0,
    val width: Double = 1.0,
) {
    fun panned(
        acrossFraction: Double,
        downFraction: Double,
        tallness: Double,
        ratio: Double = CoverTallness,
    ): PhotoFraming =
        copy(
            x = (x + acrossFraction).coerceIn(0.0, 1.0 - width),
            y = (y + downFraction).coerceIn(0.0, 1.0 - height(tallness, ratio)),
        )

    fun zoomed(
        factor: Double,
        tallness: Double,
        ratio: Double = CoverTallness,
    ): PhotoFraming {
        val wanted = (width / factor).coerceIn(SmallestWidth, 1.0)
        val settled = copy(width = wanted)
        return settled.copy(
            x = settled.x.coerceIn(0.0, 1.0 - wanted),
            y = settled.y.coerceIn(0.0, (1.0 - settled.height(tallness, ratio)).coerceAtLeast(0.0)),
        )
    }

    fun height(tallness: Double, ratio: Double = CoverTallness): Double =
        (width * ratio * tallness).coerceAtMost(1.0)

    companion object {
        const val SmallestWidth = 0.2
        const val CoverTallness = 2.0 / 3.0
        const val SquareTallness = 1.0
    }
}
