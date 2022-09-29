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
    // Logback
    implementation(libs.jackson.databind)
    implementation(libs.logback.classic)
    implementation(libs.logback.core)

    // Ktor
    implementation(libs.ktor.server.netty) // so we can get types for the application engines
    implementation(libs.ktor.server.core)

    // Projects
    implementation(project(":database"))
}
