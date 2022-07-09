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

package org.noelware.charted.search.elasticsearch

import dev.floofy.utils.exposed.asyncTransaction
import dev.floofy.utils.koin.inject
import dev.floofy.utils.slf4j.logging
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withContext
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.*
import okhttp3.internal.closeQuietly
import org.apache.http.HttpHost
import org.apache.http.auth.AuthScope
import org.apache.http.auth.UsernamePasswordCredentials
import org.apache.http.entity.ByteArrayEntity
import org.apache.http.entity.ContentType
import org.apache.http.impl.client.BasicCredentialsProvider
import org.apache.http.ssl.SSLContexts
import org.elasticsearch.client.Node
import org.elasticsearch.client.Request
import org.elasticsearch.client.RestClient
import org.elasticsearch.client.sniff.SniffOnFailureListener
import org.elasticsearch.client.sniff.Sniffer
import org.jetbrains.exposed.sql.selectAll
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.SetOnceGetValue
import org.noelware.charted.common.data.Config
import org.noelware.charted.common.data.ElasticsearchConfig
import org.noelware.charted.common.data.Feature
import org.noelware.charted.common.extensions.measureSuspendTime
import org.noelware.charted.common.extensions.measureTime
import org.noelware.charted.database.tables.UserTable
import org.noelware.charted.search.elasticsearch.index.Index
import java.io.ByteArrayOutputStream
import java.io.Closeable
import java.nio.file.Files
import java.nio.file.Paths
import java.security.KeyStore
import java.security.cert.CertificateFactory
import java.util.concurrent.atomic.AtomicBoolean

