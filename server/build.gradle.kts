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
    `charted-distribution-module`
    `charted-module`
    application
}

application {
    mainClass.set("org.noelware.charted.server.Bootstrap")
}

dependencies {
    // Logback
    implementation("com.fasterxml.jackson.core:jackson-databind:2.13.2.2")
    implementation("net.logstash.logback:logstash-logback-encoder:7.1.1")
    implementation("ch.qos.logback.contrib:logback-json-classic:0.1.5")
    implementation("ch.qos.logback.contrib:logback-jackson:0.1.5")
    implementation("ch.qos.logback:logback-classic:1.2.11")
    implementation("ch.qos.logback:logback-core:1.2.11")

    // Ktor Routing
    api("org.noelware.ktor:core:0.0.1-beta")
    implementation("org.noelware.ktor:loader-koin:0.0.1-beta")

    // Ktor (server)
    api("io.ktor:ktor-server-core:2.0.1")

    // Projects
    implementation(project(":search:elastic"))
    implementation(project(":engines:charts"))
    implementation(project(":engines:oci"))
    implementation(project(":search:meili"))
    implementation(project(":database"))
    implementation(project(":core"))

    // Just for Log4j/JCL -> slf4j
    implementation("org.apache.logging.log4j:log4j-slf4j-impl:2.17.2")
    implementation("org.slf4j:log4j-over-slf4j:1.7.32")
    implementation("org.slf4j:jcl-over-slf4j:1.7.32")

    // TOML (config parsing)
    implementation("com.akuleshov7:ktoml-core:0.2.11")
    implementation("com.akuleshov7:ktoml-file:0.2.11")
}
