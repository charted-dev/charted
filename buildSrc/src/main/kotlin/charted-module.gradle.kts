/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

import org.gradle.api.tasks.testing.logging.TestExceptionFormat
import org.gradle.api.tasks.testing.logging.TestLogEvent
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import org.noelware.charted.gradle.*
import dev.floofy.utils.gradle.*

plugins {
    kotlin("plugin.serialization")
    id("com.diffplug.spotless")
    id("kotlinx-atomicfu")
    kotlin("jvm")
}

// https://github.com/Kotlin/kotlinx-atomicfu/issues/210
atomicfu {
    val libs = extensions.getByType<VersionCatalogsExtension>().named("libs")
    dependenciesVersion = libs.findVersion("kotlinx-atomicfu").get().requiredVersion
}

group = "org.noelware.charted"
version = "$VERSION"
description = ""

repositories {
    maven("https://repo.perfectdreams.net/")
    mavenCentral()
    mavenLocal()
    noelware()
    noel()
}

dependencies {
    implementation(kotlin("reflect"))
    implementation(kotlin("stdlib"))

    // test dependencies :quantD:
    testImplementation("org.junit.jupiter:junit-jupiter-api:5.9.1")
    testRuntimeOnly("org.junit.jupiter:junit-jupiter-engine:5.9.1")
    testImplementation("org.testcontainers:testcontainers:1.17.5")
    testImplementation("org.testcontainers:junit-jupiter:1.17.6")
    testImplementation("org.slf4j:slf4j-simple:2.0.6")
    testImplementation(kotlin("test"))

    if (name != "common") {
        implementation(project(":common"))
        if (path != ":modules:config:dsl") {
            implementation(project(":modules:config:dsl"))
        }
    }
}

spotless {
    kotlin {
        licenseHeaderFile("${rootProject.projectDir}/assets/HEADING")
        trimTrailingWhitespace()
        endWithNewline()

        // it's for testing purposes, spotless is just a bork in chat.
        targetExclude("**/*.charted.kts")

        // We can't use the .editorconfig file, so we'll have to specify it here
        // issue: https://github.com/diffplug/spotless/issues/142
        ktlint()
            .setUseExperimental(true)
            .editorConfigOverride(mapOf(
                "indent_size" to "4",

                // Using `ktlint_disabled_rules` doesn't work for some reason! We need to check why
                // though, which is tricky. :(
                "disabled_rules" to "colon-spacing,annotation-spacing,filename,no-wildcard-imports,argument-list-wrapping",
                "ij_kotlin_allow_trailing_comma" to "false",
                "ktlint_code_style" to "official",
                "no-unused-imports" to "true",
                "no-unit-return" to "true",
                "no-consecutive-blank-lines" to "true",
                "experimental:fun-keyword-spacing" to "true",
                "experimental:unnecessary-parentheses-before-trailing-lambda" to "true"
            ))
    }
}

java {
    sourceCompatibility = JAVA_VERSION
    targetCompatibility = JAVA_VERSION
}

// This will transform the project path:
//
//    - :sessions -> sessions
//    - :modules:elasticsearch -> elasticsearch
//    - :modules:tracing:apm -> tracing-apm
//    - :sessions:integrations:noelware -> sessions-integrations-noelware
val projectName = path
    .substring(1)
    .replace(':', '-')
    .replace("modules-", "")

tasks {
    withType<Jar> {
        archiveFileName by "charted-$projectName-$VERSION.jar"
        manifest {
            attributes(
                mapOf(
                    "Implementation-Version" to "$VERSION",
                    "Implementation-Vendor" to "Noelware, LLC. [team@noelware.org]",
                    "Implementation-Title" to "charted-server"
                )
            )
        }
    }

    withType<KotlinCompile> {
        kotlinOptions.freeCompilerArgs += "-opt-in=kotlin.RequiresOptIn"
        kotlinOptions.javaParameters = true
        kotlinOptions.jvmTarget = JAVA_VERSION.majorVersion
    }

    withType<Test>().configureEach {
        useJUnitPlatform()
        outputs.upToDateWhen { false }
        maxParallelForks = Runtime.getRuntime().availableProcessors()
        failFast = true // kill gradle if a test fails

        testLogging {
            events.addAll(listOf(TestLogEvent.PASSED, TestLogEvent.FAILED, TestLogEvent.SKIPPED))
            showStandardStreams = true
            exceptionFormat = TestExceptionFormat.FULL
        }
    }
}
