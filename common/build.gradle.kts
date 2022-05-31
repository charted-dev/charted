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

plugins {
    `charted-module`
}

dependencies {
    // Kotlin libraries
    api(kotlin("reflect"))
    api(kotlin("stdlib"))

    // BOMs
    api(platform("org.jetbrains.kotlinx:kotlinx-serialization-bom:1.3.2"))
    api(platform("org.jetbrains.kotlinx:kotlinx-coroutines-bom:1.6.2"))
    api(platform("org.jetbrains.exposed:exposed-bom:0.38.2"))
    api(platform("dev.floofy.commons:commons-bom:2.1.0.1"))
    api(platform("org.noelware.remi:remi-bom:0.1.5-beta"))
    api(platform("io.ktor:ktor-bom:2.0.1"))

    // kotlinx.coroutines
    api("org.jetbrains.kotlinx:kotlinx-coroutines-jdk8")
    api("org.jetbrains.kotlinx:kotlinx-coroutines-core")

    // kotlinx.serialization
    api("org.jetbrains.kotlinx:kotlinx-serialization-json")
    api("org.jetbrains.kotlinx:kotlinx-serialization-core")

    // kotlinx.datetime
    api("org.jetbrains.kotlinx:kotlinx-datetime:0.3.3")

    // Noel's Utilities
    api("dev.floofy.commons:extensions-kotlin")
    api("dev.floofy.commons:extensions-koin")
    api("dev.floofy.commons:exposed")
    api("dev.floofy.commons:slf4j")

    // Exposed (PSQL)
    api("org.jetbrains.exposed:exposed-jdbc")
    api("org.jetbrains.exposed:exposed-core")
    api("org.jetbrains.exposed:exposed-dao")

    // PostgreSQL Driver
    api("org.postgresql:postgresql:42.3.4")

    // ClickHouse Driver (for audit logs, webhook events)
    api("com.clickhouse:clickhouse-jdbc:0.3.2-patch9")

    // Connection pooling (for PostgreSQL)
    api("com.zaxxer:HikariCP:5.0.1")

    // Apache Utilities!
    api("org.apache.commons:commons-lang3:3.12.0")

    // Koin
    api("io.insert-koin:koin-core:3.2.0")

    // SLF4J
    api("org.slf4j:slf4j-api:1.7.36")

    // Sentry
    api("io.sentry:sentry-kotlin-extensions:5.7.3")
    api("io.sentry:sentry:5.7.3")

    // Redis (Lettuce)
    api("io.lettuce:lettuce-core:6.1.8.RELEASE")

    // Remi
    api("org.noelware.remi:remi-support-minio")
    api("org.noelware.remi:remi-support-s3")
    api("org.noelware.remi:remi-support-fs")
    api("org.noelware.remi:remi-core")
}
