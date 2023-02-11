/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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
import com.github.ajalt.clikt.parameters.options.option
import com.github.ajalt.mordant.terminal.Terminal
import dev.floofy.utils.kotlin.ifNotNull
import guru.zoroark.tegral.openapi.dsl.OpenApiVersion
import guru.zoroark.tegral.openapi.dsl.openApi
import guru.zoroark.tegral.openapi.dsl.toJson
import guru.zoroark.tegral.openapi.dsl.toYaml
import org.noelware.charted.cli.logger
import org.noelware.charted.server.openapi.charted

class OpenAPICommand(private val terminal: Terminal): CliktCommand("Generates the OpenAPI schema without launching the API server", name = "openapi") {
    private val versionOption: String? by option(
        "--openapi-version", "-v",
        help = "The version of OpenAPI to generate to (valid values: 3.0, 3.1)",
    )

    private val formatOption: String? by option(
        "--format", "-f",
        help = "The format (json, yaml) to generate from (default: json)",
    )

    override fun run() {
        val openapi = openApi { charted() }
        val version = versionOption?.ifNotNull {
            OpenApiVersion.values().singleOrNull { it.version == this } ?: OpenApiVersion.V3_0
        } ?: OpenApiVersion.V3_0

        val document = when (formatOption) {
            null, "json" -> openapi.toJson(version)
            "yaml" -> openapi.toYaml(version)
            else -> {
                terminal.logger.warn("Invalid format [$formatOption], defaulting to JSON")
                openapi.toJson(version)
            }
        }

        terminal.println(document)
    }
}
