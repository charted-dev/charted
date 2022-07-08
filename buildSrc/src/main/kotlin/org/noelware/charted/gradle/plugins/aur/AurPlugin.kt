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

package org.noelware.charted.gradle.plugins.aur

import org.gradle.api.GradleException
import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.internal.os.OperatingSystem
import org.gradle.kotlin.dsl.register
import java.io.ByteArrayOutputStream
import java.util.regex.Pattern

class AurPlugin: Plugin<Project> {
    override fun apply(project: Project) {
        project.logger.lifecycle("[distribution:aur] Checking if we are on Linux...")

        val os = OperatingSystem.current()
        if (!os.isLinux) {
            project.logger.lifecycle("[distribution:aur] Disabling tasks due to not being on Linux! ($os)")
            return
        }

        project.logger.lifecycle("[distribution:aur] We are on Linux! Checking if Pacman exists...")
        val stdout = ByteArrayOutputStream()
        val result = project.exec {
            setIgnoreExitValue(true) // we do this ourself

            standardOutput = stdout
            commandLine("pacman", "-V")
        }

        val data = String(stdout.toByteArray())
        if (result.exitValue != 0) {
            project.logger.lifecycle("[aur:distribution] Unable to run 'pacman -V', assuming it's not installed.\n\n[ == STDOUT == ]:\n$data")
            return
        }

        val res = "v(\\d*).(\\d*).(\\d*)".toRegex().toPattern().matcher(data.split("\n")[1])
        if (!res.find()) {
            project.logger.lifecycle("[aur:distribution] Unable to match regex `v(\\d*).(\\d*).(\\d*)` via [${data.split("\n")[1]}]")
            return
        }

        val version = res.group()
        project.logger.lifecycle("[aur:distribution] Using Pacman $version!")
    }
}
