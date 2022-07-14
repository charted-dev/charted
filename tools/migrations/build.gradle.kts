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

import org.noelware.charted.gradle.plugins.docker.tasks.*
import org.noelware.charted.gradle.*
import dev.floofy.utils.gradle.*

plugins {
    id("org.noelware.charted.distribution.docker")
    id("com.diffplug.spotless")
}

group = "org.noelware.charted.tools"
version = "$VERSION"

spotless {
    sql {
        target("migrations/*.sql")
        dbeaver()
    }
}

docker {
    dockerfile(file("./Dockerfile")) {
        tags += listOf(
            "docker.noelware.org/charted/ch-migrations:latest-amd64",
            "docker.noelware.org/charted/ch-migrations:$VERSION-amd64",
            "docker.noelware.org/charted/ch-migrations:${VERSION.major}.${VERSION.minor}-amd64",
            "ghcr.io/charted-dev/ch-migrations:latest-amd64",
            "ghcr.io/charted-dev/ch-migrations:$VERSION-amd64",
            "ghcr.io/charted-dev/ch-migrations:${VERSION.major}.${VERSION.minor}-amd64"
        )
    }
}

tasks.register<BuildDockerImageTask>("buildMigrationsImage") {
    dockerContext.set(projectDir)
    useDockerBuildx by true
    dockerfile by docker.findDockerfile { it.platform == "linux/amd64" }!!

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
