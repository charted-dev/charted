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
import com.github.ajalt.clikt.parameters.options.default
import com.github.ajalt.clikt.parameters.options.option
import com.github.ajalt.clikt.parameters.options.validate
import com.github.ajalt.mordant.terminal.Terminal
import io.ktor.http.*
import org.koin.core.annotation.KoinInternalApi
import org.koin.core.context.startKoin
import org.noelware.charted.ValidationException
import org.noelware.charted.cli.logger
import org.noelware.charted.modules.openapi.kotlin.dsl.ok
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.modelConverterContext
import org.noelware.charted.modules.openapi.openApi
import org.noelware.charted.modules.openapi.toJson
import org.noelware.charted.modules.openapi.toYaml
import org.noelware.charted.server.routing.configurePathItem
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.routingModule
import kotlin.reflect.full.companionObjectInstance
import kotlin.reflect.jvm.jvmName

class OpenAPICommand(private val terminal: Terminal): CliktCommand(
    "Generates charted's OpenAPI document",
    name = "openapi",
) {
    private val formats = listOf("json", "yaml")
    private val format: String by option(
        "--format", "-f",
        help = "What format the document should be encoded in",
    ).default("json").validate {
        if (!formats.contains(it)) {
            throw ValidationException("--format", "Can't use '$it' as a format, valid values: [${formats.joinToString(", ")}]")
        }
    }

    @OptIn(KoinInternalApi::class)
    override fun run() {
        val koin = startKoin {
            modules(*routingModule.toTypedArray())
        }

        val openApi = openApi {
            server {
                description("Official Instance")
                url("https://charts.noelware.org/api")
            }

            path("/_openapi") {
                description = "Endpoint for the OpenAPI specification for charted-server"
                get {
                    description = "Gets the document in JSON format or YAML format"
                    queryParameter {
                        description = "Format to use"
                        name = "format"

                        schema<String>()
                    }

                    queryParameter {
                        description = "If the document should be pretty or not, this is only applicable to the JSON format"
                        name = "pretty"

                        schema<Boolean>()
                    }
                }
            }

            path("/_swagger") {
                description = "Endpoint for Swagger UI"
                get {
                    ok {
                        contentType(ContentType.Text.Html)
                    }
                }
            }
        }

        val controllers = koin.koin.instanceRegistry.instances.map { it.value.beanDefinition.primaryType }
        for (cls in controllers) {
            terminal.logger.debug("openapi: loading class [${cls.jvmName}]")
            val descriptor = cls.companionObjectInstance as? ResourceDescription

            // We do not want to display this warning since CLI tools might
            // only want the data itself without any warnings, so it's
            // going to be logged at the debug level
            if (descriptor == null) {
                terminal.logger.debug("openapi: class ${cls.jvmName}: skipping due to no [ResourceDescription] being available")
                continue
            }

            terminal.logger.debug("openapi: class ${cls.jvmName}: loaded description - path ${descriptor.path}")
            openApi.configurePathItem(descriptor.path, descriptor.describe())
        }

        for ((name, schema) in modelConverterContext.definedModels.entries.sortedBy { it.key }) {
            openApi.components.addSchemas(name, schema)
        }

        val doc = when (format) {
            "json" -> openApi.toJson(true)
            "yaml" -> openApi.toYaml()
            else -> throw AssertionError("we shouldn't be here")
        }

        println(doc)
        koin.close()
    }
}
