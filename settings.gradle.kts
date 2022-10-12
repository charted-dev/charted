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

rootProject.name = "charted-server"

pluginManagement {
    repositories {
        gradlePluginPortal()
        mavenCentral()
        mavenLocal()
    }
}

plugins {
    id("com.gradle.enterprise") version "3.11.2"
}

includeBuild("build-tools")
include(
    ":benchmarking",
    ":common",
    ":core",
    ":config",
    ":config:kotlin-script",
    ":config:yaml",
    ":database",
    ":distribution:aur",
    ":distribution:chart",
    ":distribution:deb",
    ":distribution:docker",
    ":distribution:homebrew",
    ":distribution:rpm",
    ":distribution:scoop",
    ":features:audit-logs",
    ":features:chart-engine",
    ":features:docker-registry",
    ":features:webhooks",
    ":lib:analytics:protobufs",
    ":lib:analytics",
    ":lib:apikeys",
    ":lib:avatars",
    ":lib:cassandra",
    ":lib:elasticsearch",
    ":lib:email",
    ":lib:gc",
    ":lib:invitations",
    ":lib:meilisearch",
    ":lib:metrics",
    ":lib:stats",
    ":lib:telemetry",
    ":lib:tracing",
    ":lib:tracing:apm",
    ":lib:tracing:opentelemetry",
    ":lib:tracing:apm:instrumented",
    ":server",
    ":sessions",
    ":sessions:apple",
    ":sessions:github",
    ":sessions:google",
    ":sessions:local",
    ":sessions:noelware",
    ":testing",
    ":testing:helm",
    ":testing:kubernetes",
    ":testing:server",
    ":tools:cli",
    ":tools:migrations",
    ":web",
    ":web:distribution",
    ":web:distribution:docker",
    ":web:distribution:helm",
    ":workers",
    ":workers:gc",
    ":workers:indexers",
    ":workers:indexers:elasticsearch",
    ":workers:indexers:meilisearch",
    ":workers:messaging",
    ":workers:messaging:kafka",
    ":workers:messaging:redis"
)

dependencyResolutionManagement {
    versionCatalogs {
        create("libs") {
            from(files("./gradle/build.versions.toml"))
        }
    }
}

gradle.settingsEvaluated {
    logger.lifecycle("[preinit] Checking if we can overwrite cache to main directory?")
    val overrideBuildCacheProp: String? = System.getProperty("org.noelware.charted.overwriteCache")
    val buildCacheDir = when (val prop = System.getProperty("org.noelware.charted.cachedir")) {
        null -> "${System.getProperty("user.dir")}/.caches/gradle"
        else -> when {
            prop.startsWith("~/") -> "${System.getProperty("user.home")}${prop.substring(1)}"
            prop.startsWith("./") -> "${System.getProperty("user.dir")}${prop.substring(1)}"
            else -> prop
        }
    }

    if (overrideBuildCacheProp != null && overrideBuildCacheProp.matches("^yes|true|1|si$".toRegex())) {
        logger.lifecycle("[preinit:cache] Setting up build cache directory in [$buildCacheDir]")
        val file = File(buildCacheDir)
        if (!file.exists()) file.mkdirs()

        buildCache {
            local {
                directory = "$file"
                removeUnusedEntriesAfterDays = 7
            }
        }
    } else {
        logger.lifecycle("[preinit] Use `-Dorg.noelware.charted.overwriteCache=true|yes|1|si` to overwrite cache to [$buildCacheDir]")
    }

    val disableJavaSanityCheck = when {
        System.getProperty("org.noelware.charted.ignoreJavaCheck", "false").matches("^(yes|true|1|si|si*)$".toRegex()) -> true
        (System.getenv("CHARTED_DISABLE_JAVA_SANITY_CHECK") ?: "false").matches("^(yes|true|1|si|si*)$".toRegex()) -> true
        else -> false
    }

    if (disableJavaSanityCheck)
        return@settingsEvaluated

    val version = JavaVersion.current()
    if (version.majorVersion.toInt() < 17)
        throw GradleException("Developing charted-server requires JDK 17 or higher, it is currently set in [${System.getProperty("java.home")}, ${System.getProperty("java.version")}] - You can ignore this check by providing the `-Dorg.noelware.charted.ignoreJavaCheck=true` system property.")
}

val buildScanServer = System.getProperty("org.noelware.charted.gradle.build-scan-server", "") ?: ""
gradleEnterprise {
    buildScan {
        if (buildScanServer.isNotEmpty()) {
            server = buildScanServer
            isCaptureTaskInputFiles = true
            publishAlways()
        } else {
            termsOfServiceUrl = "https://gradle.com/terms-of-service"
            termsOfServiceAgree = "yes"

            // Always publish if we're on CI.
            if (System.getenv("CI") != null) {
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
