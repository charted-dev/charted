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

plugins {
    id("com.diffplug.spotless") version "6.16.0"
    kotlin("jvm") version "1.8.10"

    `java-gradle-plugin`
    `kotlin-dsl`

    java
}

repositories {
    maven("https://maven.floofy.dev/repo/releases")
    gradlePluginPortal()
    mavenCentral()
}

dependencies {
    implementation("com.diffplug.spotless:spotless-plugin-gradle:6.15.0")
    implementation("com.netflix.nebula:gradle-ospackage-plugin:11.0.0")
    implementation(gradleApi())
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

@Suppress("UnstableApiUsage")
gradlePlugin {
    website.set("https://charts.noelware.org")
    vcsUrl.set("https://github.com/charted-dev/charted/tree/main/build-tools")

    plugins {
        create("nebula") {
            implementationClass = "org.noelware.charted.gradle.plugins.nebula.ChartedNebulaPlugin"
            id = "org.noelware.charted.dist.nebula"
        }
    }
}

kotlin {
    jvmToolchain {
        languageVersion.set(JavaLanguageVersion.of(17))
    }
}

tasks.withType<KotlinCompile>().configureEach {
    kotlinOptions.jvmTarget = "17"
}
