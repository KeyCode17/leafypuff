package com.leafypuff.theme

import androidx.compose.runtime.Immutable
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.em
import androidx.compose.ui.unit.sp

@Immutable
data class LeafyTypography(
    val screenTitle: TextStyle,
    val authTitle: TextStyle,
    val lockTitle: TextStyle,
    val statFigure: TextStyle,
    val monthLabel: TextStyle,
    val noteTitleInput: TextStyle,
    val cardTitle: TextStyle,
    val metaLabel: TextStyle,
    val buttonLabel: TextStyle,
    val fieldToggle: TextStyle,
    val body: TextStyle,
    val chipLabel: TextStyle,
)

private const val BODY_LINE_HEIGHT_RATIO = 1.65f

fun leafyTypography(scale: LeafyTypeScale): LeafyTypography = LeafyTypography(
    screenTitle = TextStyle(
        fontFamily = Rubik,
        fontWeight = FontWeight.W600,
        fontSize = 26.sp,
        letterSpacing = (-0.01).em,
    ),
    authTitle = TextStyle(
        fontFamily = Rubik,
        fontWeight = FontWeight.W600,
        fontSize = 28.sp,
        letterSpacing = (-0.01).em,
    ),
    lockTitle = TextStyle(
        fontFamily = Rubik,
        fontWeight = FontWeight.W600,
        fontSize = 30.sp,
        letterSpacing = (-0.01).em,
    ),
    statFigure = TextStyle(
        fontFamily = Rubik,
        fontWeight = FontWeight.W600,
        fontSize = 32.sp,
    ),
    monthLabel = TextStyle(
        fontFamily = Rubik,
        fontWeight = FontWeight.W600,
        fontSize = 20.sp,
    ),
    noteTitleInput = TextStyle(
        fontFamily = Rubik,
        fontWeight = FontWeight.W500,
        fontSize = 21.sp,
    ),
    cardTitle = TextStyle(
        fontFamily = Rubik,
        fontWeight = FontWeight.W500,
        fontSize = scale.title,
    ),
    metaLabel = TextStyle(
        fontFamily = Rubik,
        fontWeight = FontWeight.W500,
        fontSize = scale.meta,
        letterSpacing = 0.06.em,
    ),
    buttonLabel = TextStyle(
        fontFamily = Rubik,
        fontWeight = FontWeight.W600,
        fontSize = 13.sp,
        letterSpacing = 0.04.em,
    ),
    fieldToggle = TextStyle(
        fontFamily = Rubik,
        fontWeight = FontWeight.W600,
        fontSize = 11.sp,
        letterSpacing = 0.04.em,
    ),
    body = TextStyle(
        fontFamily = Inter,
        fontWeight = FontWeight.W400,
        fontSize = scale.body,
        lineHeight = scale.body * BODY_LINE_HEIGHT_RATIO,
    ),
    chipLabel = TextStyle(
        fontFamily = Inter,
        fontWeight = FontWeight.W500,
        fontSize = 12.sp,
    ),
)
