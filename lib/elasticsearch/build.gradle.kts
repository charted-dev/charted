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
    implementation(libs.elasticsearch.rest.client.sniffer)
    implementation(libs.elasticsearch.rest.client)
    implementation(libs.elasticsearch.java.client)
    implementation(project(":lib:metrics"))
    implementation(libs.jackson.databind)
    implementation(project(":lib:stats"))
    implementation(project(":database"))

    testImplementation(libs.testcontainers.elasticsearch)
    testImplementation(project(":testing"))
    testImplementation(libs.slf4j.simple)
}
