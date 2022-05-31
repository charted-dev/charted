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

package org.noelware.charted.search.elastic

import dev.floofy.utils.slf4j.logging
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.json.*
import org.apache.http.HttpHost
import org.apache.http.auth.AuthScope
import org.apache.http.auth.UsernamePasswordCredentials
import org.apache.http.impl.client.BasicCredentialsProvider
import org.apache.http.ssl.SSLContexts
import org.elasticsearch.client.Node
import org.elasticsearch.client.Request
import org.elasticsearch.client.RestClient
import org.elasticsearch.client.sniff.SniffOnFailureListener
import org.elasticsearch.client.sniff.Sniffer
import org.noelware.charted.common.config.ElasticsearchConfig
import org.noelware.charted.search.elastic.apache.SentryRequestInterceptor
import org.noelware.charted.search.elastic.apache.SentryResponseInterceptor
import java.nio.file.Files
import java.nio.file.Paths
import java.security.KeyStore
import java.security.cert.CertificateFactory

@OptIn(ExperimentalSerializationApi::class)
class ElasticsearchBackend(config: ElasticsearchConfig, private val json: Json): AutoCloseable {
    private var _clusterInfo: Pair<String, String>? = null
    private var _version: String? = null
    private val client: RestClient
    private val log by logging<ElasticsearchBackend>()

    /**
     * The current version of Elasticsearch we are running.
     */
    val serverVersion: String
        get() = _version ?: error("Rest client wasn't successfully built.")

    /**
     * The information about the cluster, mapped by tuple of `(Name, UUID)`.
     */
    val clusterInfo: Pair<String, String>
        get() = _clusterInfo ?: error("Rest client wasn't successfully built.")

    init {
        log.debug("Creating connection to Elasticsearch nodes [${config.nodes.joinToString(", ")}]")
        val builder = if (config.cloudId != null) RestClient.builder(config.cloudId) else RestClient.builder(
            *config.nodes.map {
                val splitted = it.split(":")
                val host = splitted.first()

                if (splitted.size == 2) {
                    val port = splitted.last()
                    HttpHost(host, Integer.parseInt(port))
                } else {
                    HttpHost(host, 9200)
                }
            }.toTypedArray()
        )

        val listener = SniffOnFailureListener()
        builder.setFailureListener(object: RestClient.FailureListener() {
            override fun onFailure(node: Node) {
                val attributes = if (node.attributes == null) emptyMap() else node.attributes

                log.warn("Node ${node.name ?: "?"} with host [${node.host} - ${attributes.map { "${it.key}=${it.value}" }.joinToString(", ")}] has failed to execute, now sniffing...")
                listener.onFailure(node)
            }
        })

        if (config.clientSslPath != null) {
            log.debug("Found SSL configuration for client at path [${config.clientSslPath}]")

            val sslPath = Paths.get(config.clientSslPath)
            val sslFactory = CertificateFactory.getInstance("X.509")
            val trustedCa = Files.newInputStream(sslPath).use {
                sslFactory.generateCertificate(it)
            }

            val trustStore = KeyStore.getInstance("pkcs12")
            trustStore.load(null, null)
            trustStore.setCertificateEntry("ca", trustedCa)

            val sslContext = SSLContexts.custom()
                .loadKeyMaterial(trustStore, null)
                .build()

            builder.setHttpClientConfigCallback { build ->
                build.setSSLContext(sslContext)
                build.addInterceptorFirst(SentryRequestInterceptor)
                build.addInterceptorLast(SentryResponseInterceptor)

                if (config.auth != null) {
                    val provider = BasicCredentialsProvider()
                    provider.setCredentials(AuthScope.ANY, UsernamePasswordCredentials(config.auth!!.username, config.auth!!.password))

                    build.setDefaultCredentialsProvider(provider)
                }

                build
            }
        } else {
            builder.setHttpClientConfigCallback { build ->
                build.addInterceptorFirst(SentryRequestInterceptor)
                build.addInterceptorLast(SentryResponseInterceptor)

                if (config.auth != null) {
                    val provider = BasicCredentialsProvider()
                    provider.setCredentials(AuthScope.ANY, UsernamePasswordCredentials(config.auth!!.username, config.auth!!.password))

                    build.setDefaultCredentialsProvider(provider)
                }

                build
            }
        }

        client = builder.build()

        log.debug("Created REST client for ES, now sniffing nodes...")
        val sniffer = Sniffer.builder(client).build()
        listener.setSniffer(sniffer)

        val info = client.performRequest(Request("GET", "/"))
        val data = json.decodeFromStream<JsonObject>(info.entity.content)

        val currentCluster = data["cluster_name"]?.jsonPrimitive?.contentOrNull
        val currentClusterUuid = data["cluster_uuid"]?.jsonPrimitive?.contentOrNull
        val version = data["version"]?.jsonObject?.get("number")?.jsonPrimitive?.contentOrNull

        if (version == null || currentCluster == null || currentClusterUuid == null)
            throw IllegalStateException("Could not get information from Elasticsearch nodes. ($data)")

        log.debug("Connected to cluster [$currentCluster <$currentClusterUuid>] that is using v$version of Elasticsearch!")
        _version = version
        _clusterInfo = currentCluster to currentClusterUuid

        runBlocking {
            setupOrUpdateIndexes()
            indexData()
        }
    }

    override fun close() {
        log.warn("Disconnecting from Elasticsearch...")
        client.close()
    }

    suspend fun indexData() {
        log.info("Now indexing data...")
    }

    fun search(
        query: String,
        limit: Int = 25,
        offset: Int = 0,
        fields: List<String> = listOf("id")
    ): JsonObject {
        val request = Request("POST", "/charted-*/_search")
        val searchQuery = buildJsonObject {
            put("from", offset)
            put("size", limit)
            putJsonObject("query") {
                putJsonObject("multi_match") {
                    put("query", query)
                    put("fields", JsonArray(fields.map { JsonPrimitive(it) }))
                }
            }
        }

        request.setJsonEntity(json.encodeToString(JsonObject.serializer(), searchQuery))
        val resp = client.performRequest(request)
        if (resp.statusLine.statusCode !in 200..300) {
            val payload = json.decodeFromStream(JsonObject.serializer(), resp.entity.content)
            throw IllegalStateException("Unable to request to POST /charted-*/_search: $payload")
        }

        val payload = json.decodeFromStream(JsonObject.serializer(), resp.entity.content)
        val hits = payload["hits"]!!.jsonObject

        return buildJsonObject {
            put("took_in_ms", payload["took"]!!.jsonPrimitive)
            put("max_score", payload["max_score"]!!.jsonPrimitive)
            put("total", hits["total"]!!.jsonObject["value"]!!)
            putJsonArray("hits") {
                for (hit in hits["hits"]!!.jsonArray) {
                    addJsonObject {
                        put("id", hit.jsonObject["_id"]!!.jsonPrimitive)

                        // this is probably bad in scale but shrug
                        val source = hit.jsonObject["_source"]!!.jsonObject.toMap()
                        for ((key, value) in source)
                            put(key, value)
                    }
                }
            }
        }
    }

    private suspend fun setupOrUpdateIndexes() {
        log.info("Checking if indexes exist...")
    }
}
