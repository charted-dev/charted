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

import dev.floofy.utils.gradle.*
import org.gradle.api.tasks.testing.logging.TestExceptionFormat
import org.gradle.api.tasks.testing.logging.TestLogEvent
import org.noelware.charted.gradle.*
import kotlin.jvm.optionals.getOrNull

plugins {
    kotlin("plugin.serialization")
    id("com.diffplug.spotless")
    id("kotlinx-atomicfu")
    kotlin("jvm")

    java
}

group = "org.noelware.charted"
version = "$VERSION"
description = "\uD83D\uDCE6 You know, for Helm Charts?"

val libs = extensions.getByType<VersionCatalogsExtension>().named("libs")
fun VersionCatalog.version(name: String): String = libs.findVersion(name).getOrNull()?.requiredVersion ?: error("Unknown version for catalog '$name' exists!")
fun VersionCatalog.get(name: String): MinimalExternalModuleDependency = libs.findLibrary(name).getOrNull()?.get() ?: error("Unknown library '$name' in catalog")

// https://github.com/Kotlin/kotlinx-atomicfu/issues/210
atomicfu {
    dependenciesVersion = libs.version("kotlinx-atomicfu")
}

repositories {
    maven("https://repo.perfectdreams.net/")
    mavenCentral()
    mavenLocal()
    noelware()
    noel()
}

dependencies {
    // Kotlin libraries
    implementation(kotlin("reflect"))
    implementation(kotlin("stdlib"))

    // Java Annotations (only for Java usage)
    implementation(libs.get("jetbrains-annotations"))

    // Test Dependencies
    testImplementation(libs.get("testcontainers-junit"))
    testImplementation(libs.get("junit-jupiter-engine"))
    testImplementation(libs.get("testcontainers-core"))
    testImplementation(project(":testing:containers"))
    testImplementation(libs.get("junit-jupiter-api"))
    testImplementation(libs.get("slf4j-simple"))
    testImplementation(kotlin("test"))

    // Add the `:common` module to all projects that aren't :common
    if (name != "common") {
        implementation(project(":common"))

        // Include the configuration DSL if we aren't :common OR :config:dsl
        if (path != ":config:dsl") {
            implementation(project(":config:dsl"))
        }
    }

    // Add common libraries that are used through out all projects
    // kotlinx.serialization
    api(libs.get("kotlinx-serialization-core"))
    api(libs.get("kotlinx-serialization-json"))

    // kotlinx.coroutines
    api(libs.get("kotlinx-coroutines-core"))
    api(libs.get("kotlinx-coroutines-jdk8"))

    // kotlinx.datatime
    api(libs.get("kotlinx-datetime"))

    // SLF4J
    api(libs.get("slf4j-api"))

    // Noel's Utilities
    api(libs.get("noel-commons-extensions-kotlin"))
    api(libs.get("noel-commons-extensions-koin"))
    api(libs.get("noel-commons-java-utils"))
    api(libs.get("noel-commons-slf4j"))

    // Apache Utilities
    api(libs.get("apache-commons-lang3"))

    // Sentry
    api(libs.get("sentry-kotlin-extensions"))
    api(libs.get("sentry"))

    // Bouncycastle
    api(libs.get("bouncycastle"))

    // Snowflake
    api(libs.get("snowflake"))

    // Jackson
    api(libs.get("jackson-databind"))
}

applySpotless()

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
                    "Implementation-Title" to "charted-server",
                ),
            )
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

    withType<JavaCompile>().configureEach {
        options.isIncremental = true
        options.encoding = "UTF-8"
        options.isFork = true
    }
}
