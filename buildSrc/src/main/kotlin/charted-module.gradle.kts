/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022 Noelware <team@noelware.org>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *    http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import org.noelware.charted.gradle.*
import dev.floofy.utils.gradle.*

plugins {
    kotlin("plugin.serialization")
    id("com.diffplug.spotless")
    kotlin("jvm")
}

apply(plugin = "kotlinx-atomicfu")

group = "org.noelware.charted"
version = "$VERSION"

repositories {
    maven("https://repo.perfectdreams.net/")
    mavenCentral()
    mavenLocal()
    noelware()
    noel()
}

dependencies {
    // Kotlin
    implementation(kotlin("stdlib"))
    implementation(kotlin("reflect"))

    // kotlinx.coroutines
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-jdk8:1.6.1")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.6.1")

    // kotlinx.serialization
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.3.2")
    api("org.jetbrains.kotlinx:kotlinx-serialization-core:1.3.2")

    // kotlinx.datetime libraries
    implementation("org.jetbrains.kotlinx:kotlinx-datetime:0.3.2")

    // Noel Utilities
    implementation("dev.floofy.commons:extensions-kotlin:2.1.0.1")
    implementation("dev.floofy.commons:extensions-koin:2.1.0.1")
    implementation("dev.floofy.commons:exposed:2.1.0.1")
    implementation("dev.floofy.commons:slf4j:2.1.0.1")

    // Exposed
    api("org.jetbrains.exposed:exposed-core:0.38.2")
    api("org.jetbrains.exposed:exposed-jdbc:0.38.2")
    api("org.jetbrains.exposed:exposed-dao:0.38.2")

    // PostgreSQL driver
    api("org.postgresql:postgresql:42.3.4")

    // Connection pooling
    api("com.zaxxer:HikariCP:5.0.1")

    // Apache Utilities
    implementation("org.apache.commons:commons-lang3:3.12.0")

    // Koin
    implementation("io.insert-koin:koin-core:3.1.6")

    // SLF4J
    api("org.slf4j:slf4j-api:1.7.36")

    // Sentry
    implementation("io.sentry:sentry:5.7.3")
    implementation("io.sentry:sentry-kotlin-extensions:5.7.3")

    // Redis (Lettuce)
    api("io.lettuce:lettuce-core:6.1.8.RELEASE")
}

spotless {
    kotlin {
        trimTrailingWhitespace()
        licenseHeaderFile("${rootProject.projectDir}/assets/HEADING")
        endWithNewline()

        // We can't use the .editorconfig file, so we'll have to specify it here
        // issue: https://github.com/diffplug/spotless/issues/142
        // ktlint 0.35.0 (default for Spotless) doesn't support trailing commas
        ktlint("0.43.0")
            .userData(
                mapOf(
                    "no-consecutive-blank-lines" to "true",
                    "no-unit-return" to "true",
                    "disabled_rules" to "no-wildcard-imports,colon-spacing",
                    "indent_size" to "4"
                )
            )
    }
}

java {
    sourceCompatibility = JAVA_VERSION
    targetCompatibility = JAVA_VERSION
}

tasks.withType<Jar> {
    manifest {
        attributes(
            "Implementation-Title" to "charted-server",
            "Implementation-Version" to "$VERSION",
            "Implementation-Vendor" to "Noelware, Inc. <team@noelware.org>"
        )
    }
}

tasks {
    compileKotlin {
        kotlinOptions.jvmTarget = JAVA_VERSION.toString()
        kotlinOptions.javaParameters = true
        kotlinOptions.freeCompilerArgs += "-opt-in=kotlin.RequiresOptIn"
    }
}
