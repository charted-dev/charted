package org.noelware.charted.gradle.plugins.docker

import org.gradle.api.provider.Property
import java.io.File

/**
 * Represents a Dockerfile that the tasks can use to interact with.
 * @param path The path to the image.
 * @param platform The platform to use, default: linux/amd64
 * @param extraBuildArgs Any extra build arguments to add
 */
data class Dockerfile(
    val path: String,
    var platform: String = "linux/amd64",
    var extraBuildArgs: Map<String, String> = mapOf()
) {
    fun addBuildArgument(pair: Pair<String, String>): Dockerfile {
        extraBuildArgs += pair
        return this
    }
}

abstract class DockerPluginExtension {
    abstract val minDockerVersion: Property<String>
    abstract val mainDirectory: Property<String>

    val dockerfiles = mutableListOf<Dockerfile>()

    init {
        minDockerVersion.convention(">=20.10")
        mainDirectory.convention("./distribution/docker")
    }

    fun dockerfile(path: String, builder: Dockerfile.() -> Unit = {}) {
        dockerfiles.add(Dockerfile(path).apply(builder))
    }

    fun dockerfile(path: File, builder: Dockerfile.() -> Unit = {}) {
        dockerfiles.add(Dockerfile("$path").apply(builder))
    }
}
