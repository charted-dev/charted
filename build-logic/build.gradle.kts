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

import org.gradle.api.tasks.testing.logging.TestLogEvent
import org.gradle.api.tasks.testing.logging.TestExceptionFormat
import org.jetbrains.kotlin.gradle.dsl.JvmTarget
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import kotlin.jvm.optionals.getOrNull

plugins {
    id("com.diffplug.spotless") version "6.18.0"
    `java-gradle-plugin`
    `kotlin-dsl`
    java
}

group = "org.noelware.charted.gradle"
version = "0.0.0-devel.0"

val libs: VersionCatalog = extensions.getByType<VersionCatalogsExtension>().named("libs")
fun get(name: String): MinimalExternalModuleDependency =
    libs.findLibrary(name).getOrNull()?.get() ?: error("Unknown library '$name' in catalog")

repositories {
    maven("https://maven.floofy.dev/repo/releases")
    maven("https://maven.noelware.org")
    gradlePluginPortal()
    mavenCentral()
    mavenLocal()
}

dependencies {
    implementation("org.jetbrains.kotlinx:atomicfu-gradle-plugin:0.20.2")
    implementation("com.diffplug.spotless:spotless-plugin-gradle:6.19.0")
    implementation("com.netflix.nebula:gradle-ospackage-plugin:11.3.0")
    implementation("com.google.protobuf:protobuf-gradle-plugin:0.9.3")
    implementation("dev.floofy.commons:gradle:2.5.1")
    implementation(kotlin("serialization", "1.8.22"))
    implementation(kotlin("gradle-plugin", "1.8.22"))
    implementation(gradleApi())

    // test dependencies
    testImplementation(get("system-stubs-jupiter"))
    testImplementation(get("junit-jupiter-engine"))
    testImplementation(get("junit-jupiter-api"))
    testImplementation(get("system-stubs-core"))
    testImplementation(get("assertj"))
}

@Suppress("UnstableApiUsage")
gradlePlugin {
    vcsUrl.set("https://github.com/charted-dev/charted/tree/main/build-logic")
    website.set("https://charts.noelware.org")

    plugins {
        create("nebula") {
            implementationClass = "org.noelware.charted.gradle.plugins.nebula.ChartedNebulaPlugin"
            id = "org.noelware.charted.dist.nebula"
        }

        create("restIntegTest") {
            implementationClass = "org.noelware.charted.gradle.plugins.restIntegTest.RestIntegTestPlugin"
            id = "org.noelware.charted.testing.restIntegTest"
        }
    }
}

kotlin {
    jvmToolchain {
        languageVersion.set(JavaLanguageVersion.of(17))
    }
}

spotless {
    // For Kotlin and Kotlin (Gradle), we will need to move the license header
    // as the last step due to https://github.com/diffplug/spotless/issues/1599
    kotlin {
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
        compilerOptions {
            jvmTarget.set(JvmTarget.JVM_17)
        }
    }

    withType<Test>().configureEach {
        useJUnitPlatform()
        outputs.upToDateWhen { false }
        maxParallelForks = Runtime.getRuntime().availableProcessors()
        failFast = true

        testLogging {
            events(
                TestLogEvent.PASSED,
                TestLogEvent.FAILED,
                TestLogEvent.SKIPPED,
                TestLogEvent.STANDARD_ERROR,
                TestLogEvent.STANDARD_OUT,
                TestLogEvent.STARTED,
            )

            showCauses = true
            showExceptions = true
            exceptionFormat = TestExceptionFormat.FULL
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
