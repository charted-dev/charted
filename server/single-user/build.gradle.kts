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

plugins {
    id("charted-module")
}

dependencies {
    // Test dependencies
    testImplementation(libs.ktor.client.content.negotitation)
    testImplementation(libs.ktor.serialization.kotlinx.json)
    testImplementation(libs.ktor.server.test.host)
    testImplementation(libs.ktor.serialization)
    testImplementation(libs.ktor.client.java)
    testImplementation(libs.ktor.server.core)

    // Modules that we require (which is... most of them!)
    implementation(projects.modules.sessions.integrations.github)
    implementation(projects.modules.search.elasticsearch)
    implementation(projects.modules.sessions.local)
    implementation(projects.modules.sessions.ldap)
    implementation(projects.config.kotlinScript)
    implementation(projects.modules.helmCharts)
    implementation(projects.modules.postgresql)
    implementation(projects.modules.sessions)
    implementation(projects.modules.tracing)
    implementation(projects.modules.logging)
    implementation(projects.modules.openapi)
    implementation(projects.modules.storage)
    implementation(projects.modules.avatars)
    implementation(projects.modules.metrics)
    implementation(projects.modules.search)
    implementation(projects.modules.emails)
    implementation(projects.modules.redis)
    implementation(projects.config.yaml)
    implementation(projects.config)

    // kotlinx.coroutines debug
    implementation(libs.kotlinx.coroutines.debug)

    // Ktor (Server)
    implementation(libs.ktor.client.content.negotitation)
    implementation(libs.ktor.serialization.kotlinx.json)
    implementation(libs.ktor.server.content.negotiation)
    implementation(libs.ktor.server.auto.head.response)
    implementation(libs.ktor.server.default.headers)
    implementation(libs.ktor.server.caching.headers)
    implementation(libs.ktor.server.double.receive)
    implementation(libs.ktor.server.status.pages)
    implementation(libs.ktor.server.ratelimiting)
    implementation(libs.ktor.serialization)
    implementation(libs.ktor.server.netty)
    implementation(libs.ktor.server.cors)
    implementation(libs.ktor.server.core)
    implementation(libs.ktor.client.java)

    // Just for Log4j/JCL/JUL -> slf4j
    implementation(libs.slf4j.over.log4j)
    implementation(libs.slf4j.over.jcl)
    implementation(libs.slf4j.from.jul)

    // Janino (for Logback)
    implementation(libs.janino)

    // YAML
    implementation(libs.kaml)

    // Spring Security Crypto
    implementation(libs.spring.security.crypto)

    // Apache Commons Validator
    implementation(libs.apache.commons.validator)

    // Remi (fs)
    implementation(libs.remi.storage.fs)
    implementation(libs.remi.core)

    // JWT
    implementation(libs.jwt)

    // Caffeine
    implementation(libs.caffeine)
}
