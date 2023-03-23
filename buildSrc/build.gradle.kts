/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
// import org.noelware.charted.gradle.*
import kotlin.jvm.optionals.getOrElse

plugins {
    id("com.diffplug.spotless") version "6.17.0"

    `java-gradle-plugin`
    `kotlin-dsl`

    java
}

group = "org.noelware.charted.gradle"

val libs = extensions.getByType<VersionCatalogsExtension>().named("libs")
fun VersionCatalog.get(name: String): MinimalExternalModuleDependency = libs.findLibrary(name).getOrElse { null }?.get() ?: error("Unknown library '$name' in catalog")

repositories {
    maven("https://maven.floofy.dev/repo/releases")
    maven("https://maven.noelware.org")
    gradlePluginPortal()
    mavenCentral()
    mavenLocal()
}

dependencies {
    implementation("org.jetbrains.kotlinx:atomicfu-gradle-plugin:0.20.0")
    implementation("com.diffplug.spotless:spotless-plugin-gradle:6.17.0")
    implementation("com.netflix.nebula:gradle-ospackage-plugin:11.0.0")
    implementation("com.google.protobuf:protobuf-gradle-plugin:0.9.2")
    implementation("dev.floofy.commons:gradle:2.5.1")
    implementation(kotlin("serialization", "1.8.10"))
    implementation(kotlin("gradle-plugin", "1.8.10"))
    implementation(gradleApi())

    // test dependencies
    testImplementation("uk.org.webcompere:system-stubs-jupiter:2.0.2")
    testImplementation("uk.org.webcompere:system-stubs-core:2.0.2")
    testImplementation(libs.get("junit-jupiter-engine"))
    testImplementation(libs.get("junit-jupiter-api"))
}

@Suppress("UnstableApiUsage")
gradlePlugin {
    website.set("https://charts.noelware.org")
    vcsUrl.set("https://github.com/charted-dev/charted/tree/main/buildSrc")

    plugins {
        create("nebula") {
            implementationClass = "org.noelware.charted.gradle.plugins.nebula.ChartedNebulaPlugin"
            id = "org.noelware.charted.dist.nebula"
        }
    }
}

kotlin {
    explicitApi()
    jvmToolchain {
        languageVersion.set(JavaLanguageVersion.of(17))
    }
}

spotless {
    // For Kotlin and Kotlin (Gradle), we will need to move the license header
    // as the last step due to https://github.com/diffplug/spotless/issues/1599
    kotlin {
        targetExclude("**/*.charted.kts")
        endWithNewline()
        encoding("UTF-8")
        target("**/*.kt")
        ktlint().apply {
            setUseExperimental(true)
            setEditorConfigPath(file("${rootProject.projectDir}/../.editorconfig"))
        }

        licenseHeaderFile(file("${rootProject.projectDir}/../assets/HEADING"))
    }

    kotlinGradle {
        endWithNewline()
        encoding("UTF-8")
        target("**/*.gradle.kts")
        ktlint().apply {
            setUseExperimental(true)
            setEditorConfigPath(file("${rootProject.projectDir}/../.editorconfig"))
        }

        licenseHeaderFile(file("${rootProject.projectDir}/../assets/HEADING"), "(package |@file|import |pluginManagement|plugins|rootProject.name)")
    }

    java {
        licenseHeaderFile(file("${rootProject.projectDir}/../assets/HEADING"))
        trimTrailingWhitespace()
        removeUnusedImports()
        palantirJavaFormat()
        endWithNewline()
        encoding("UTF-8")
    }
}

tasks {
    withType<KotlinCompile>().configureEach {
        kotlinOptions.jvmTarget = "17"
    }

    withType<Test>().configureEach {
        useJUnitPlatform()
        outputs.upToDateWhen { false }
        maxParallelForks = Runtime.getRuntime().availableProcessors()
        failFast = true

        testLogging {
            events(
                org.gradle.api.tasks.testing.logging.TestLogEvent.PASSED,
                org.gradle.api.tasks.testing.logging.TestLogEvent.FAILED,
                org.gradle.api.tasks.testing.logging.TestLogEvent.SKIPPED,
                org.gradle.api.tasks.testing.logging.TestLogEvent.STANDARD_ERROR,
                org.gradle.api.tasks.testing.logging.TestLogEvent.STANDARD_OUT,
                org.gradle.api.tasks.testing.logging.TestLogEvent.STARTED,
            )

            showCauses = true
            showExceptions = true
            exceptionFormat = org.gradle.api.tasks.testing.logging.TestExceptionFormat.FULL
        }
    }
}

configurations.configureEach {
    if (isCanBeResolved) {
        attributes {
            attribute(GradlePluginApiVersion.GRADLE_PLUGIN_API_VERSION_ATTRIBUTE, project.objects.named(GradleVersion.current().version))
        }
    }
}
