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
    id("com.gradle.enterprise") version "3.10.2"
}

includeBuild("build-tools")
include(
    ":common",
    ":core",
    ":database",
    ":distribution:aur",
    ":distribution:chart",
    ":distribution:deb",
    ":distribution:docker",
    ":distribution:homebrew",
    ":distribution:rpm",
    ":distribution:scoop",
    ":features:audit-logs",
    ":features:docker-registry",
    ":features:webhooks",
    ":lib:analytics:protobufs",
    ":lib:analytics",
    ":lib:apikeys",
    ":lib:cassandra",
    ":lib:elasticsearch",
    ":lib:email",
    ":lib:invitations",
    ":lib:meilisearch",
    ":lib:metrics",
    ":lib:telemetry",
    ":server",
    ":sessions",
    ":sessions:apple",
    ":sessions:github",
    ":sessions:google",
    ":sessions:local",
    ":sessions:noelware",
    ":testing",
    ":tools:migrations"
)

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

//    if (System.getProperty("org.noelware.charted.ignoreJavaCheck") == "true")
//        return@settingsEvaluated
//
//    if (JavaVersion.current() != JavaVersion.VERSION_17) {
//        throw GradleException("This build of charted-server requires JDK 17. It's currently [${System.getProperty("java.home")}], you can ignroe this check by providing '-Dorg.noelware.charted.ignoreJavaCheck=true'")
//    }
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
        }

        obfuscation {
            ipAddresses { listOf("0.0.0.0") }
            hostname { "[redacted]" }
            username { "[redacted]" }
        }
    }
}
