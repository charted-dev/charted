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

import com.charleskorn.kaml.SequenceStyle
import com.charleskorn.kaml.Yaml
import com.charleskorn.kaml.YamlConfiguration
import com.charleskorn.kaml.encodeToStream
import com.github.ajalt.clikt.core.CliktCommand
import com.github.ajalt.clikt.parameters.arguments.argument
import com.github.ajalt.clikt.parameters.options.flag
import com.github.ajalt.clikt.parameters.options.option
import com.github.ajalt.clikt.parameters.types.file
import com.github.ajalt.mordant.terminal.Terminal
import kotlinx.serialization.modules.EmptySerializersModule
import org.noelware.charted.cli.logger
import org.noelware.charted.configuration.kotlin.dsl.Config
import java.io.File
import kotlin.system.exitProcess

class GenerateConfigCommand(private val terminal: Terminal): CliktCommand(
    name = "generate-config",
    help = "Generates a configuration file in the specified destination",
) {
    private val dryRun: Boolean by option(
        help = "If the configuration file shouldn't be written, it'll just print to stdout",
    ).flag("--dry-run", "-d")

    private val dest: File by argument(
        "dest",
        "Destination file to write to",
    ).file(
        mustExist = false,
        canBeFile = true,
        canBeDir = false,
        mustBeWritable = false,
        mustBeReadable = false,
        canBeSymlink = false,
    )

    override fun run() {
        if (dest.exists()) {
            terminal.logger.warn("Destination file [$dest] already exists")
            exitProcess(0)
        }

        val config = Config()
        val yaml = Yaml(
            EmptySerializersModule(),
            YamlConfiguration(
                encodeDefaults = false,
                strictMode = true,
                encodingIndentationSize = 4,
                sequenceStyle = SequenceStyle.Block,
                sequenceBlockIndent = 4,
            ),
        )

        dest.outputStream().use { stream -> yaml.encodeToStream(config, stream) }
        terminal.logger.info("Wrote configuration file in [$dest]")
    }
}
