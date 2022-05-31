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
    // Ktor (client)
    implementation("io.ktor:ktor-serialization-kotlinx-json")
    implementation("io.ktor:ktor-client-content-negotiation")
    implementation("io.ktor:ktor-serialization")
    implementation("io.ktor:ktor-client-okhttp")
    api("com.squareup.okhttp:okhttp:2.7.5")
    api("io.ktor:ktor-server-core")
    api("io.ktor:ktor-client-core")

    // Prometheus (for metrics)
    implementation("io.prometheus:simpleclient_hotspot:0.15.0")
    implementation("io.prometheus:simpleclient_common:0.15.0")
    implementation("io.prometheus:simpleclient:0.15.0")

    // Haru (scheduling)
    implementation("dev.floofy.haru:Haru:1.3.0")

    // Projects
    implementation(project(":libs:elasticsearch"))
    implementation(project(":libs:meilisearch"))

    // JWT
    implementation("com.auth0:java-jwt:3.19.2")
}
