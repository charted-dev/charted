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
    `charted-java-module`
    `charted-module`
    `charted-test`
}

dependencies {
    api(platform("org.jetbrains.kotlinx:kotlinx-serialization-bom:1.3.3"))
    api(platform("org.jetbrains.kotlinx:kotlinx-coroutines-bom:1.6.3"))
    api(platform("org.jetbrains.exposed:exposed-bom:0.38.2"))
    api(platform("org.noelware.remi:remi-bom:0.3.2-beta"))
    api(platform("dev.floofy.commons:commons-bom:2.2.1"))
    api(platform("io.ktor:ktor-bom:2.0.3"))

    api("org.jetbrains.kotlinx:kotlinx-serialization-json")
    api("org.jetbrains.kotlinx:kotlinx-serialization-core")
    api("org.jetbrains.kotlinx:kotlinx-coroutines-core")
    api("org.jetbrains.kotlinx:kotlinx-coroutines-jdk8")
    api("org.jetbrains.kotlinx:kotlinx-datetime:0.3.3")
    api("com.clickhouse:clickhouse-jdbc:0.3.2-patch9")
    api("io.sentry:sentry-kotlin-extensions:6.1.0")
    api("org.apache.commons:commons-lang3:3.12.0")
    api("io.lettuce:lettuce-core:6.1.8.RELEASE")
    api("dev.floofy.commons:extensions-kotlin")
    api("org.noelware.remi:remi-support-minio")
    api("dev.floofy.commons:extensions-koin")
    api("org.jetbrains.exposed:exposed-jdbc")
    api("org.jetbrains.exposed:exposed-core")
    api("org.noelware.remi:remi-support-s3")
    api("org.noelware.remi:remi-support-fs")
    api("org.jetbrains.exposed:exposed-dao")
    api("org.postgresql:postgresql:42.3.6")
    api("com.google.guava:guava:31.1-jre")
    api("io.insert-koin:koin-core:3.2.0")
    api("org.noelware.remi:remi-core")
    api("dev.floofy.commons:exposed")
    api("org.slf4j:slf4j-api:1.7.36")
    api("com.zaxxer:HikariCP:5.0.1")
    api("dev.floofy.commons:slf4j")
    api("io.sentry:sentry:6.1.0")
}
