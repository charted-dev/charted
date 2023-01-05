/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

import com.charleskorn.kaml.Yaml
import com.charleskorn.kaml.YamlConfiguration
import com.github.ajalt.clikt.core.CliktCommand
import com.github.ajalt.clikt.parameters.arguments.argument
import com.github.ajalt.clikt.parameters.types.file
import com.github.ajalt.mordant.terminal.Terminal
import kotlinx.serialization.encodeToString
import kotlinx.serialization.modules.EmptySerializersModule
import org.noelware.charted.RandomStringGenerator
import org.noelware.charted.cli.logger
import org.noelware.charted.configuration.kotlin.dsl.Config
import java.io.File
import java.nio.charset.Charset

class GenerateConfigCommand(private val terminal: Terminal): CliktCommand(
    "Generates a configuration file in the specified [DEST]",
    name = "generate",
    invokeWithoutSubcommand = true,
) {
    private val dest: File by argument("dest")
        .file(
            mustExist = false,
            canBeFile = true,
            canBeDir = false,
            mustBeWritable = false,
            mustBeReadable = false,
            canBeSymlink = false,
        )

    override fun run() {
        terminal.logger.info("Writing configuration file in $dest...")
        if (dest.exists()) terminal.logger.fatal("File $dest already exists")

        val secretKey = RandomStringGenerator.generate(64)
        val config = Config(jwtSecretKey = secretKey)

        val yaml = Yaml(
            EmptySerializersModule(),
            YamlConfiguration(encodeDefaults = true),
        )

        if (!dest.createNewFile()) {
            terminal.logger.fatal("Unable to create new file since it was already created?")
        }

        val contents = yaml.encodeToString(config)
        dest.writeText(contents, Charset.defaultCharset())

        terminal.logger.info("Wrote default configuration in file [$dest]")
    }
}
