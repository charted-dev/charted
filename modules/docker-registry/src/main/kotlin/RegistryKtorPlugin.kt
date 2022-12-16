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

package org.noelware.charted.modules.docker.registry

import dev.floofy.utils.koin.inject
import dev.floofy.utils.slf4j.logging
import io.ktor.client.*
import io.ktor.client.request.*
import io.ktor.content.*
import io.ktor.content.ByteArrayContent
import io.ktor.http.*
import io.ktor.http.content.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.util.*
import io.ktor.utils.io.*
import io.sentry.Sentry
import org.apache.commons.lang3.time.StopWatch
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.extensions.doFormatTime
import org.noelware.charted.extensions.ifSentryEnabled
import org.noelware.charted.types.responses.ApiResponse

private val NO_BODY_CONTENT: List<HttpMethod> = listOf(HttpMethod.Get, HttpMethod.Head)

@OptIn(InternalAPI::class)
val RegistryKtorPlugin = createApplicationPlugin("RegistryKtorPlugin") {
    val httpClient: HttpClient by inject()
    val config: Config by inject()
    val log by logging("org.noelware.charted.modules.docker.registry.KtorPlugin")

    onCall { call ->
        // Proceed with the call anyway if it's not a /v2/ endpoint call.
        if (!call.request.path().contains("/v2/")) {
            return@onCall
        }

        // Get the receive channel, where the (possible) HTTP request
        // body is received from.
        val channel = call.request.receiveChannel()

        // Now, we need to allocate an array of bytes of what we can store
        // from the request call given by us
        val size = channel.availableForRead
        val byteArray = ByteArray(size)

        // Now, let's fully read it
        channel.readFully(byteArray, 0, size)
        log.debug("Received ${byteArray.size} bytes to send out to local registry server!")

        // Now, we do the actual HTTP request call~!
        val sw = StopWatch.createStarted()
        try {
            val method = call.request.httpMethod
            val endpoint = "http://${config.dockerRegistry!!.host}:${config.dockerRegistry!!.port}"
            val resp = httpClient.request(endpoint) {
                this.method = method
                headers {
                    appendAll(call.request.headers)
                    appendAll(
                        Headers.build {
                            for ((key, value) in config.dockerRegistry!!.headers) {
                                append(key, value)
                            }
                        }
                    )
                }

                if (!NO_BODY_CONTENT.contains(method)) {
                    setBody(ByteArrayContent(byteArray, call.request.contentType()))
                }
            }

            log.info("Got sent back ${resp.status.value} ${resp.status.description} on proxied request [${method.value} $endpoint]")
            if (call.request.httpMethod == HttpMethod.Head) {
                for ((key, value) in resp.headers.entries()) {
                    call.response.headers.append(key, value.first())
                }

                call.respond(resp.status)
                return@onCall
            }

            call.respond(object: OutgoingContent.WriteChannelContent() {
                override val contentLength: Long? = resp.headers[HttpHeaders.ContentLength]?.toLong()
                override val contentType: ContentType? = resp.headers[HttpHeaders.ContentType]?.let { ContentType.parse(it) }
                override val headers: Headers = resp.headers
                override val status: HttpStatusCode = resp.status

                override suspend fun writeTo(channel: ByteWriteChannel) {
                    resp.content.copyAndClose(channel)
                }
            })
        } catch (e: Exception) {
            sw.stop()
            ifSentryEnabled { Sentry.captureException(e) }
            log.error("Unable to handle Docker Registry call in ${sw.doFormatTime()} [${call.request.httpMethod.value} ${call.request.path()}]:", e)

            call.respond(HttpStatusCode.InternalServerError, ApiResponse.err(e))
        }
    }
}
