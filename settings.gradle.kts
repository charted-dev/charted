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

rootProject.name = "charted-server"

pluginManagement {
    repositories {
        gradlePluginPortal()
        mavenCentral()
        mavenLocal()
    }
}

plugins {
    id("org.gradle.toolchains.foojay-resolver-convention") version "0.4.0"
    id("com.gradle.enterprise") version "3.12.3"
}

enableFeaturePreview("TYPESAFE_PROJECT_ACCESSORS")
dependencyResolutionManagement {
    versionCatalogs {
        create("libs") {
            from(files("./gradle/build.versions.toml"))
        }
    }
}

include(
    ":cli",
    ":common",
    ":server",
    ":config",
    ":config:dsl",
    ":config:kotlin-script",
    ":config:yaml",
    ":distribution:debian",
    ":features:audit-logs",
    ":features:docker-registry",
    ":features:garbage-collection",
    ":features:invitations",
    ":features:webhooks",
    ":modules:analytics",
    ":modules:avatars",
    ":modules:emails",
    ":modules:helm-charts",
    ":modules:invitations",
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
    ":modules:storage",
    ":modules:telemetry",
    ":testing:containers",
    ":testing:framework",
    ":testing:gradle:integ-runner",
)
