package org.noelware.charted.gradle.plugins.docker

import org.gradle.api.provider.Property
import org.gradle.api.tasks.Exec
import org.gradle.api.tasks.Input
import org.gradle.api.tasks.TaskAction
import org.gradle.work.DisableCachingByDefault
import org.noelware.charted.gradle.VERSION
import org.noelware.charted.gradle.plugins.docker

@DisableCachingByDefault(because = "Docker caches by default")
abstract class BuildDockerImageTask: Exec() {
    @get:Input
    abstract val dockerfile: Property<Dockerfile>

    @get:Input
    abstract val tag: Property<String>

    init {
        group = "build"

        // By default, we build it as `charted-server:<current version>`
        tag.convention("charted-server:$VERSION")
    }

    @TaskAction
    fun execute() {
        val extension = project.extensions.docker!!
        val dockerImage = dockerfile.get()
        var arguments = listOf("docker", "build", extension.mainDirectory.get(), "-f", dockerImage.path, "--platform", dockerImage.platform)
        for ((key, value) in dockerImage.extraBuildArgs) {
            arguments += listOf("--build-arg", "$key=$value")
        }

        logger.info("$ ${arguments.joinToString(" ")}")
        commandLine(*arguments.toTypedArray())

        // log it to terminal
        standardOutput = System.out
    }
}
