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
        copy(x = x + acrossFraction, y = y + downFraction).fitted(tallness, ratio)

    fun zoomed(
        factor: Double,
        tallness: Double,
        ratio: Double = CoverTallness,
    ): PhotoFraming = copy(width = width / factor).fitted(tallness, ratio)

    fun fitted(tallness: Double, ratio: Double = CoverTallness): PhotoFraming {
        val settled = width.coerceIn(SmallestWidth, widest(tallness, ratio))
        val down = settled * ratio * tallness
        return PhotoFraming(
            x = x.coerceIn(0.0, (1.0 - settled).coerceAtLeast(0.0)),
            y = y.coerceIn(0.0, (1.0 - down).coerceAtLeast(0.0)),
            width = settled,
        )
    }

    fun height(tallness: Double, ratio: Double = CoverTallness): Double =
        (width * ratio * tallness).coerceAtMost(1.0)

    fun widest(tallness: Double, ratio: Double = CoverTallness): Double = when {
        tallness <= 0.0 || ratio <= 0.0 -> 1.0
        else -> (1.0 / (ratio * tallness)).coerceIn(SmallestWidth, 1.0)
    }

    companion object {
        const val SmallestWidth = 0.2
        const val CoverTallness = 2.0 / 3.0
        const val SquareTallness = 1.0
    }
}
