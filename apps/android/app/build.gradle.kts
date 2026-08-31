plugins {
    alias(libs.plugins.android.application)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.kotlin.compose)
}

val workspaceDir = rootProject.file("../..")
val hostLib = if (System.getProperty("os.name").startsWith("Mac")) {
    "libleafypuff_core.dylib"
} else {
    "libleafypuff_core.so"
}
val uniffiOutDir = layout.buildDirectory.dir("generated/uniffi")

val cargoBuildHost by tasks.registering(Exec::class) {
    workingDir = workspaceDir
    commandLine("cargo", "build", "-p", "leafypuff-core", "--features", "ffi-bindgen")
}

val generateUniffiBindings by tasks.registering(Exec::class) {
    dependsOn(cargoBuildHost)
    workingDir = workspaceDir
    commandLine(
        "cargo", "run", "-p", "leafypuff-core", "--features", "ffi-bindgen",
        "--bin", "uniffi-bindgen", "--", "generate",
        "--library", "target/debug/$hostLib",
        "--language", "kotlin",
        "--out-dir", uniffiOutDir.get().asFile.absolutePath,
        "--no-format",
    )
}

android {
    namespace = "com.leafypuff"
    compileSdk = 35

    defaultConfig {
        applicationId = "com.leafypuff"
        minSdk = 23
        targetSdk = 35
        versionCode = 3
        versionName = "0.3.0"
    }

    buildFeatures {
        compose = true
    }

    kotlinOptions {
        jvmTarget = "17"
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    sourceSets["main"].kotlin.srcDir(uniffiOutDir)
}

dependencies {
    implementation(platform(libs.compose.bom))
    implementation(libs.compose.ui)
    implementation(libs.compose.foundation)
    implementation(libs.compose.material3)
    implementation(libs.compose.ui.tooling.preview)
    debugImplementation(libs.compose.ui.tooling)
    implementation(libs.androidx.activity.compose)
    implementation(libs.androidx.biometric)
    implementation(libs.kotlinx.datetime)
    implementation(libs.jna) { artifact { type = "aar" } }
    implementation(libs.kotlinx.coroutines.core)
    testImplementation(libs.kotlin.test)
    testImplementation(libs.jna)
}

tasks.withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile>().configureEach {
    dependsOn(generateUniffiBindings)
}

tasks.withType<Test>().configureEach {
    dependsOn(cargoBuildHost)
    systemProperty("jna.library.path", File(workspaceDir, "target/debug").absolutePath)
}
