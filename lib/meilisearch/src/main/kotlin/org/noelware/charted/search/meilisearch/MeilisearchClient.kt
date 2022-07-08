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

package org.noelware.charted.search.meilisearch

import dev.floofy.meilisearch.rest.RESTClient
import dev.floofy.utils.slf4j.logging
import io.ktor.client.*
import io.ktor.http.*
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.jsonPrimitive
import org.noelware.charted.common.SetOnceGetValue
import org.noelware.charted.common.data.MeilisearchConfig
import org.noelware.charted.common.extensions.measureTime

class MeilisearchClient(httpClient: HttpClient, config: MeilisearchConfig) {
    private val _serverVersion: SetOnceGetValue<String> = SetOnceGetValue()
    private val _client: SetOnceGetValue<RESTClient> = SetOnceGetValue()
    private val log by logging<MeilisearchClient>()

    val serverVersion: String
        get() = _serverVersion.value

    init {
        log.info("Initializing Meilisearch client...")
        _client.value = RESTClient {
            endpoint = config.endpoint
            apiKey = config.masterKey

            useHttpClient(httpClient)
        }

        log.info("Checking if Meilisearch is healthy...")
        log.measureTime("Received Meilisearch health in %T") {
            runBlocking {
                val health: JsonObject = _client.value.requestHandler.request(HttpMethod.Get, "/health")
                val status = health["status"]?.jsonPrimitive?.content
                    ?: throw IllegalStateException("Missing `status` attribute")

                if (status != "available") {
                    throw IllegalStateException("Meilisearch is not available. Please wait a bit and re-run charted-server again.")
                }

                val version = _client.value.version()
                _serverVersion.value = version.pkgVersion
            }
        }
    }
}
