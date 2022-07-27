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
    // BOM
    api(platform("org.jetbrains.kotlinx:kotlinx-serialization-bom:1.3.3"))
    api(platform("org.jetbrains.kotlinx:kotlinx-coroutines-bom:1.6.3"))
    api(platform("org.jetbrains.exposed:exposed-bom:0.38.2"))
    api(platform("dev.floofy.commons:commons-bom:2.2.1.1"))
    api(platform("org.noelware.remi:remi-bom:0.3.2-beta"))
    api(platform("io.ktor:ktor-bom:2.0.3"))

    // kotlinx.serialization
    api("org.jetbrains.kotlinx:kotlinx-serialization-json")
    api("org.jetbrains.kotlinx:kotlinx-serialization-core")

    // kotlinx.coroutines
    api("org.jetbrains.kotlinx:kotlinx-coroutines-core")
    api("org.jetbrains.kotlinx:kotlinx-coroutines-jdk8")

    // kotlinx.datetime
    api("org.jetbrains.kotlinx:kotlinx-datetime:0.4.0")

    // Logging (slf4j)
    api("org.slf4j:slf4j-api:1.7.36")

    // Database drivers + Exposed + HikariCP
    api("org.jetbrains.exposed:exposed-jdbc")
    api("org.jetbrains.exposed:exposed-core")
    api("org.jetbrains.exposed:exposed-dao")
    api("org.postgresql:postgresql:42.3.6")
    api("com.zaxxer:HikariCP:5.0.1")

    // Redis
    api("io.lettuce:lettuce-core:6.1.8.RELEASE")

    // Noel's Utilities
    api("dev.floofy.commons:extensions-kotlin")
    api("dev.floofy.commons:extensions-koin")
    api("dev.floofy.commons:exposed")
    api("dev.floofy.commons:slf4j")

    // Apache Utilities
    api("org.apache.commons:commons-lang3:3.12.0")

    // Remi (storage management)
    api("org.noelware.remi:remi-support-minio")
    api("org.noelware.remi:remi-support-s3")
    api("org.noelware.remi:remi-support-fs")
    api("org.noelware.remi:remi-core")

    // Sentry
    api("io.sentry:sentry-kotlin-extensions:6.1.0")
    api("io.sentry:sentry:6.1.0")

    // Koin
    api("io.insert-koin:koin-core:3.2.0")

    // YAML (configuration)
    api("com.charleskorn.kaml:kaml:0.46.0")

    // Haru (scheduling)
    api("dev.floofy.haru:Haru:1.3.0")

    // Ktor Server + Client
    api("com.squareup.okhttp3:okhttp:4.10.0")
    api("io.ktor:ktor-server-core")
    api("io.ktor:ktor-client-core")

    // Exposed Power Utils
    api("net.perfectdreams.exposedpowerutils:postgres-power-utils:1.0.0")
}
