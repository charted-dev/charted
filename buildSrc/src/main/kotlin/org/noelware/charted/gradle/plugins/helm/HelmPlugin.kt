package org.noelware.charted.gradle.plugins.helm

import io.github.z4kn4fein.semver.constraints.satisfiedBy
import io.github.z4kn4fein.semver.constraints.toConstraint
import io.github.z4kn4fein.semver.toVersion
import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.kotlin.dsl.create
import org.gradle.kotlin.dsl.register
import java.io.ByteArrayOutputStream

class HelmPlugin: Plugin<Project> {
    override fun apply(project: Project) {
        project.extensions.create<HelmPluginExtension>("helm")
        project.logger.lifecycle("[distribution:helm] Checking if `helm` is installed...")

        val stdout = ByteArrayOutputStream()
        val result = project.exec {
            commandLine("helm", "version", "--template", "\"{{.Version}}\"")
            setIgnoreExitValue(true)
            standardOutput = stdout
        }

        val stdoutStr = String(stdout.toByteArray())
        if (result.exitValue != 0) {
            project.logger.lifecycle("[distribution:helm] Unable to run 'helm version' to check if Helm was installed.\n\n[ == STDOUT == ]:\n$stdoutStr")
            return
        }

        val helmVersion = stdoutStr.replace("\"", "").replace("v", "").trim()
        if (!(">=3.x.x".toConstraint() satisfiedBy helmVersion.toVersion())) {
            project.logger.lifecycle("[distribution:helm] charted-server requires Helm 3 to be installed, currently using [$helmVersion]")
            return
        }

        project.logger.lifecycle("[distribution:helm] Helm v$helmVersion exists on machine!")
        project.tasks.register<DeployChartTask>("deployChart")
        project.tasks.register<TestChartTask>("testChart")

        project.logger.lifecycle("Added :deployChart and :testChart tasks~ ^-^")
    }
}
