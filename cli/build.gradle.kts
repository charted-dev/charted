/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

import org.noelware.charted.gradle.*
import dev.floofy.utils.gradle.*
import java.text.SimpleDateFormat
import java.util.Date

plugins {
    `charted-module`
    application
}

dependencies {
    implementation(libs.spring.security.crypto)
    implementation(libs.semver)
    implementation(libs.mordant)
    implementation(libs.clikt)
    implementation(libs.kaml)

    implementation(project(":modules:config:kotlin-script"))
    implementation(project(":modules:helm-charts"))
    implementation(project(":modules:config:yaml"))
    implementation(project(":databases:postgres"))
    implementation(project(":modules:config:dsl"))
    implementation(project(":modules:analytics"))
    implementation(project(":modules:storage"))
    implementation(project(":modules:metrics"))
    implementation(project(":modules:config"))
    implementation(project(":server"))

    // Tegral OpenAPI
    implementation(libs.tegral.openapi)
}

application {
    mainClass by "org.noelware.charted.cli.CliMainKt"
}

distributions {
    main {
        distributionBaseName by "charted"
        contents {
            into("systemd") {
                from("$projectDir/distribution/charted.service")
            }

            into("bin") {
                from("$projectDir/distribution/bin/charted.ps1")
                from("$projectDir/distribution/bin/charted")
            }

            into("config") {
                from("$projectDir/distribution/config/logback.properties")
                from("$projectDir/distribution/config/charted.yaml")
            }

            from(
                "$projectDir/distribution/README.txt",
                "$projectDir/distribution/LICENSE"
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
                    "version" to "$VERSION",
                    "commit_sha" to VERSION.gitCommitHash!!.trim(),
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

    startScripts {
        enabled = false
    }
}
