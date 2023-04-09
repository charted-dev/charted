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

package org.noelware.charted.cli.commands.abstractions

import com.charleskorn.kaml.SequenceStyle
import com.charleskorn.kaml.Yaml
import com.charleskorn.kaml.YamlConfiguration
import com.github.ajalt.clikt.core.CliktCommand
import com.github.ajalt.clikt.parameters.options.option
import com.github.ajalt.clikt.parameters.types.file
import kotlinx.serialization.encodeToString
import kotlinx.serialization.modules.EmptySerializersModule
import org.noelware.charted.configuration.kotlin.dsl.Config
import java.io.File

/**
 * Represents a [CliktCommand] that exposes a [config] property to get
 * the current configuration, if needed.
 */
abstract class ConfigAwareCliktCommand(
    help: String = "",
    epilog: String = "",
    name: String? = null,
    invokeWithoutSubcommand: Boolean = false,
    printHelpOnEmptyArgs: Boolean = false,
    helpTags: Map<String, String> = emptyMap(),
    autoCompleteEnvvar: String? = "",
    allowMultipleSubcommands: Boolean = false,
    treatUnknownOptionsAsArgs: Boolean = false,
    hidden: Boolean = false
): CliktCommand(
    help,
    epilog,
    name,
    invokeWithoutSubcommand,
    printHelpOnEmptyArgs,
    helpTags,
    autoCompleteEnvvar,
    allowMultipleSubcommands,
    treatUnknownOptionsAsArgs,
    hidden,
) {
    private val yaml = Yaml(
        EmptySerializersModule(),
        YamlConfiguration(
            encodeDefaults = false,
            sequenceStyle = SequenceStyle.Block,
            sequenceBlockIndent = 4,
        ),
    )

    // canonical locations that *might* have what we're looking for
    private val rootConfigFile = File("./config.yml")
    private val configDir = File("./config")
    private val chartedKtsScriptFile = File(configDir, "config.charted.kts")
    private val chartedConfigYamlFile = File(configDir, "charted.yaml")

    @Suppress("MemberVisibilityCanBePrivate")
    internal val config: File? by option(
        "--config", "-c",
        help = "The configuration path to use",
        envvar = "CHARTED_CONFIG_PATH",
    ).file(
        mustExist = false,
        canBeFile = true,
        canBeDir = false,
        mustBeWritable = false,
        mustBeReadable = true,
        canBeSymlink = true,
    )

    fun resolveConfigFile(): File = when {
        config != null -> config!!
        rootConfigFile.exists() && rootConfigFile.isFile -> rootConfigFile
        configDir.exists() && chartedConfigYamlFile.exists() -> chartedConfigYamlFile
        configDir.exists() && chartedKtsScriptFile.exists() -> chartedKtsScriptFile
        else -> {
            if (!configDir.exists()) {
                configDir.mkdirs()
            }

            val config = Config()
            chartedConfigYamlFile.writeText(yaml.encodeToString(config))
            chartedConfigYamlFile
        }
    }
}
