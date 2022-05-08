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
    // Ktor Server
    implementation("io.ktor:ktor-serialization-kotlinx-json:2.0.1")
    implementation("io.ktor:ktor-server-content-negotiation:2.0.1")
    implementation("io.ktor:ktor-server-auto-head-response:2.0.1")
    implementation("io.ktor:ktor-server-default-headers:2.0.1")
    implementation("io.ktor:ktor-server-status-pages:2.0.1")
    implementation("io.ktor:ktor-serialization:2.0.1")
    implementation("io.ktor:ktor-server-netty:2.0.1")
    implementation("io.ktor:ktor-server-cors:2.0.1")
    api("io.ktor:ktor-server-core:2.0.1")

    // Ktor (client)
    implementation("io.ktor:ktor-client-content-negotiation:2.0.1")
    implementation("io.ktor:ktor-client-okhttp:2.0.1")
    api("com.squareup.okhttp:okhttp:2.7.5")

    // Prometheus (for metrics)
    implementation("io.prometheus:simpleclient_hotspot:0.15.0")
    implementation("io.prometheus:simpleclient_common:0.15.0")
    implementation("io.prometheus:simpleclient:0.15.0")

    // Remi
    implementation("org.noelware.remi:remi-support-minio:0.1.4-beta.3")
    implementation("org.noelware.remi:remi-support-s3:0.1.4-beta.3")
    implementation("org.noelware.remi:remi-support-fs:0.1.4-beta.3")
    api("org.noelware.remi:remi-core:0.1.4-beta.3")

    // Subprojects
    implementation(project(":search:elastic"))
    implementation(project(":search:meili"))
    implementation(project(":analytics"))

    // Spring (daemon server)
    api("org.springframework.boot:spring-boot:2.6.7")

    // JWT
    implementation("com.auth0:java-jwt:3.19.2")

    // Ratelimit
    implementation("app.softwork:ratelimit:0.2.1")

    // Ktor Routing
    implementation("org.noelware.ktor:core:0.1-beta")
    implementation("org.noelware.ktor:loader-koin:0.1-beta")
}
