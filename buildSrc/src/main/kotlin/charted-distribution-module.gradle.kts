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

import dev.floofy.utils.gradle.by
import java.io.StringReader
import java.util.Properties

plugins {
    id("nebula.ospackage-base")
    `maven-publish`
    distribution
}

distributions {
    main {
        distributionBaseName by "charted"
        contents {
            from("${rootProject.projectDir}/distribution/README.txt")
            from("${rootProject.projectDir}/distribution/LICENSE")
            from("${rootProject.projectDir}/distribution/config/config.toml")
            from("${rootProject.projectDir}/distribution/config/logback.properties")
        }
    }
}

val isPublishing = System.getenv("NOEL_PUBLISHING") ?: "false"
val YES_NO_REGEX = "(yes|no|true|false|0|1)".toRegex()

if (YES_NO_REGEX.matches(isPublishing) && listOf("yes", "1", "true").contains(isPublishing)) {
    logger.lifecycle("Publishing is enabled! Setting up publishing module...")

    // Get the `publishing.properties` file from the `gradle/` directory
    // in the root project.
    val publishingPropsFile = file("${rootProject.projectDir}/gradle/publishing.properties")
    val publishingProps = Properties()

    // If the file exists, let's get the input stream
    // and load it.
    if (publishingPropsFile.exists()) {
        publishingProps.load(publishingPropsFile.inputStream())
    } else {
        // Check if we do in environment variables
        val accessKey = System.getenv("NOEL_PUBLISHING_ACCESS_KEY") ?: ""
        val secretKey = System.getenv("NOEL_PUBLISHING_SECRET_KEY") ?: ""

        if (accessKey.isNotEmpty() && secretKey.isNotEmpty()) {
            val data = """
            |s3.accessKey=$accessKey
            |s3.secretKey=$secretKey
            """.trimMargin()

            publishingProps.load(StringReader(data))
        }
    }

    publishing {
        publications {
            create<MavenPublication>("distribution") {
                artifact(tasks.distZip.get())
                artifact(tasks.distTar.get())
            }

            repositories {
                maven("s3://cdn.noelware.org") {
                    credentials(AwsCredentials::class.java) {
                        accessKey = publishingProps.getProperty("s3.accessKey") ?: ""
                        secretKey = publishingProps.getProperty("s3.secretKey") ?: ""
                    }
                }
            }
        }
    }
}
