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
