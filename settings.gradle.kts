/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

import org.gradle.internal.os.OperatingSystem
import java.net.URI

rootProject.name = "charted-server"

pluginManagement {
    repositories {
        gradlePluginPortal()
        mavenCentral()
        mavenLocal()
    }
}

plugins {
    id("org.gradle.toolchains.foojay-resolver-convention") version "0.5.0"
    id("com.gradle.enterprise") version "3.13.1"
}

enableFeaturePreview("TYPESAFE_PROJECT_ACCESSORS")
dependencyResolutionManagement {
    versionCatalogs {
        create("libs") {
            from(files("./gradle/build.versions.toml"))
        }
    }
}

includeBuild("build-logic")
include(
    ":cli",
    ":common",
    ":server",
    ":server:single-user",
    ":config",
    ":config:dsl",
    ":config:kotlin-script",
    ":config:yaml",
    ":distribution:debian",
    ":features:docker-registry",
    ":features:garbage-collection",
    ":features:invitations",
    ":modules:analytics",
    ":modules:avatars",
    ":modules:cache-worker",
    ":modules:clickhouse",
    ":modules:emails",
    ":modules:helm-charts",
    ":modules:logging",
    ":modules:metrics",
    ":modules:openapi",
    ":modules:postgresql",
    ":modules:redis",
    ":modules:search",
    ":modules:search:elasticsearch",
    ":modules:search:meilisearch",
    ":modules:sessions",
    ":modules:sessions:integrations:github",
    ":modules:sessions:ldap",
    ":modules:sessions:local",
    ":modules:setup",
    ":modules:setup:single-user",
    ":modules:storage",
    ":modules:telemetry",
    ":modules:tracing",
    ":modules:tracing:elastic-apm",
    ":modules:tracing:opentelemetry",
    ":modules:tracing:sentry",
    ":testing:containers",
)

val buildScanServer: String = System.getProperty("org.noelware.gradle.build-scan.server", "")
gradleEnterprise {
    buildScan {
        if (buildScanServer.isNotBlank()) {
            publishAlways()
            server = buildScanServer
        } else {
            termsOfServiceAgree = "yes"
            termsOfServiceUrl = "https://gradle.com/terms-of-service"

            if (System.getenv("CI") != null) publishAlways()
        }

        obfuscation {
            ipAddresses { listOf("0.0.0.0") }
            hostname { "[redacted]" }
            username { "[redacted]" }
        }
    }
}

@Suppress("INACCESSIBLE_TYPE")
val validOs = listOf(OperatingSystem.LINUX, OperatingSystem.MAC_OS, OperatingSystem.WINDOWS).map { it.familyName }
val validArch = listOf("amd64", "arm64")

gradle.settingsEvaluated {
    val os = OperatingSystem.current()
    if (!validOs.contains(os.familyName)) {
        throw GradleException(
            """
        |charted-server can only be developed and compiled on Windows, macOS, or Linux. Received ${os.familyName} (${os.version}),
        |which is not a valid platform to develop on.
        """.trimMargin("|"),
        )
    }

    val arch = when (System.getProperty("os.arch")) {
        "amd64", "x86_64" -> "amd64"
        "aarch64", "arm64" -> "arm64"
        else -> "unknown"
    }

    if (!validArch.contains(arch)) {
        throw GradleException("Building or developing charted-server on architecture [${System.getProperty("os.arch")}] is not supported")
    }

    val buildCacheUri = System.getProperty("org.noelware.gradle.build-cache.url")
    val buildCacheDir = System.getProperty("org.noelware.gradle.build-cache.dir")
    val shouldOverride = buildCacheDir != null || buildCacheUri != null

    if (shouldOverride && buildCacheUri != null) {
        val uri = URI.create(buildCacheUri)
        val isCi = System.getenv("CI") != null

        buildCache {
            remote(HttpBuildCache::class) {
                isAllowInsecureProtocol = uri.scheme == "http"
                isPush = isCi
                url = uri

                val username = System.getProperty("org.noelware.gradle.build-cache.username")
                if (username != null) {
                    val password = System.getProperty("org.noelware.gradle.build-cache.password")
                        ?: throw GradleException("Missing `org.noelware.gradle.build-cache.password` system property when using `org.noelware.gradle.build-cache.username`")

                    credentials {
                        this.username = username
                        this.password = password
                    }
                }
            }
        }
    }

    if (shouldOverride && buildCacheDir != null) {
        val buildCacheDirFile = File(buildCacheDir)
        if (!buildCacheDirFile.exists()) buildCacheDirFile.mkdirs()
        if (!buildCacheDirFile.isDirectory) throw GradleException("Expected path [$buildCacheDir] to be a directory")

        buildCache {
            local {
                removeUnusedEntriesAfterDays = 14
                directory = buildCacheDirFile
            }
        }
    }
}
