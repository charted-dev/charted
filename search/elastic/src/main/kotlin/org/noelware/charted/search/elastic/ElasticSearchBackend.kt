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

import dev.floofy.utils.koin.inject
import dev.floofy.utils.slf4j.logging
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.json.*
import org.apache.http.HttpHost
import org.apache.http.auth.AuthScope
import org.apache.http.auth.UsernamePasswordCredentials
import org.apache.http.impl.client.BasicCredentialsProvider
import org.apache.http.ssl.SSLContexts
import org.elasticsearch.client.*
import org.elasticsearch.client.sniff.SniffOnFailureListener
import org.elasticsearch.client.sniff.Sniffer
import org.noelware.charted.search.elastic.interceptor.ApacheSentryRequestInterceptor
import org.noelware.charted.search.elastic.interceptor.ApacheSentryResponseInterceptor
import java.io.Closeable
import java.nio.file.Files
import java.nio.file.Paths
import java.security.KeyStore
import java.security.cert.CertificateFactory

@OptIn(ExperimentalSerializationApi::class)
class ElasticSearchBackend(config: ElasticsearchConfig): Closeable {
    private val log by logging<ElasticSearchBackend>()
    private val json by inject<Json>()

    val serverVersion: String
    val clusterName: String
    val clusterUUID: String

    private var client: RestClient? = null

