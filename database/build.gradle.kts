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
    implementation("net.perfectdreams.exposedpowerutils:postgres-power-utils:1.0.0")
    implementation("org.springframework.security:spring-security-crypto:5.7.1")
    implementation("org.jetbrains.exposed:exposed-kotlin-datetime")
    implementation("commons-validator:commons-validator:1.7")
    implementation("io.ktor:ktor-server-core")

    testImplementation("org.testcontainers:postgresql:1.17.2")
}
