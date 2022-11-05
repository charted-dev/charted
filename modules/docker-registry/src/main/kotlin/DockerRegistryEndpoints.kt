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
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

@file:Suppress("UNUSED")

package org.noelware.charted.modules.docker.registry

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.modules.docker.registry.headers.DockerRegistryHeadersBuilder
import org.noelware.charted.modules.docker.registry.tokens.RegistryServiceTokenManager
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Get

class DockerRegistryEndpoints(
    private val serviceTokens: RegistryServiceTokenManager,
    private val registry: DockerRegistry,
    private val config: Config
): AbstractEndpoint("/v2") {
    init {
        val headerPlugin = createRouteScopedPlugin("ChartedRegistryAuthPlugin") {
            onCall { call ->
                val auth = call.request.header(HttpHeaders.Authorization)
                if (auth == null) {
                    val serverUrl = if (config.baseUrl != null) {
                        config.baseUrl
                    } else {
                        "http${if (config.server.ssl != null) "s" else ""}://${config.server.host}:${config.server.port}"
                    }

                    call.response.header("WWW-Authenticate", "Bearer realm=\"$serverUrl/v2/token\",service=\"noelware_charts_registry\",scope=\"pull,push\"")
                    val exception = DockerRegistryException(RegistryErrorCode.UNAUTHORIZED)

                    call.respond(exception.status, exception.toApiError())
                }
            }
        }

        install(headerPlugin)
    }

    @Get
    suspend fun main(call: ApplicationCall) {
        call.doRespond("")
    }
}

private // Helper method to add the headers we need.
suspend inline fun <reified T: Any> ApplicationCall.doRespond(
    data: T,
    status: HttpStatusCode = HttpStatusCode.OK,
    builder: DockerRegistryHeadersBuilder.() -> Unit = {}
) {
    val headers = DockerRegistryHeadersBuilder().apply(builder).build()
    if (!isHandled) {
        if (headers.range != null) {
            response.header("Range", headers.range)
        }

        if (headers.uploadUUID != null) {
            response.header("Docker-Upload-Uuid", headers.uploadUUID)
        }

        if (headers.contentDigest != null) {
            response.header("Docker-Content-Digest", headers.contentDigest)
            response.header("ETag", "\"${headers.contentDigest}\"")
        }

        response.header("Docker-Distribution-Api-Version", "registry/2.0")
    }

    respond(status, data)
}
