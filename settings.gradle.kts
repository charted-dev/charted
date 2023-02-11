import java.net.URI

/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

rootProject.name = "charted-server"

pluginManagement {
    repositories {
        gradlePluginPortal()
        mavenCentral()
        mavenLocal()
    }
}

plugins {
    id("com.gradle.enterprise") version "3.12.3"
}

include(
    ":cli",
    ":common",
    ":server",
    ":distribution:deb",
    ":distribution:rpm",
    ":databases:clickhouse",
    ":databases:clickhouse:migrations",
    ":databases:postgres",
    ":modules:analytics:extensions",
    ":modules:analytics",
    ":modules:apikeys",
    ":modules:audit-logs",
    ":modules:avatars",
    ":modules:config:dsl",
    ":modules:config:kotlin-script",
    ":modules:config:yaml",
    ":modules:docker-registry",
    ":modules:elasticsearch",
    ":modules:emails",
    ":modules:garbage-collector",
    ":modules:helm-charts",
    ":modules:invitations",
    ":modules:logging",
    ":modules:meilisearch",
    ":modules:metrics",
    ":modules:redis",
    ":modules:sessions",
    ":modules:sessions:ldap",
    ":modules:sessions:local",
    ":modules:sessions:openid",
    ":modules:sessions:integrations:github",
    ":modules:sessions:integrations:noelware",
    ":modules:storage",
    ":modules:telemetry",
    ":modules:webhooks",
    ":test:containers",
    ":test:framework"
)

dependencyResolutionManagement {
    @Suppress("UnstableApiUsage")
    versionCatalogs {
        create("libs") {
            from(files("./gradle/build.versions.toml"))
        }
    }
}

val isCI = System.getenv("CI") != null
gradle.settingsEvaluated {
    val javaVersion = JavaVersion.current()
    val disableJavaSanityCheck = when {
        System.getProperty("org.noelware.gradle.ignoreJavaCheck", "false") matches "^(yes|true|1|si|si*)$".toRegex() -> true
        (System.getenv("GRADLE_DISABLE_JAVA_SANITY_CHECK") ?: "false") matches "^(yes|true|1|si|si*)$".toRegex() -> true
        else -> false
    }

    if (!disableJavaSanityCheck && javaVersion.majorVersion.toInt() < 17) {
        throw GradleException("""
        |charted-server requires Java 17 or higher to be developed on with Gradle. You're currently on
        |Java ${javaVersion.majorVersion} [${Runtime.version()}], to disable the sanity checks, you will
        |need to pass in:
        |
        |  - environment variable `GRADLE_DISABLE_JAVA_SANITY_CHECK` with `yes`, `true`, `1`, or `si`.
        |                                    ~ or ~
        |  - system property `org.noelware.gradle.ignoreJavaCheck` with `yes`, `true`, `1`, or `si`.
        """.trimMargin("|"))
    }

    val buildCacheHttpUri: String? = System.getProperty("org.noelware.gradle.buildCache.url")
    val buildCacheDir: String? = System.getProperty("org.noelware.gradle.buildCache.dir")
    val shouldOverrideBuildCache = buildCacheHttpUri != null || buildCacheDir != null

    if (buildCacheHttpUri != null && buildCacheDir != null) {
        logger.warn("You have defined both system properties: [org.noelware.gradle.buildCache.url] and [org.noelware.gradle.buildCache.dir], please use one or another!")
    }

    logger.lifecycle(if (shouldOverrideBuildCache) "Using custom build cache strategy..." else "Cannot override build cache without `org.noelware.gradle.buildCache.dir` or `org.noelware.gradle.buildCache.url` system property.")
    if (shouldOverrideBuildCache && buildCacheHttpUri != null) {
        logger.info("Attempting to place build cache in URI [$buildCacheHttpUri]")

        val uri = URI.create(buildCacheHttpUri)
        buildCache {
            remote<HttpBuildCache> {
                isAllowInsecureProtocol = uri.scheme == "http"
                isPush = isCI || System.getProperty("org.noelware.gradle.buildCache.shouldPush", "false") matches "^(yes|true|si|si*)$".toRegex()
                url = uri

                val username = System.getProperty("org.noelware.gradle.buildCache.username")
                if (username != null) {
                    val password = System.getProperty("org.noelware.gradle.buildCache.password") ?: throw GradleException("Missing `org.noelware.gradle.buildCache.password` system property")
                    credentials {
                        this.username = username
                        this.password = password
                    }
                }
            }
        }
    }

    if (shouldOverrideBuildCache && buildCacheDir != null) {
        logger.info("Configuring build cache in directory [$buildCacheDir]")
        val file = File(buildCacheDir)
        if (!file.exists()) file.mkdirs()

        buildCache {
            local {
                directory = "$file"
                removeUnusedEntriesAfterDays = 7
            }
        }
    }
}

val buildScanServer = System.getProperty("org.noelware.gradle.buildScan.server", "")!!
gradleEnterprise {
    buildScan {
        if (buildScanServer.isNotEmpty()) {
            server = buildScanServer

            publishAlways()
        } else {
            termsOfServiceUrl = "https://gradle.com/terms-of-service"
            termsOfServiceAgree = "yes"

            // Always publish if we're on CI.
            if (isCI) {
                publishAlways()
            }
        }

        obfuscation {
            ipAddresses { listOf("0.0.0.0") }
            hostname { "[redacted]" }
            username { "[redacted]" }
        }
    }
}
