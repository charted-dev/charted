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

import org.noelware.charted.gradle.*
import java.util.Properties
import java.io.StringReader

plugins {
    `maven-publish`
    distribution
}

val publishingPropsFile = file("${rootProject.projectDir}/gradle/publishing.properties")
val publishingProps = Properties()

if (publishingPropsFile.exists()) {
    publishingProps.load(publishingPropsFile.inputStream())
} else {
    val accessKey = System.getenv("NOELWARE_PUBLISHING_ACCESS_KEY") ?: ""
    val secretKey = System.getenv("NOELWARE_PUBLISHING_SECRET_KEY") ?: ""

    if (accessKey.isNotEmpty() && secretKey.isNotEmpty()) {
        val data = """
        |s3.accessKey=$accessKey
        |s3.secretKey=$secretKey
        """.trimMargin()

        publishingProps.load(StringReader(data))
    }
}

val isNightlyRelease = run {
    val env = System.getenv("NOELWARE_PUBLISHING_NIGHTLY") ?: "false"
    env.matches("^(yes|true|1|si|si*)$".toRegex())
}

publishing {
    publications {
        create<MavenPublication>("charted") {
            // TODO: run the `:server:distZip`
        }
    }

    repositories {
        val arch = System.getProperty("os.arch")
        val architecture = when {
            arch == "amd64" -> "x64"
            arch.matches("^(arm64|aarch64)$".toRegex()) -> "aarch64"
            else -> error("Architecture $arch is not supported.")
        }

        val url = "s3://dl.noelware.org/charted/server/${rootProject.version}/$architecture"
        maven(url) {
            credentials(AwsCredentials::class) {
                accessKey = publishingProps.getProperty("s3.accessKey") ?: System.getenv("NOELWARE_PUBLISHING_ACCESS_KEY") ?: ""
                secretKey = publishingProps.getProperty("s3.secretKey") ?: System.getenv("NOELWARE_PUBLISHING_SECRET_KEY") ?: ""
            }
        }
    }
}
