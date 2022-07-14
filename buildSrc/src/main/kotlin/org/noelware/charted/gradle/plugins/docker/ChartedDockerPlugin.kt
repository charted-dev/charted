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

import io.github.z4kn4fein.semver.constraints.satisfiedBy
import io.github.z4kn4fein.semver.constraints.toConstraint
import io.github.z4kn4fein.semver.toVersion
import org.gradle.api.Project
import org.gradle.api.Plugin
import org.gradle.kotlin.dsl.create
import java.io.ByteArrayOutputStream

class ChartedDockerPlugin: Plugin<Project> {
    override fun apply(project: Project) {
        val extension = project.extensions.create<ChartedDockerExtension>("docker")
        project.logger.lifecycle("[distribution:docker] Checking if `docker` is installed...")

        val stdout = ByteArrayOutputStream()
        try {
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

            val minVersion = extension.minimumDockerVersion.get()
            project.logger.lifecycle("[distribution:docker] Checking if Docker version satisifies '$minVersion'")

            val constraint = minVersion.toConstraint()
            val currentDockerVersion = stdoutStr.replace("'", "").trim()

            if (!(constraint satisfiedBy currentDockerVersion.toVersion(strict = false))) {
                project.logger.lifecycle("[distribution:docker] Version $stdoutStr is not satisified by constraint $minVersion.")
                return
            }

            project.logger.lifecycle("[distribution:docker] \uD83D\uDC33 Using Docker v$currentDockerVersion!")
        } catch (e: Exception) {
            project.logger.lifecycle("[distribution:docker] Assuming Docker isn't available on system, original exception:", e)
        }
    }
}
