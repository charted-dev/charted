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

import dev.floofy.utils.gradle.by
import java.text.SimpleDateFormat
import java.util.Date

plugins {
    `charted-module`
    `charted-test`
    application
}

dependencies {
    // Logback
    implementation("net.logstash.logback:logstash-logback-encoder:7.2")
    implementation("ch.qos.logback:logback-classic:1.2.11")
    implementation("ch.qos.logback:logback-core:1.2.11")

    // kotlinx.coroutines Debug
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-debug")

    // Ktor Routing
    implementation("org.noelware.ktor:loader-koin:0.3.1-beta")
    implementation("org.noelware.ktor:core:0.3.1-beta")

    // Ktor (server)
    implementation("io.ktor:ktor-serialization-kotlinx-json")
    implementation("io.ktor:ktor-client-content-negotiation")
    implementation("io.ktor:ktor-server-content-negotiation")
    implementation("io.ktor:ktor-server-auto-head-response")
    implementation("io.ktor:ktor-server-default-headers")
    implementation("io.ktor:ktor-server-double-receive")
    implementation("io.ktor:ktor-server-status-pages")
    implementation("io.ktor:ktor-server-websockets")
    implementation("io.ktor:ktor-serialization")
    implementation("io.ktor:ktor-client-okhttp")
    implementation("io.ktor:ktor-server-netty")
    implementation("io.ktor:ktor-server-cors")

    // Projects
    implementation(project(":features:docker-registry"))
    implementation(project(":features:audit-logs"))
    implementation(project(":features:webhooks"))
    implementation(project(":lib:elasticsearch"))
    implementation(project(":lib:invitations"))
    implementation(project(":lib:meilisearch"))
    implementation(project(":sessions:github"))
    implementation(project(":sessions:local"))
    implementation(project(":lib:cassandra"))
    implementation(project(":lib:analytics"))
    implementation(project(":lib:telemetry"))
    implementation(project(":lib:metrics"))
    implementation(project(":lib:apikeys"))
    implementation(project(":lib:email"))
    implementation(project(":sessions"))
    implementation(project(":database"))
    implementation(project(":core"))

    // JWT
    implementation("com.auth0:java-jwt:3.19.2")

    // Just for Log4j/JCL -> slf4j
    implementation("org.slf4j:log4j-over-slf4j:1.7.36")
    implementation("org.slf4j:jcl-over-slf4j:1.7.36")

    // Conditional logic for logback
    implementation("org.codehaus.janino:janino:3.1.7")
}

application {
    mainClass by "org.noelware.charted.server.Bootstrap"
}

distributions {
    main {
        distributionBaseName by "charted-server"
        contents {
            from(
                "$projectDir/bin/config/logback.properties",
                "$projectDir/bin/config/charted.example.yml",
                "$projectDir/bin/charted-server.ps1",
                "$projectDir/bin/charted-server",
                "$projectDir/charted.service",
                "$projectDir/bin/README.txt",
                "$projectDir/bin/LICENSE"
            )
        }
    }
}

tasks {
    processResources {
        filesMatching("build-info.json") {
            val formatter = SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ssXXX")

            expand(
                mapOf(
                    "version" to "${org.noelware.charted.gradle.VERSION}",
                    "commit_sha" to org.noelware.charted.gradle.COMMIT_HASH,
                    "build_date" to formatter.format(Date())
                )
            )
        }
    }
}
