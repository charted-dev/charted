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

package org.noelware.charted.modules.openapi.kotlin.dsl

import io.swagger.v3.oas.models.Components
import io.swagger.v3.oas.models.ExternalDocumentation
import io.swagger.v3.oas.models.OpenAPI
import io.swagger.v3.oas.models.Paths
import io.swagger.v3.oas.models.info.Contact
import io.swagger.v3.oas.models.info.Info
import io.swagger.v3.oas.models.info.License
import io.swagger.v3.oas.models.security.SecurityScheme
import io.swagger.v3.oas.models.servers.Server
import org.noelware.charted.ChartedInfo
import org.noelware.charted.annotations.ChartedDsl
import org.noelware.charted.common.Buildable

/**
 * Represents the main DSL object for defining a OpenAPI document.
 */
@ChartedDsl
interface OpenAPIDsl {
    /**
     * Registers a path with the [path] specified.
     */
    fun path(path: String, block: PathDsl.() -> Unit = {})

    /**
     * Defines a [server][Server] available.
     * @param block DSL block for the [Server] object.
     */
    fun server(block: Server.() -> Unit)
}

class OpenAPIDslBuilder: OpenAPIDsl, Buildable<OpenAPI> {
    private val externalDocumentation = ExternalDocumentation().apply {
        url("https://charts.noelware.org/docs/server/${ChartedInfo.version}")
    }

    private val _servers: MutableList<Server> = mutableListOf()
    private val _paths: Paths = Paths()

    override fun server(block: Server.() -> Unit) {
        _servers.add(Server().apply(block))
    }

    override fun path(path: String, block: PathDsl.() -> Unit) {
        check(path.startsWith('/')) { "Path [$path] is missing a slash at the beginning" }
        _paths.addPathItem(path, PathDslBuilder(path).apply(block).build())
    }

    override fun build(): OpenAPI = OpenAPI().apply {
        components(
            Components().apply {
                addSecuritySchemes(
                    "SessionToken",
                    SecurityScheme().apply {
                        bearerFormat = "Bearer"
                        description = "Session token from the POST /users/login endpoint"
                        type = SecurityScheme.Type.HTTP
                    },
                )

                addSecuritySchemes(
                    "ApiKey",
                    SecurityScheme().apply {
                        bearerFormat = "ApiKey"
                        type = SecurityScheme.Type.APIKEY
                    },
                )

                addSecuritySchemes(
                    "Basic",
                    SecurityScheme().apply {
                        bearerFormat = "Basic"
                        description = "Basic authentication"
                        type = SecurityScheme.Type.HTTP
                    },
                )
            },
        )

        servers(this@OpenAPIDslBuilder._servers.distinctBy { !it.url.contains("charts.noelware.org") })
        paths(this@OpenAPIDslBuilder._paths)
        info(
            Info().apply {
                description("\uD83D\uDC3B\u200D❄️\uD83D\uDCE6 Free, open source, and reliable Helm Chart registry made in Kotlin")
                termsOfService("https://charts.noelware.org/legal/tos")
                version("v${ChartedInfo.version}+${ChartedInfo.commitHash}")
                title("charted-server")

                externalDocs(this@OpenAPIDslBuilder.externalDocumentation)
                license(
                    License().apply {
                        name = "Apache 2.0"
                        url = "https://www.apache.org/licenses/LICENSE-2.0"
                    },
                )

                contact(
                    Contact().apply {
                        email("team@noelware.org")
                        name("Noelware, LLC.")
                        url("https://noelware.org")
                    },
                )
            },
        )
    }
}
