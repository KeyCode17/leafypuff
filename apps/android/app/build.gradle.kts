import java.util.Properties

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

val androidSdkDir: String = System.getenv("ANDROID_HOME")
    ?: System.getenv("ANDROID_SDK_ROOT")
    ?: Properties().apply {
        rootProject.file("local.properties").inputStream().use { load(it) }
    }.getProperty("sdk.dir")
val androidNdkDir = "$androidSdkDir/ndk/27.2.12479018"
val androidAbis = listOf("arm64-v8a", "armeabi-v7a", "x86_64")

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

val cargoBuildAndroid by tasks.registering(Exec::class) {
    workingDir = workspaceDir
    environment("ANDROID_NDK_HOME", androidNdkDir)
    commandLine(
        listOf("cargo", "ndk")
            + androidAbis.flatMap { listOf("-t", it) }
            + listOf("-o", file("src/main/jniLibs").absolutePath)
            + listOf("build", "-p", "leafypuff-core", "--features", "ffi", "--release"),
    )
}

tasks.named("preBuild") { dependsOn(cargoBuildAndroid) }

android {
    namespace = "com.leafypuff"
    compileSdk = 35

    defaultConfig {
        applicationId = "com.leafypuff"
        minSdk = 23
        targetSdk = 35
        versionCode = 31
        versionName = "0.22.0"

        // The deployed API. A build field rather than a constant in source so a debug build
        // can be pointed at a local api without editing Kotlin.
        buildConfigField(
            "String",
            "API_BASE_URL",
            "\"${project.findProperty("leafypuff.apiBaseUrl") ?: "https://leafypuff-api.daffakaryudi.web.id"}\"",
        )

        // JNA ships jnidispatch for mips, mips64, armeabi and x86 — architectures Android
        // dropped years ago. Without this filter they ride along in every APK.
        ndk {
            abiFilters += listOf("arm64-v8a", "armeabi-v7a", "x86_64")
        }
    }

    buildFeatures {
        compose = true
        buildConfig = true
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
    implementation(libs.compose.material.icons.extended)
    implementation(libs.compose.ui.tooling.preview)
    debugImplementation(libs.compose.ui.tooling)
    implementation(libs.androidx.activity.compose)
    implementation(libs.androidx.biometric)
    implementation(libs.androidx.core.ktx)
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
