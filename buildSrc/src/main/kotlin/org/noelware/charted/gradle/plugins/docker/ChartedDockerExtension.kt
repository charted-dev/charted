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

package org.noelware.charted.gradle.plugins.docker

import org.gradle.api.provider.Property
import java.io.File

data class Dockerfile(
    val path: String,
    var platform: String = "linux/amd64",
    var buildArguments: Map<String, String> = mapOf(),
    var image: String = "docker.noelware.org/charted/server",
    var tags: List<String> = listOf(),
    var isWindows: Boolean = false
): java.io.Serializable

abstract class ChartedDockerExtension {
    abstract val minimumDockerVersion: Property<String>
    private val dockerfiles = mutableListOf<Dockerfile>()

    init {
        minimumDockerVersion.convention(">=20.10")
    }

    fun dockerfile(path: String, builder: Dockerfile.() -> Unit = {}) {
        dockerfiles.add(Dockerfile(path).apply(builder))
    }

    fun dockerfile(path: File, builder: Dockerfile.() -> Unit = {}) {
        dockerfiles.add(Dockerfile(path.toPath().toString()).apply(builder))
    }

    fun findDockerfile(predicate: (Dockerfile) -> Boolean): Dockerfile? = dockerfiles.find(predicate)
}
