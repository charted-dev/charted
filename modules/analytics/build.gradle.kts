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
    `charted-module` // only so "api()" can work idk why
}

dependencies {
    // Metrics module (so it can consume more data)
    implementation(project(":modules:elasticsearch"))
    implementation(project(":databases:postgres"))
    implementation(project(":modules:metrics"))
    implementation(project(":modules:redis"))

    // Noelware Analytics
    api(libs.noelware.analytics.server.kotlin.support)
    api(libs.noelware.analytics.server)
    implementation(libs.grpc.services)
}
