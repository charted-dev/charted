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
import java.io.File

plugins {
    `charted-module`
    `charted-test`
    application
}

dependencies {
    // Logstash
    implementation(libs.logback.logstash)
    implementation(libs.logback.classic)
    implementation(libs.sentry.logback)
    implementation(libs.logback.core)

    // kotlinx.coroutines Debug
    implementation(libs.kotlinx.coroutines.debug)

    // Ktor Routing
    implementation(libs.noelware.ktor.routing.loaders.koin)
    implementation(libs.noelware.ktor.routing.core)

    // Ktor (Server)
    implementation(libs.ktor.client.content.negotitation)
    implementation(libs.ktor.serialization.kotlinx.json)
    implementation(libs.ktor.server.content.negotiation)
    implementation(libs.ktor.server.auto.head.response)
    implementation(libs.ktor.server.default.headers)
    implementation(libs.ktor.server.double.receive)
    implementation(libs.ktor.server.status.pages)
    implementation(libs.ktor.server.websockets)
    implementation(libs.ktor.serialization)
    implementation(libs.ktor.client.okhttp)
    implementation(libs.ktor.server.netty)
    implementation(libs.ktor.server.cors)

    // Projects
    implementation(project(":features:docker-registry"))
    implementation(project(":config:kotlin-script"))
    implementation(project(":features:audit-logs"))
    implementation(project(":features:webhooks"))
    implementation(project(":lib:elasticsearch"))
    implementation(project(":lib:invitations"))
    implementation(project(":lib:meilisearch"))
    implementation(project(":sessions:github"))
    implementation(project(":lib:tracing:apm"))
    implementation(project(":sessions:local"))
    implementation(project(":lib:cassandra"))
    implementation(project(":lib:analytics"))
    implementation(project(":lib:telemetry"))
    implementation(project(":config:yaml"))
    implementation(project(":lib:metrics"))
    implementation(project(":lib:apikeys"))
    implementation(project(":lib:avatars"))
    implementation(project(":lib:email"))
    implementation(project(":lib:stats"))
    implementation(project(":sessions"))
    implementation(project(":database"))
    implementation(project(":config"))
    implementation(project(":core"))

    // JWT
    implementation(libs.jwt)

    // Just for Log4j/JCL -> slf4j
    implementation(libs.slf4j.over.log4j)
    implementation(libs.slf4j.over.jcl)

    // Conditional logic for logback
    implementation(libs.janino)
}

application {
    mainClass by "org.noelware.charted.server.Bootstrap"
}

distributions {
    main {
        distributionBaseName by "charted-server"
        contents {
            into("bin") {
                from("$projectDir/bin/charted-server.ps1")
                from("$projectDir/bin/charted-server")
            }

            into("config") {
                from("$projectDir/bin/config/logback.properties")
                from("$projectDir/bin/config/charted.yml")
            }

            from(
                "$projectDir/bin/README.txt",
                "$projectDir/bin/LICENSE"
            )
        }
    }
}

val buildWebUI by tasks.registering(Exec::class) {
    workingDir(project(":web").projectDir)
    commandLine("yarn")
    args("build")
}

val collectWebUI by tasks.registering(Copy::class) {
    dependsOn(buildWebUI)

    from(File(project(":web").projectDir, "dist"))
    into(File(projectDir, "build/resources/main/frontend"))
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

    distZip {
        archiveFileName by "charted-server.zip"
    }

    distTar {
        archiveFileName by "charted-server.tar.gz"
        compression = Compression.GZIP // use gzip for the compression :>
    }

    installDist {
        dependsOn(collectWebUI)
    }

    startScripts {
        enabled = false
    }
}
