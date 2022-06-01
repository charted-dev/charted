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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.oci.proxy

import dev.floofy.utils.slf4j.logging
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.util.*
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonObject
import org.noelware.charted.common.config.OciProxyConfig
import java.net.URI

// Hop-by-hop headers. These are removed when sent to the backend.
// As of RFC 7230, hop-by-hop headers are required to appear in the
// Connection header field. These are the headers defined by the
// obsoleted RFC 2616 (section 13.5.1) and are used for backward
// compatibility.
//
// https://cs.opensource.google/go/go/+/refs/tags/go1.18.2:src/net/http/httputil/reverseproxy.go;drc=8a56c7742d96c8ef8e8dcecaf3d1c0e9f022f708;l=176
private val HOP_HEADERS = listOf(
    "Connection",
    "Proxy-Connection", // non-standard but still sent by libcurl and rejected by e.g. google
    "Keep-Alive",
    "Proxy-Authenticate",
    "Proxy-Authorization",
    "Te", // canonicalized version of "TE"
    "Trailer", // not Trailers per URL above; https://www.rfc-editor.org/errata_search.php?eid=4522
    "Transfer-Encoding",
    "Upgrade",
)

/**
 * Represents the reverse proxy mechanism for moving requests from **charted-server** to a
 * local Docker registry if the server's configuration has provided to use an OCI registry for
 * repositories rather than using chart-based repositories.
 */
class DockerRegistryReverseProxy(
    private val config: OciProxyConfig,
    private val httpClient: HttpClient,
    private val json: Json
) {
    private val log by logging<DockerRegistryReverseProxy>()

    @OptIn(InternalAPI::class)
    suspend fun reroute(call: ApplicationCall): Boolean {
        val uri = URI.create("http${if (config.ssl) "s://" else "://"}${config.host}:${config.port}${call.request.uri}")
        log.info("Requesting to ${call.request.httpMethod.value} $uri...")

        val resp = httpClient.request {
            url {
                takeFrom(uri)
            }

            method = call.request.httpMethod

            if (call.request.httpMethod.value != "GET" || call.request.httpMethod.value != "HEAD") {
                body = call.receive()
                bodyType = call.receiveType
            }

            for ((header, value) in call.request.headers.toMap()) {
                log.debug("APPENDING HEADER - $header (${value.joinToString(", ")})")

                // Do not append anything from `Connection` header.
                if (header == "Connection")
                    continue

                // If the headers are unsafe headers, then don't include them
                val isUnsafe = HttpHeaders.UnsafeHeadersList.firstOrNull { it.lowercase() == header.lowercase() } != null
                if (isUnsafe) {
                    log.debug("UNSAFE HEADER ACCORDING TO KTOR - $header")
                    continue
                }

                this.header(header, value.first())
            }
        }

        for ((header, value) in resp.headers.toMap()) {
            log.debug("APPENDING HEADER - $header (${value.joinToString(", ")})")

            // Do not append anything from `Connection` header.
            if (header == "Connection")
                continue

            // Do not add the headers that are in the `HOP_HEADERS`
            // list.
            if (HOP_HEADERS.contains(header))
                continue

            // If the headers are unsafe headers, then don't include them
            val isUnsafe = HttpHeaders.UnsafeHeadersList.firstOrNull { it.lowercase() == header.lowercase() } != null
            if (isUnsafe) {
                log.debug("UNSAFE HEADER ACCORDING TO KTOR - $header")
                continue
            }

            call.response.header(header, value.first())
        }

        val body = resp.body<String>()
        if (body.startsWith('{') && body.endsWith('}')) {
            val data = json.decodeFromString(JsonObject.serializer(), body)
            call.respond(resp.status, data)
        } else {
            call.respond(resp.status, body)
        }

        return false
    }
}
