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
import org.gradle.api.artifacts.VersionCatalog
import org.gradle.api.tasks.bundling.Jar
import org.gradle.api.tasks.compile.JavaCompile
import org.gradle.api.tasks.testing.Test
import org.gradle.api.tasks.testing.logging.TestExceptionFormat
import org.gradle.api.tasks.testing.logging.TestLogEvent
import org.gradle.kotlin.dsl.*
import org.jetbrains.kotlin.gradle.dsl.JvmTarget
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import org.noelware.charted.gradle.*

import java.text.SimpleDateFormat
import java.util.Date

plugins {
    kotlin("plugin.serialization")
    id("com.diffplug.spotless")
    id("kotlinx-atomicfu")
    kotlin("jvm")

    java
    idea
}

group = "org.noelware.charted"
version = "$VERSION"
description = "\uD83D\uDCE6 You know, for Helm Charts?"

public val libs: VersionCatalog = extensions.getByType<VersionCatalogsExtension>().named("libs")

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
    testImplementation(libs.get("assertj"))
    testImplementation(kotlin("test"))

    // Make sure the runtime is available so that the server doesn't crash whenever
    // if 'kotlinx/atomicfu/AtomicFU' was not loaded.
    //
    // The plugin source declares the JVM dependency as `implementation` (https://github.com/Kotlin/kotlinx-atomicfu/blob/master/atomicfu-gradle-plugin/src/main/kotlin/kotlinx/atomicfu/plugin/gradle/AtomicFUGradlePlugin.kt#L50-L56)
    // and implementation dependency configurations are only scoped to that project and wasn't able to be accessed,
    // so `api` is used for the dependency instead of `implementation`, which is global to add
    // modules that implement it but in our case, it's loaded into every module anyway so...
    api(libs.get("kotlinx-atomicfu-jvm"))

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

    // Swagger Annotations
    api(libs.get("swagger-annotations"))
}

applySpotless()

// We also enable testing on the new K2 compiler that we do tests
// on, or just in case to preview builds on
val enableK2Compiler = System.getProperty("org.noelware.charted.k2-compiler", "false")
    .let { it matches "^(yes|true|1|si*)$".toRegex() }

kotlin {
    if (enableK2Compiler) {
        logger.info("Detected `org.noelware.charted.k2-compiler` system property, enabling it in this source-set")
        sourceSets.all {
            languageSettings {
                languageVersion = "2.0"
            }
        }
    }
}

java {
    toolchain {
        languageVersion by JavaLanguageVersion.of(JAVA_VERSION.majorVersion)
    }
}

// This will transform the project path:
//
//    - :sessions -> sessions
//    - :modules:elasticsearch -> elasticsearch
//    - :modules:tracing:apm -> tracing-apm
//    - :sessions:integrations:noelware -> sessions-integrations-noelware
val projectName: String = path
    .substring(1)
    .replace(':', '-')
    .replace("modules-", "")

val RFC3339Formatter = SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss'Z'")
tasks {
    withType<KotlinCompile>().configureEach {
        compilerOptions {
            javaParameters by true
            jvmTarget by JvmTarget.fromTarget(JAVA_VERSION.majorVersion)
        }
    }

    withType<Jar> {
        archiveFileName by "charted-$projectName-$VERSION.jar"
        manifest {
            attributes(
                mapOf(
                    "Implementation-Build-Date" to RFC3339Formatter.format(Date()),
                    "Implementation-Version" to "$VERSION",
                    "Implementation-Vendor" to "Noelware, LLC.",
                    "Implementation-Title" to projectName,
                    "Created-By" to GradleVersion.current(),
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

    // an "attempt" to fix:
    // * What went wrong:
    // Execution failed for task ':common:jar'.
    // > Entry META-INF/common.kotlin_module is a duplicate but no duplicate handling strategy has been set.
    //   Please refer to https://docs.gradle.org/8.0.2/dsl/org.gradle.api.tasks.Copy.html#org.gradle.api.tasks.Copy:duplicatesStrategy
    //   for details.
    jar {
        duplicatesStrategy = DuplicatesStrategy.INCLUDE
    }
}
