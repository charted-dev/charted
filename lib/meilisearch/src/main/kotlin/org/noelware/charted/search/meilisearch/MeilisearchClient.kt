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
import dev.floofy.meilisearch.rest.task.waitForCompletion
import dev.floofy.utils.exposed.asyncTransaction
import dev.floofy.utils.slf4j.logging
import io.ktor.client.*
import io.ktor.http.*
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.jsonPrimitive
import kotlinx.serialization.json.put
import org.jetbrains.exposed.sql.selectAll
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.SetOnceGetValue
import org.noelware.charted.common.data.MeilisearchConfig
import org.noelware.charted.common.extensions.measureSuspendTime
import org.noelware.charted.common.extensions.measureTime
import org.noelware.charted.database.tables.UserTable

private val INDEXES = listOf(
    "charted-users",
    "charted-repositories",
    "charted-organizations"
)

class MeilisearchClient(httpClient: HttpClient, config: MeilisearchConfig) {
    private val _serverVersion: SetOnceGetValue<String> = SetOnceGetValue()
    private val _client: SetOnceGetValue<RESTClient> = SetOnceGetValue()
    private val log by logging<MeilisearchClient>()

    val serverVersion: String
        get() = _serverVersion.value

    val client: RESTClient
        get() = _client.value

    init {
        log.info("Initializing Meilisearch client...")
        _client.value = RESTClient {
            endpoint = config.endpoint
            apiKey = config.masterKey

            useHttpClient(httpClient)
        }

        log.info("Checking if Meilisearch is healthy...")
        log.measureSuspendTime("Received Meilisearch health in %T") {
            val health: JsonObject = client.requestHandler.request(HttpMethod.Get, "/health")
            val status = health["status"]?.jsonPrimitive?.content
                ?: throw IllegalStateException("Missing `status` attribute")

            if (status != "available") {
                throw IllegalStateException("Meilisearch is not available. Please wait a bit and re-run charted-server again.")
            }

            val version = client.version()
            _serverVersion.value = version.pkgVersion
        }

        log.measureSuspendTime("Created/updated indexes and indexed data to Meilisearch in %T") {
            createIfNotExists()
            indexData()
        }
    }

    private suspend fun createIfNotExists() {
        for (index in INDEXES) {
            log.debug("├── Does index $index exist?")

            val exists = client.index(index)
            if (exists == null) {
                log.debug("│   ├── It doesn't exist! Creating...")

                val task = client.createIndex(index, "id")
                log.measureSuspendTime("│   ├── Took %T to await task #${task.uid}") {
                    task.waitForCompletion(ChartedScope, client = client)
                }
            } else {
                log.debug("│   ├──  Index [$index] already exists~")
            }
        }
    }

    private suspend fun indexData() {
        log.debug("Indexing data from PostgreSQL...")

        for (index in INDEXES) {
            log.measureSuspendTime("Took %T to index all data for [$index]") {
                when (index) {
                    "charted-users" -> {
                        val users = asyncTransaction(ChartedScope) {
                            UserTable.selectAll().toList()
                        }

                        if (users.isEmpty()) {
                            log.warn("Skipping index [$index] since there is no data.")
                            return@measureSuspendTime
                        }

                        for (user in users) {
                            val data = buildJsonObject {
                                put("description", user[UserTable.description])
                                put("username", user[UserTable.username])
                                put("name", user[UserTable.name])
                                put("id", user[UserTable.id].value)
                            }

                            val task = client.createOrReplaceDocuments(index, listOf(data), "id")
                            task.waitForCompletion(ChartedScope, client = client)
                        }
                    }
                }
            }
        }
    }
}
