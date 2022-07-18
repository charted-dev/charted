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

import org.noelware.charted.gradle.plugins.docker.tasks.BuildDockerImageTask
import org.noelware.charted.gradle.*
import dev.floofy.utils.gradle.*
import org.noelware.charted.gradle.plugins.docker.Dockerfile

plugins {
    id("org.noelware.charted.distribution.docker")
}

val semver = VERSION.toSemVer(false)
docker {
    addDockerfile(Dockerfile(
        "./amd64.Dockerfile",
        "linux/amd64",
        mapOf("VERSION" to "$VERSION", "COMMIT_SHA" to VERSION.getGitCommit()?.trim()),
        listOf(
            "docker.noelware.org/charted/server:latest-amd64",
            "docker.noelware.org/charted/server:${semver.major}-amd64",
            "docker.noelware.org/charted/server:${semver.major}.${semver.minor}-amd64",
            "docker.noelware.org/charted/server:${semver.major}.${semver.minor}.${semver.patch}-amd64",
            "ghcr.io/charted-dev/charted:latest-amd64",
            "ghcr.io/charted-dev/charted:${semver.major}-amd64",
            "ghcr.io/charted-dev/charted:${semver.major}.${semver.minor}-amd64",
            "ghcr.io/charted-dev/charted:${semver.major}:${semver.minor}.${semver.patch}-amd64"
        ),
        false
    ))

    addDockerfile(Dockerfile(
        "./arm64.Dockerfile",
        "linux/amd64",
        mapOf("VERSION" to "$VERSION", "COMMIT_SHA" to VERSION.getGitCommit()?.trim()),
        listOf(
            "docker.noelware.org/charted/server:latest-arm64",
            "docker.noelware.org/charted/server:${semver.major}-arm64",
            "docker.noelware.org/charted/server:${semver.major}.${semver.minor}-arm64",
            "docker.noelware.org/charted/server:${semver.major}.${semver.minor}.${semver.patch}-arm64",
            "ghcr.io/charted-dev/charted:latest-arm64",
            "ghcr.io/charted-dev/charted:${semver.major}-arm64",
            "ghcr.io/charted-dev/charted:${semver.major}.${semver.minor}-arm64",
            "ghcr.io/charted-dev/charted:${semver.major}:${semver.minor}.${semver.patch}-arm64"
        ),
        false
    ))
}

tasks.register<BuildDockerImageTask>("buildLinux64Image") {
    dockerContext.set(rootProject.projectDir)
    useDockerBuildx by true
    dockerfile by docker.dockerfiles.find { it.platform() == "linux/amd64" }!!

    val shouldCacheFrom = System.getenv("CHARTED_DOCKER_CACHE_FROM")
    if (shouldCacheFrom != null) {
        cacheFrom.set(file(shouldCacheFrom))
    }

    val shouldCacheTo = System.getenv("CHARTED_DOCKER_CACHE_TO")
    if (shouldCacheTo != null) {
        cacheTo.set(file(shouldCacheTo))
    }

    val shouldCacheValue = if (System.getenv("CHARTED_DOCKER_SHOULD_CACHE") != null) {
        val value = System.getenv("CHARTED_DOCKER_SHOULD_CACHE")
        value.matches("no|false|0$".toRegex())
    } else {
        true
    }

    shouldCache by shouldCacheValue
}