    init {
        log.debug("Creating connection to Elasticsearch with nodes [${config.nodes.joinToString(", ")}]")

        val builder = if (config.cloudId != null) {
            RestClient.builder(config.cloudId)
        } else {
            RestClient.builder(
                *config.nodes.map { val (host, port) = it.split(":"); HttpHost(host, Integer.parseInt(port)) }.toTypedArray()
            )
        }

        val listener = SniffOnFailureListener()
        builder.setFailureListener(object: RestClient.FailureListener() {
            override fun onFailure(node: Node) {
                log.warn(
                    "Node ${node.host} (${node.name} on v${node.version}) [${
                    node.attributes.map { "${it.key}=${it.value}" }.joinToString(", ")
                    }] has failed to execute an action, sniffing!"
                )

                listener.onFailure(node)
            }
        })

        if (config.clientSslPath != null) {
            log.debug("Client SSL path was configured at ${config.clientSslPath}! Configuring...")

            // TODO: should we provide our own keystore for this?
            //       and use `charted elastic:ssl add ./path/to/client.ssl`?
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

            builder.setHttpClientConfigCallback {
                it.setSSLContext(sslContext)
                it.addInterceptorFirst(ApacheSentryRequestInterceptor)
                it.addInterceptorLast(ApacheSentryResponseInterceptor)

                if (config.auth != null) {
                    val provider = BasicCredentialsProvider()
                    provider.setCredentials(AuthScope.ANY, UsernamePasswordCredentials(config.auth.username, config.auth.password))

                    it.setDefaultCredentialsProvider(provider)
                }

                it
            }
        } else {
            builder.setHttpClientConfigCallback {
                it.addInterceptorFirst(ApacheSentryRequestInterceptor)
                it.addInterceptorLast(ApacheSentryResponseInterceptor)
                if (config.auth != null) {
                    val provider = BasicCredentialsProvider()
                    provider.setCredentials(AuthScope.ANY, UsernamePasswordCredentials(config.auth.username, config.auth.password))

                    it.setDefaultCredentialsProvider(provider)
                }

                it
            }
        }

        client = builder.build()

        log.debug("Created a REST client! Now sniffing nodes...")
        val sniffer = Sniffer.builder(client).build()
        listener.setSniffer(sniffer)

        val info = client!!.performRequest(Request("GET", "/"))
        val data = json.decodeFromStream(JsonObject.serializer(), info.entity.content)
        val currentCluster = data["cluster_name"]?.jsonPrimitive?.contentOrNull
        val currentClusterId = data["cluster_uuid"]?.jsonPrimitive?.contentOrNull
        val version = data["version"]?.jsonObject?.get("number")?.jsonPrimitive?.contentOrNull

        if (version == null || currentCluster == null || currentClusterId == null)
            throw IllegalStateException("Could not get information from Elasticsearch nodes. ($data)")

        log.debug("Reached to cluster $currentCluster ($currentClusterId) that is using v$version of Elasticsearch!")
        serverVersion = version
        clusterName = currentCluster
        clusterUUID = currentClusterId

        createIndexes()
    }

    override fun close() {
        if (client == null) return

        log.warn("Disconnecting from Elasticsearch...")
        client?.close()
    }

    private fun createIndexes() {
        log.info("Creating Elasticsearch indexes if they don't exist...")

        for ((index, settings) in INDEX_SETTINGS) {
            log.info("Does index $index exist?...")
            val res1 = client!!.performRequest(Request("HEAD", "/$index"))
            if (res1.statusLine.statusCode == 200) {
                log.warn("Index $index already exists! Skipping...")
                continue
            }

            log.warn("Index $index doesn't exist!")
            val request = Request("PUT", "/$index")
            request.setJsonEntity(json.encodeToString(JsonObject.serializer(), settings))

            val res2 = client!!.performRequest(request)
            if (res2.statusLine.statusCode !in 200..300) {
                val body = json.decodeFromStream(JsonObject.serializer(), res2.entity.content)
                log.warn("Unable to create a request to \"PUT /$index\" - $body")
            } else {
                log.info("Index $index now exists in Elasticsearch :D")
            }
        }
    }

    fun info(): Map<String, JsonObject> {
        val dataMap = mutableMapOf<String, JsonObject>()
        val res = client!!.performRequest(Request("GET", "/charted_users,charted_repos,charted_repo_members,charted_org_members,charted_orgs/_stats"))
        val data = json.decodeFromStream(JsonObject.serializer(), res.entity.content)

        for (index in listOf("charted_users", "charted_repos", "charted_repo_members", "charted_orgs", "charted_org_members")) {
            val d = data["indices"]!!.jsonObject[index]!!.jsonObject
            val health = d["health"]!!.jsonPrimitive.content
            val primaries = d["primaries"]!!.jsonObject

            dataMap[index] = buildJsonObject {
                put("health", health)
                put("documents", primaries["docs"]!!.jsonObject["count"]!!.jsonPrimitive)
                put("size_in_bytes", primaries["store"]!!.jsonObject["size_in_bytes"]!!.jsonPrimitive)
            }
        }

        return dataMap
    }

    fun search(
        index: Indexes,
        query: String,
        limit: Int = 25,
        offset: Int = 0,
        fieldsToRequest: List<String> = listOf()
    ): JsonObject {
        val request = Request("POST", "/${index.index}/_search")
        request.addParameters(
            mapOf(
                "from" to limit.toString(),
                "size" to offset.toString()
            )
        )

        val searchQuery = buildJsonObject {
            put(
                "bool",
                buildJsonObject {
                    put(
                        "must",
                        buildJsonArray {
                            add(
                                buildJsonObject {
                                    put(
                                        "simple_query_string",
                                        buildJsonObject {
                                            put("fields", JsonArray(fieldsToRequest.ifEmpty { listOf("id") }.map { JsonPrimitive(it) }))
                                            put("query", query)
                                        }
                                    )
                                }
                            )
                        }
                    )
                }
            )

            put(
                "sort",
                buildJsonArray {
                    add(
                        buildJsonObject {
                            put("_doc", "desc")
                        }
                    )
                }
            )
        }

        request.setJsonEntity(json.encodeToString(JsonObject.serializer(), searchQuery))
        val res = client!!.performRequest(request)

        if (res.statusLine.statusCode !in 200..300) {
            val payload = json.decodeFromStream(JsonObject.serializer(), res.entity.content)
            throw IllegalStateException("Unable to request to POST /${index.index}/_search: $payload")
        }

        val payload = json.decodeFromStream(JsonObject.serializer(), res.entity.content)
        return buildJsonObject {
            put("took", payload["took"]!!.jsonPrimitive)
            put("max_score", payload["max_score"]?.let { JsonNull }!!)
            put("hits", JsonArray(payload["hits"]!!.jsonArray.map { it }))
        }
    }
}
