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

@file:Suppress("UnstableApiUsage")

import org.noelware.infra.gradle.toolchains.NoelwareJvmToolchainResolver

rootProject.name = "charted-server"

pluginManagement {
    repositories {
        maven("https://maven.floofy.dev/repo/releases")
        maven("https://maven.noelware.org")
        gradlePluginPortal()
        mavenCentral()
        mavenLocal()
    }
}

buildscript {
    dependencies {
        classpath("org.noelware.gradle:gradle-infra-plugin:1.1.1")
    }
}

plugins {
    //id("org.gradle.toolchains.foojay-resolver-convention") version "0.4.0"
    id("org.noelware.gradle.settings") version "1.1.1"
    id("com.gradle.enterprise") version "3.12.3"
}

toolchainManagement {
    jvm {
        javaRepositories {
            repository("noelware") {
                resolverClass.set(NoelwareJvmToolchainResolver::class.java)
            }
        }
    }
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
    versionCatalogs {
        create("libs") {
            from(files("./gradle/build.versions.toml"))
        }
    }
}
