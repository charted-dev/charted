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
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

plugins {
    `charted-java-module`
    `charted-module`
    `charted-test`
}

dependencies {
    api(platform("org.jetbrains.kotlinx:kotlinx-serialization-bom:1.3.3"))
    api(platform("org.jetbrains.kotlinx:kotlinx-coroutines-bom:1.6.3"))
    api(platform("org.jetbrains.exposed:exposed-bom:0.38.2"))
    api(platform("dev.floofy.commons:commons-bom:2.2.1.1"))
    api(platform("org.noelware.remi:remi-bom:0.4.1-beta"))
    api(platform("io.ktor:ktor-bom:2.0.3"))

    // kotlinx.serialization
    api(libs.kotlinx.serialization.core)
    api(libs.kotlinx.serialization.json)

    // kotlinx.coroutines
    api(libs.kotlinx.coroutines.core)
    api(libs.kotlinx.coroutines.jdk8)

    // kotlinx.datetime
    api(libs.kotlinx.datetime)

    // Logging (slf4j)
    api(libs.slf4j.api)

    // Database drivers + Exposed + HikariCP
    api(libs.exposed.jdbc)
    api(libs.exposed.core)
    api(libs.exposed.dao)
    api(libs.postgresql)
    api(libs.hikaricp)

    // Redis
    api(libs.lettuce)

    // Noel's Utilities
    api(libs.noel.commons.extensions.kotlin)
    api(libs.noel.commons.extensions.koin)
    api(libs.noel.commons.exposed)
    api(libs.noel.commons.slf4j)

    // Apache Utilities
    api(libs.apache.commons.lang3)

    // Remi (storage management)
    api(libs.remi.support.minio)
    api(libs.remi.support.s3)
    api(libs.remi.support.fs)
    api(libs.remi.core)

    // Sentry
    api(libs.sentry.kotlin.extensions)
    api(libs.sentry)

    // Koin
    api(libs.koin)

    // YAML (configuration)
    api(libs.kaml)

    // Haru (scheduling)
    api(libs.haru)

    // Ktor Server + Client
    api(libs.ktor.server.core)
    api(libs.ktor.client.core)
    api(libs.okhttp)

    // Exposed Power Utils
    api(libs.exposed.powergamer.tools)

    // Apache Commons Validator
    api(libs.apache.commons.validator)

    // Spring Security Crypto
    api(libs.spring.security.crypto)

    // SemVer validation
    api(libs.semver)
}
