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
    `charted-test`
}

dependencies {
    // Ktor
    implementation("io.ktor:ktor-serialization-kotlinx-json")
    implementation("io.ktor:ktor-client-content-negotiation")
    implementation("io.ktor:ktor-serialization")
    implementation("io.ktor:ktor-client-okhttp")
    api("com.squareup.okhttp3:okhttp:4.10.0")
    api("io.ktor:ktor-server-core")
    api("io.ktor:ktor-client-core")

    // Ktor Routing
    implementation("org.noelware.ktor:core:0.3.1-beta")

    // Haru (scheduling)
    implementation("dev.floofy.haru:Haru:1.3.0")

    // JWT
    implementation("com.auth0:java-jwt:3.19.2")

    // Logback (for json logback formatter)
    implementation("com.fasterxml.jackson.core:jackson-databind:2.13.3")
    implementation("ch.qos.logback.contrib:logback-json-classic:0.1.5")
    implementation("ch.qos.logback.contrib:logback-jackson:0.1.5")

    // Projects
    implementation(project(":database"))

    // Apache Commons Validator
    api("commons-validator:commons-validator:1.7")

    // Spring Security Crypto
    api("org.springframework.security:spring-security-crypto:5.7.1")
}
