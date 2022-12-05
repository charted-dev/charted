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
    `charted-module`
}

dependencies {
    // Projects required to run the server :quantD:
    implementation(project(":databases:clickhouse"))
    implementation(project(":databases:postgres"))
    implementation(project(":modules:audit-logs"))
    implementation(project(":modules:apikeys"))
    implementation(project(":modules:avatars"))
    implementation(project(":modules:config:kotlin-script"))
    implementation(project(":modules:config:yaml"))
    implementation(project(":modules:config"))
    implementation(project(":modules:docker-registry"))
    implementation(project(":modules:elasticsearch"))
    implementation(project(":modules:emails"))
    implementation(project(":modules:helm-charts"))
    implementation(project(":modules:invitations"))
    implementation(project(":modules:logging"))
    implementation(project(":modules:meilisearch"))
    implementation(project(":modules:metrics"))
    implementation(project(":modules:redis"))
    implementation(project(":modules:sessions:integrations:github"))
    implementation(project(":modules:sessions:integrations"))
    implementation(project(":modules:sessions:ldap"))
    implementation(project(":modules:sessions:local"))
    implementation(project(":modules:sessions"))
    implementation(project(":modules:storage"))
    implementation(project(":modules:telemetry"))

    // kotlinx.coroutines debug
    implementation(libs.kotlinx.coroutines.debug)

    // Ktor Routing
    implementation(libs.noelware.ktor.routing.loaders.koin)
    implementation(libs.noelware.ktor.routing.core)

    // HikariCP (for database)
    implementation(libs.hikaricp)

    // Spring Security Crypto
    implementation(libs.spring.security.crypto)

    // Ktor (Server)
    implementation(libs.ktor.client.content.negotitation)
    implementation(libs.ktor.serialization.kotlinx.json)
    implementation(libs.ktor.server.content.negotiation)
    implementation(libs.ktor.server.auto.head.response)
    implementation(libs.ktor.server.default.headers)
    implementation(libs.ktor.server.double.receive)
    implementation(libs.ktor.server.status.pages)
    implementation(libs.ktor.server.websockets)
    implementation(libs.ktor.serialization)
    implementation(libs.ktor.client.okhttp)
    implementation(libs.ktor.server.netty)
    implementation(libs.ktor.server.cors)

    // Ktor Testing
    testImplementation("io.ktor:ktor-server-test-host:2.1.3")

    // JWT
    implementation(libs.jwt)

    // Just for Log4j/JCL -> slf4j
    implementation(libs.slf4j.over.log4j)
    implementation(libs.slf4j.over.jcl)

    // Sentry~!
    implementation(libs.sentry.kotlin.extensions)

    // Tegral OpenAPI
    implementation(libs.tegral.openapi)

    // Janino (for logback)
    implementation(libs.janino)

    // Elastic APM
    implementation(libs.elastic.apm.agent.attach)

    // Apache Commons Validator
    implementation(libs.apache.commons.validator)

    // SemVer
    implementation(libs.semver)

    // Noelware Analytics
    implementation(libs.noelware.analytics.server.kotlin.support)
    implementation(libs.noelware.analytics.server)
    implementation(libs.grpc.services)
}
