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

package org.noelware.charted.search.meili

import com.meilisearch.sdk.Client
import com.meilisearch.sdk.Config
import com.meilisearch.sdk.SearchRequest
import dev.floofy.utils.slf4j.logging
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import io.ktor.http.*
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.jsonPrimitive
import kotlinx.serialization.json.put

/**
 * Represents the backend for using Meilisearch rather than Elasticsearch.
 */
class MeilisearchBackend(config: MeilisearchConfig, httpClient: HttpClient) {
    val client: Client
    val healthy: Boolean
    val serverVersion: String

    private val log by logging<MeilisearchBackend>()

    init {
        log.info("Initializing Meilisearch backend...")

        val cfg = if (config.masterKey != null) Config(
            config.endpoint.let {
                if (!config.endpoint.startsWith("http:"))
                    return@let "http://$it"

                it
            },
            config.masterKey
        ) else Config(
            config.endpoint.let {
                if (!config.endpoint.startsWith("http:"))
                    return@let "http://$it"

                it
            }
        )

        client = Client(cfg)

        val health = runBlocking {
            httpClient.get("http://${config.endpoint}/health") {
                if (config.masterKey != null) {
                    header("Authorization", "Bearer ${config.masterKey}")
                }
            }
        }

        log.debug("RESPONSE FOR /health: ${health.status.value} ${health.status.description}")

        if (!health.status.isSuccess())
            throw IllegalStateException("Meilisearch instance is not healthy, not continuing...")

        val status = runBlocking {
            health.body<JsonObject>()
        }["status"]!!.jsonPrimitive.content

        healthy = if (status != "available") {
            log.warn("Meilisearch instance is not healthy, search endpoint will be unavailable.")
            false
        } else {
            log.info("Meilisearch instance is healthy!")
            true
        }

        val version = runBlocking {
            val res = httpClient.get("http://${config.endpoint}/version") {
                if (config.masterKey != null) {
                    header("Authorization", "Bearer ${config.masterKey}")
                }
            }

            res.body<JsonObject>()
        }

        val pkgVersion = version["pkgVersion"]!!.jsonPrimitive.content
        val commitSha = version["commitSha"]!!.jsonPrimitive.content

        log.info("Meilisearch instance is running v$pkgVersion${if (commitSha != "unknown") " ($commitSha)" else ""} of Meilisearch!")
        serverVersion = "$pkgVersion${if (commitSha != "unknown") " ($commitSha)" else ""}"

        runBlocking { setup() }
    }

    private suspend fun setup() {
        log.debug("Setting up Meilisearch for indices...")

        val indexes = listOf(
            Indexes.ORGANIZATION.index,
            Indexes.ORGANIZATION_MEMBER.index,
            Indexes.REPOSITORY.index,
            Indexes.REPOSITORY_MEMBER.index,
            Indexes.USER.index
        )

        val allIndexes = client.indexList
        for (index in indexes) {
            log.debug("Does index $index exist...?")

            val idx = allIndexes.firstOrNull { it.uid == index }
            if (idx == null) {
                log.debug("Index $index doesn't exist! Creating...")

                val i = client.createIndex(index, "id")
                log.info("Task ${i.uid} was posted. Current status => ${i.status} (duration=>${i.duration}, type=>${i.type})")
            } else {
                log.debug("Index $index already exists!")
            }
        }
    }

    suspend fun indexAllData() {
    }

    fun search(
        index: Indexes,
        query: String,
        limit: Int = 25,
        offset: Int = 0,
        filters: List<String> = listOf(),
        facets: List<String> = listOf(),
        attributes: List<String> = listOf(),
        sort: List<String> = listOf()
    ): JsonObject {
        val i = client.index(index.index)
        val request = SearchRequest().apply {
            q = query

            this.offset = offset
            this.limit = limit

            if (filters.isNotEmpty()) {
                filter = filters.toTypedArray()
            }

            if (facets.isNotEmpty()) {
                facetsDistribution = facets.toTypedArray()
            }

            if (attributes.isNotEmpty()) {
                attributesToRetrieve = attributes.toTypedArray()
            }

            if (sort.isNotEmpty()) {
                this.sort = sort.toTypedArray()
            }
        }

        val res = i.search(request)
        return buildJsonObject {
            put("offset", offset)
            put("limit", limit)
            put("hits", res.hits.toJsonArray())
            put("took", res.processingTimeMs)
        }
    }
}
