package org.noelware.charted.gradle.plugins.docker

import io.github.z4kn4fein.semver.constraints.satisfiedBy
import io.github.z4kn4fein.semver.constraints.toConstraint
import io.github.z4kn4fein.semver.toVersion
import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.kotlin.dsl.create
import org.gradle.kotlin.dsl.register
import java.io.ByteArrayOutputStream

class DockerPlugin: Plugin<Project> {
    override fun apply(project: Project) {
        val extension = project.extensions.create<DockerPluginExtension>("docker")
        project.logger.lifecycle("[distribution:docker] Checking if `docker` is installed...")

        val stdout = ByteArrayOutputStream()
        val result = project.exec {
            commandLine("docker", "version", "--format='{{.Client.Version}}'")
            setIgnoreExitValue(true)
            standardOutput = stdout
        }

        val stdoutStr = String(stdout.toByteArray())
        if (result.exitValue != 0) {
            project.logger.lifecycle("[distribution:docker] Unable to run 'docker version' to check if Docker was installed.\n\n[ == STDOUT == ]:\n$stdoutStr")
            return
        }

        val minVersion = extension.minDockerVersion.get()
        project.logger.lifecycle("[distribution:docker] Checking if Docker version satisifies '$minVersion'")

        val constraint = minVersion.toConstraint()
        val currentDockerVersion = stdoutStr.replace("'", "").trim()

        if (!(constraint satisfiedBy currentDockerVersion.toVersion(strict = false))) {
            project.logger.lifecycle("[distribution:docker] Version $stdoutStr is not satisified by constraint $minVersion.")
            return
        }

        project.logger.lifecycle("[distribution:docker] \uD83D\uDC33 Using Docker v$currentDockerVersion!")
        project.tasks.register<BuildDockerImageTask>("buildDockerImage")

        project.logger.lifecycle("[distribution:docker] Added the buildDockerImage task~ ^-^")
    }
}
