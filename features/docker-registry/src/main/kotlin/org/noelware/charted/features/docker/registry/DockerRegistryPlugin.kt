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

package org.noelware.charted.features.docker.registry

import dev.floofy.utils.koin.inject
import dev.floofy.utils.slf4j.logging
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import io.ktor.content.ByteArrayContent
import io.ktor.http.*
import io.ktor.http.content.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.util.*
import io.ktor.utils.io.*
import kotlinx.serialization.json.*
import org.slf4j.Logger
import java.util.*

@OptIn(InternalAPI::class)
private suspend fun HttpClient.doRequest(
    call: ApplicationCall,
    host: String,
    port: Int,
    ssl: Boolean,
    log: Logger,
    data: ByteArray,
    headerBuilder: HeadersBuilder.() -> Unit = {}
): OutgoingContent? {
    val noBodyMethods = listOf(HttpMethod.Get, HttpMethod.Head)
    val method = call.request.httpMethod
    val endpoint = "http${if (ssl) "s" else ""}://$host:$port${call.request.uri}"

    log.info("Proxying request from us -> Local Docker Registry [${method.value} $endpoint]")
    val response = request(endpoint) {
        this.method = method
        headers(headerBuilder)

        if (!noBodyMethods.contains(call.request.httpMethod)) {
            setBody(ByteArrayContent(data, call.request.contentType()))
        }
    }

    log.info("Received ${response.status} on proxied request [${method.value} $endpoint]")
    if (call.request.httpMethod == HttpMethod.Head) {
        val proxiedHeaders = response.headers
        for ((key, value) in proxiedHeaders.entries()) {
            if (key.lowercase().startsWith("content-type")) continue
            if (key.lowercase().startsWith("content-length")) continue

            // TODO: what to do if there is more than one?
            call.response.headers.append(key, value.first())
        }

        call.respond(response.status)
        return null
    }

    val proxiedHeaders = response.headers
    return object: OutgoingContent.WriteChannelContent() {
        override val contentLength: Long? = proxiedHeaders[HttpHeaders.ContentLength]?.toLong()
        override val contentType: ContentType? = proxiedHeaders[HttpHeaders.ContentType]?.let { ContentType.parse(it) }
        override val headers: Headers = Headers.build {
            appendAll(
                proxiedHeaders.filter { key, _ ->
                    if (key.lowercase().startsWith("content-type")) return@filter false
                    if (key.lowercase().startsWith("content-length")) return@filter false

                    true
                }
            )
        }

        override val status: HttpStatusCode = response.status
        override suspend fun writeTo(channel: ByteWriteChannel) {
            response.content.copyAndClose(channel)
        }
    }
}

@OptIn(InternalAPI::class)
val DockerRegistryPlugin = createApplicationPlugin("DockerRegistryPlugin", ::RegistryConfig) {
    val httpClient: HttpClient by inject()
    val log by logging("org.noelware.charted.features.docker.registry.DockerRegistryPlugin")

    log.info("Initialized the Docker Registry Ktor plugin!")
    onCall { call ->
        if (!call.request.uri.contains("/v2")) {
            return@onCall
        }

        log.info("Proxying request to [${call.request.httpMethod} http${if (pluginConfig.ssl) "s" else ""}://${pluginConfig.host}:${pluginConfig.port}${call.request.uri}]")
        val channel = call.request.receiveChannel()
        val size = channel.availableForRead
        val byteArray = ByteArray(size)
        channel.readFully(byteArray, 0, size)
        log.debug("Read body channel! received [${byteArray.size}] bytes")

        try {
            val content = httpClient.doRequest(
                call,
                pluginConfig.host,
                pluginConfig.port,
                pluginConfig.ssl,
                log,
                byteArray
            ) {
                appendAll(
                    call.request.headers.filter { key, _ ->
                        if (key.equals(HttpHeaders.ContentType, ignoreCase = true)) return@filter false
                        if (key.equals(HttpHeaders.ContentLength, ignoreCase = true)) return@filter false
                        if (key.equals(HttpHeaders.Host, ignoreCase = true)) return@filter false

                        true
                    }
                )
            } ?: return@onCall

            call.respond(content)
        } catch (e: Exception) {
            log.error("Unable to request to ${call.request.httpMethod} http${if (pluginConfig.ssl) "s" else ""}://${pluginConfig.host}:${pluginConfig.port}${call.request.uri}:", e)
            call.respond(
                HttpStatusCode.InternalServerError,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("message", "Unable to proxy to Docker Registry.")
                            put("code", "UNABLE_TO_PROXY")
                        }
                    }
                }
            )
        }
    }
}
