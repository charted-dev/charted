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

package org.noelware.charted.server.routing

import io.ktor.http.*
import io.swagger.v3.oas.models.OpenAPI
import io.swagger.v3.oas.models.PathItem
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.toApiBaseUrl
import org.noelware.charted.modules.openapi.kotlin.dsl.ok
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.modelConverterContext
import org.noelware.charted.modules.openapi.openApi

fun generateOpenAPIDocument(
    config: Config,
    controllers: List<RestController>,
    includeConfiguredServer: Boolean = true
): OpenAPI {
    val openApi = openApi {
        server {
            description("Official Instance")
            url("https://charts.noelware.org/api")
        }

        if (includeConfiguredServer) {
            server {
                description("Main Instance")
                url(config.toApiBaseUrl().trimEnd('/'))
            }
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
                    description = "Only applicable to `?format=json` -- if the document should be pretty or not"
                    name = "pretty"

                    schema<Boolean>()
                }
            }
        }

        if (config.swagger) {
            path("/_swagger") {
                description = "Endpoint for Swagger UI"
                get {
                    ok {
                        contentType(ContentType.Text.Html)
                    }
                }
            }
        }
    }

    for (controller in controllers) {
        val dsl = try {
            controller.toPathDsl()
        } catch (e: NotImplementedError) {
            null
        } ?: continue

        if (APIVersion.default() == controller.apiVersion) {
            openApi.configurePathItem(controller.path, dsl)
        }

        openApi.configurePathItem("${controller.apiVersion.toRoutePath()}${controller.path}".trimEnd('/'), dsl)
    }

    for ((name, schema) in modelConverterContext.definedModels.entries.sortedBy { it.key }) {
        openApi.components.addSchemas(name, schema)
    }

    return openApi
}

private fun OpenAPI.configurePathItem(path: String, newPathItem: PathItem) {
    val actualPath = if (path.endsWith("?}")) path.substring(0, path.indexOf("?}")) + "}" else path
    if (paths.containsKey(actualPath)) {
        val pathItem = paths[actualPath]!!
        if (newPathItem.get != null && pathItem.get == null) {
            pathItem.get(newPathItem.get!!)
        }

        if (newPathItem.put != null && pathItem.put == null) {
            pathItem.put(newPathItem.put!!)
        }

        if (newPathItem.head != null && pathItem.head == null) {
            pathItem.head(newPathItem.head!!)
        }

        if (newPathItem.post != null && pathItem.post == null) {
            pathItem.post(newPathItem.post!!)
        }

        if (newPathItem.patch != null && pathItem.patch == null) {
            pathItem.patch(newPathItem.patch!!)
        }

        if (newPathItem.delete != null && pathItem.delete == null) {
            pathItem.delete(newPathItem.delete!!)
        }

        paths.addPathItem(actualPath, pathItem)
    } else {
        paths.addPathItem(actualPath, newPathItem)
    }
}