@OptIn(ExperimentalSerializationApi::class)
class ElasticsearchClient(
    private val config: ElasticsearchConfig,
    private val json: Json
): Closeable {
    private val _serverVersion: SetOnceGetValue<String> = SetOnceGetValue()
    private val _clusterInfo: SetOnceGetValue<Pair<String, String>> = SetOnceGetValue() // Pair<Name, UUID>
    private val _closed: AtomicBoolean = AtomicBoolean(false)
    private val _client: SetOnceGetValue<RestClient> = SetOnceGetValue()
    private val log by logging<ElasticsearchClient>()

    val serverVersion: String
        get() = _serverVersion.value

    val clusterName: String
        get() = _clusterInfo.value.first

    val clusterUUID: String
        get() = _clusterInfo.value.second

    val closed: Boolean
        get() = _closed.get()

    val client: RestClient
        get() = _client.value

    fun connect() {
        if (closed) return

        log.debug("Creating connection to Elasticsearch with nodes [${config.nodes.joinToString(", ")}]")
        val builder = if (config.cloudId != null) {
            RestClient.builder(config.cloudId)
        } else {
            RestClient.builder(
                *config.nodes.map { val (host, port) = it.split(":"); HttpHost(host, port.toInt()) }.toTypedArray()
            )
        }

        val listener = SniffOnFailureListener()
        builder.setFailureListener(object: RestClient.FailureListener() {
            override fun onFailure(node: Node) {
                val attributes = if (node.attributes != null) "[${node.attributes.map { "${it.key}=${it.value}" }.joinToString(", ")}" else "[none]"
                log.warn("Elasticsearch node ${node.host} [${node.name} v${node.version}] $attributes has failed on executing an action, sniffing for other nodes...")

                listener.onFailure(node)
            }
        })

        if (config.clientSslPath != null) {
            log.debug("Received client SSL path at [${config.clientSslPath}]")

            val sslPath = Paths.get(config.clientSslPath!!)
            val sslFactory = CertificateFactory.getInstance("X.509")
            val trustedCa = Files.newInputStream(sslPath).use { sslFactory.generateCertificate(it) }
            val trustStore = KeyStore.getInstance("pkcs12")
            trustStore.load(null, null)
            trustStore.setCertificateEntry("ca", trustedCa)

            val sslContext = SSLContexts.custom()
                .loadKeyMaterial(trustStore, null)
                .build()

            builder.setHttpClientConfigCallback {
                it.setSSLContext(sslContext)
                if (config.auth != null) {
                    val provider = BasicCredentialsProvider()
                    provider.setCredentials(
                        AuthScope.ANY,
                        UsernamePasswordCredentials(config.auth!!.username, config.auth!!.password)
                    )

                    it.setDefaultCredentialsProvider(provider)
                }

                it
            }
        } else {
            builder.setHttpClientConfigCallback {
                if (config.auth != null) {
                    val provider = BasicCredentialsProvider()
                    provider.setCredentials(
                        AuthScope.ANY,
                        UsernamePasswordCredentials(config.auth!!.username, config.auth!!.password)
                    )

                    it.setDefaultCredentialsProvider(provider)
                }

                it
            }
        }

        _client.value = builder.build()
        val sniffer = Sniffer.builder(client).build()
        listener.setSniffer(sniffer)

        log.debug("Successfully created the Elasticsearch REST client! Checking if we can request to it...")
        log.measureTime("Took %T to connect to Elasticsearch.") {
            val info = client.performRequest(Request("GET", "/"))
            val data = json.decodeFromStream<JsonObject>(info.entity.content)
            val clusterName = data["cluster_name"]?.jsonPrimitive?.contentOrNull
            val clusterID = data["cluster_uuid"]?.jsonPrimitive?.contentOrNull
            val version = data["version"]?.jsonObject?.get("number")?.jsonPrimitive?.contentOrNull

            if (version == null && clusterName == null && clusterID == null) {
                throw IllegalStateException("Couldn't get cluster information in %T")
            }

            // TODO: check if we are on 8.x
            log.debug("Reached to cluster $clusterName [$clusterID], using v$version.")
            _clusterInfo.value = Pair(clusterName!!, clusterID!!)
            _serverVersion.value = version!!
        }

        log.measureTime("Took %T to create indexes and to index all data!") {
            createIndexes()
            runBlocking {
                indexData()
            }
        }
    }

    override fun close() {
        if (closed) return

        log.measureTime("Closed REST client in %T") {
            client.closeQuietly()
        }
    }

    fun search(
        query: String,
        limit: Int = 25,
        offset: Int = 0
    ): JsonObject {
        val request = Request("POST", "/charted-*/_search")
        val searchQuery = buildJsonObject {
            put("from", offset)
            put("size", limit)
            putJsonObject("query") {
                putJsonObject("query_string") {
                    put("query", query)
                }
            }
        }

        request.setJsonEntity(json.encodeToString(JsonObject.serializer(), searchQuery))
        val res = client.performRequest(request)
        if (res.statusLine.statusCode !in 200..300) {
            val payload = json.decodeFromStream(JsonObject.serializer(), res.entity.content)
            throw IllegalStateException("Unable to request to POST /charted-/_search: $payload")
        }

        val payload = json.decodeFromStream(JsonObject.serializer(), res.entity.content)
        val hits = payload["hits"]!!.jsonObject
        return buildJsonObject {
            put("took", payload["took"]!!.jsonPrimitive)
            put("max_score", hits["max_score"]!!.jsonPrimitive)
            put("total", hits["total"]!!.jsonObject["value"]!!)
            put(
                "hits",
                JsonArray(
                    hits["hits"]!!.jsonArray.map {
                        buildJsonObject {
                            put("index", it.jsonObject["_index"]!!.jsonPrimitive)
                            put("id", it.jsonObject["_id"]!!.jsonPrimitive)

                            // this is probably bad in scale but shrug
                            val source = it.jsonObject["_source"]!!.jsonObject.toMap()
                            for ((key, value) in source)
                                put(key, value)
                        }
                    }
                )
            )

            put("source", "Elasticsearch v$serverVersion")
        }
    }

    private fun createIndexes() {
        val config by inject<Config>()
        val featureIndexes = mutableListOf<Index>()

        if (config.isFeatureEnabled(Feature.AUDIT_LOGS)) {
            featureIndexes.add(Index.AUDIT_LOGS)
        }

        if (config.isFeatureEnabled(Feature.WEBHOOKS)) {
            featureIndexes.add(Index.WEBHOOK_EVENTS)
        }

        val indexes = Index.indexes + featureIndexes
        log.info("Creating indexes [${indexes.joinToString(", ") { it.name }}")

        for (index in indexes) {
            log.info("├── Does index ${index.name} exist?")
            val exists = try {
                val res = client.performRequest(Request("HEAD", "/${index.name}"))
                res.statusLine.statusCode in 200..299
            } catch (e: Exception) {
                throw e
            }

            if (exists) {
                log.info("│   ├── Index ${index.name} exists! Updating index mappings...")
                val settings = index.settings["settings"]!!.jsonObject
                val settingsReq = Request("PUT", "/${index.name}/_mapping")
                settingsReq.setJsonEntity(json.encodeToString(settings))

                try {
                    val res = client.performRequest(settingsReq)
                    if (res.statusLine.statusCode !in 200..299) {
                        log.warn("│   ├── Unable to update index settings for index ${index.name}! There might be some errors when searching or indexing data.")
                    } else {
                        log.info("│   ├── Mappings for index ${index.name} have been updated.")
                    }
                } catch (e: Exception) {
                    log.error("│   ├── Unable to update index mappings for ${index.name}:", e)
                }

                continue
            }

            log.info("│   ├── Index ${index.name} doesn't exist on cluster! Creating index...")
            val request = Request("PUT", "/${index.name}")
            request.setJsonEntity(json.encodeToString(index.settings))

            try {
                val res = client.performRequest(request)
                if (res.statusLine.statusCode !in 200..299) {
                    log.warn("│   ├── Unable to create index ${index.name}! There might be some errors when searching or indexing data.")
                } else {
                    log.info("│   ├── Created index: ${index.name}")
                }
            } catch (e: Exception) {
                log.error("│   ├── Unable to create index ${index.name}:", e)
            }
        }
    }

    private suspend fun indexData() {
        val config by inject<Config>()
        val featureIndexes = mutableListOf<Index>()

        if (config.isFeatureEnabled(Feature.AUDIT_LOGS)) {
            featureIndexes.add(Index.AUDIT_LOGS)
        }

        if (config.isFeatureEnabled(Feature.WEBHOOKS)) {
            featureIndexes.add(Index.WEBHOOK_EVENTS)
        }

        val indexes = Index.indexes + featureIndexes
        for (index in indexes) {
            log.debug("Indexing data for ${index.name}!")
            log.measureSuspendTime("Took %T to index data for index [${index.name}]") {
                when (index.name) {
                    "charted-users" -> {
                        val users = asyncTransaction(ChartedScope) {
                            UserTable.selectAll().toList()
                        }

                        if (users.isEmpty()) {
                            log.warn("Skipping index [${index.name}] due to no data being available.")
                            return@measureSuspendTime
                        }

                        val baos = ByteArrayOutputStream()
                        for (data in users) {
                            val id = data[UserTable.id].value
                            val entity = buildJsonObject {
                                put("description", data[UserTable.description])
                                put("username", data[UserTable.username])
                                put("name", data[UserTable.name])
                            }

                            withContext(Dispatchers.IO) {
                                baos.write("{\"index\":{\"_id\":$id}}".toByteArray())
                                baos.write('\n'.code)
                                baos.write(json.encodeToString(entity).toByteArray())
                                baos.write('\n'.code)
                            }
                        }

                        val request = Request("POST", "/${index.name}/_bulk")
                        request.entity = ByteArrayEntity(baos.toByteArray(), ContentType.APPLICATION_JSON)

                        val res = client.performRequest(request)
                        if (res.statusLine.statusCode !in 200..300) {
                            val body = json.decodeFromStream(JsonObject.serializer(), res.entity.content)
                            log.warn("Unable to send a request to \"POST /${index.name}/_bulk\": $body")
                        } else {
                            log.debug("Indexed all data for index [${index.name}]")
                        }
                    }
                }
            }
        }
    }

    fun info(): Map<String, JsonObject> {
        val data = mutableMapOf<String, JsonObject>()
        val config by inject<Config>()
        val featureIndexes = mutableListOf<Index>()

        if (config.isFeatureEnabled(Feature.AUDIT_LOGS)) {
            featureIndexes.add(Index.AUDIT_LOGS)
        }

        if (config.isFeatureEnabled(Feature.WEBHOOKS)) {
            featureIndexes.add(Index.WEBHOOK_EVENTS)
        }

        val indexes = Index.indexes + featureIndexes

        val res = client.performRequest(Request("GET", "/${indexes.joinToString(",") { it.name }}/_stats"))
        val resData = json.decodeFromStream(JsonObject.serializer(), res.entity.content)
        for (index in indexes) {
            val d = resData["indices"]!!.jsonObject[index.name]!!.jsonObject
            val health = d["health"]!!.jsonPrimitive.content
            val primaries = d["primaries"]!!.jsonObject

            data[index.name] = buildJsonObject {
                put("size_in_bytes", primaries["store"]!!.jsonObject["size_in_bytes"]!!.jsonPrimitive)
                put("documents", primaries["docs"]!!.jsonObject["count"]!!.jsonPrimitive)
                put("health", health)
            }
        }

        return data
    }
}
