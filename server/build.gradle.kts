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

import org.noelware.charted.gradle.*
import java.text.SimpleDateFormat
import dev.floofy.utils.gradle.*
import java.util.Date

plugins {
    `charted-distribution-module`
    `charted-module`
    application
}

application {
    mainClass by "org.noelware.charted.server.Bootstrap"
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
    implementation("org.noelware.ktor:loader-koin:0.1.1-beta")
    implementation("org.noelware.ktor:core:0.1.1-beta")

    // Ktor (server)
    implementation("io.ktor:ktor-serialization-kotlinx-json")
    implementation("io.ktor:ktor-server-content-negotiation")
    implementation("io.ktor:ktor-server-auto-head-response")
    implementation("io.ktor:ktor-server-default-headers")
    implementation("io.ktor:ktor-server-double-receive")
    implementation("io.ktor:ktor-server-status-pages")
    implementation("io.ktor:ktor-serialization")
    implementation("io.ktor:ktor-server-netty")
    implementation("io.ktor:ktor-server-cors")
    api("io.ktor:ktor-server-core")

    // Projects
    implementation(project(":libs:elasticsearch"))
    implementation(project(":libs:meilisearch"))
    implementation(project(":libs:telemetry"))
    implementation(project(":libs:analytics"))
    implementation(project(":audit-logs"))
    implementation(project(":oci-proxy"))
    implementation(project(":webhooks"))
    implementation(project(":database"))
    implementation(project(":core"))

    // JWT
    implementation("com.auth0:java-jwt:3.19.2")

    // Just for Log4j/JCL -> slf4j
    implementation("org.slf4j:log4j-over-slf4j:1.7.36")
    implementation("org.slf4j:jcl-over-slf4j:1.7.36")

    // Conditional logic for logback
    implementation("org.codehaus.janino:janino:3.1.7")

    // Commons validator
    implementation("commons-validator:commons-validator:1.7")

    // Spring Security (argon2 hashing)
    implementation("org.springframework.security:spring-security-crypto:5.6.3")

    // YAML (configuration)
    implementation("com.charleskorn.kaml:kaml:0.44.0")
}

tasks {
    processResources {
        filesMatching("build-info.json") {
            val date = Date()
            val formatter = SimpleDateFormat("EEE, MMM d, YYYY - HH:mm:ss a")

            expand(
                mapOf(
                    "version" to "$VERSION",
                    "commit_sha" to COMMIT_HASH,
                    "build_date" to formatter.format(date)
                )
            )
        }
    }
}
