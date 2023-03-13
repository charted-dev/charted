/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

package org.noelware.charted.cli.commands

import com.github.ajalt.clikt.core.CliktCommand
import com.github.ajalt.clikt.parameters.arguments.argument
import com.github.ajalt.clikt.parameters.types.file
import com.github.ajalt.mordant.terminal.Terminal
import org.noelware.charted.cli.logger
import org.noelware.charted.configuration.kotlin.host.KotlinScriptConfigurationHost
import java.io.File

class ValidateKotlinScriptCommand(private val terminal: Terminal): CliktCommand(
    help = "Validates a .charted.kts file",
    name = "kotlin-script",
) {
    private val file: File by argument(
        "dest",
        "Valid file to a .charted.kts file.",
    ).file(
        mustExist = false,
        canBeFile = true,
        canBeDir = false,
        mustBeWritable = false,
        mustBeReadable = false,
        canBeSymlink = true,
    )

    override fun run() {
        if (!file.extension.contains("kts")) {
            terminal.logger.fatal("File [$file] was not a .charted.kts file!")
        }

        try {
            KotlinScriptConfigurationHost.load(file)
            terminal.logger.info("Kotlin Script is valid")
        } catch (e: Exception) {
            terminal.logger.fatal("Kotlin Script was not validated successfully:", e.message!!)
        }
    }
}
