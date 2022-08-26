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

package org.noelware.charted.elasticsearch

import co.elastic.clients.elasticsearch.ElasticsearchAsyncClient
import co.elastic.clients.elasticsearch._types.ElasticsearchException
import co.elastic.clients.json.jackson.JacksonJsonpMapper
import co.elastic.clients.transport.rest_client.RestClientTransport
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import dev.floofy.utils.slf4j.logging
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.future.await
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonObject
import okhttp3.internal.closeQuietly
import org.apache.http.HttpHost
import org.apache.http.auth.AuthScope
import org.apache.http.auth.UsernamePasswordCredentials
import org.apache.http.impl.client.BasicCredentialsProvider
import org.apache.http.ssl.SSLContexts
import org.elasticsearch.client.Node
import org.elasticsearch.client.RestClient
import org.elasticsearch.client.sniff.SniffOnFailureListener
import org.elasticsearch.client.sniff.Sniffer
import org.noelware.charted.common.ChartedScope
import org.noelware.charted.common.SetOnceGetValue
import org.noelware.charted.common.data.Config
import org.noelware.charted.common.data.Feature
import org.noelware.charted.common.extensions.measureSuspendTime
import org.noelware.charted.elasticsearch.apache.SentryApacheHttpClientRequestInterceptor
import org.noelware.charted.elasticsearch.apache.SentryApacheHttpClientResponseInterceptor
import org.noelware.charted.elasticsearch.stats.ElasticsearchStats
import java.nio.file.Files
import java.nio.file.Paths
import java.security.KeyStore
import java.security.cert.CertificateFactory
import java.util.concurrent.atomic.AtomicBoolean

class DefaultElasticsearchService(
    private val config: Config,
    private val json: Json
): ElasticsearchService {
    private val _serverVersion: SetOnceGetValue<String> = SetOnceGetValue()
    private val _cluster: SetOnceGetValue<Pair<String, String>> = SetOnceGetValue()
    private val _closed: AtomicBoolean = AtomicBoolean(false)
    private val _client: SetOnceGetValue<ElasticsearchAsyncClient> = SetOnceGetValue()
    private val log by logging<DefaultElasticsearchService>()

    /**
     * Returns a list of indexes the service is responsible for.
     */
    override val indexes: List<String>
        get() {
            val indexes = mutableListOf("charted_users", "charted_repositories", "charted_organizations")
            if (config.isFeatureEnabled(Feature.WEBHOOKS)) {
                indexes.add("charted_webhook_events")
            }

            return indexes.toList()
        }

    /**
     * Returns the current server version of the Elasticsearch cluster it is
     * connected to.
     */
    override val serverVersion: String
        get() = _serverVersion.value

    /**
     * Returns the Elasticsearch cluster's name that was collected when the client was
     * being connected.
     */
    override val clusterName: String
        get() = _cluster.value.first

    /**
     * Returns the Elasticsearch cluster's UUId that was collected when the
     * client was being collected.
     */
    override val clusterUUID: String
        get() = _cluster.value.second

    /**
     * Returns if the service was currently closed.
     */
    override val closed: Boolean
        get() = _closed.get()

    /**
     * Returns a reference of the [asynchronous client][ElasticsearchAsyncClient].
     */
    override val client: ElasticsearchAsyncClient
        get() = _client.value

    /**
     * Connects to the Elasticsearch cluster that was connected.
     */
    override suspend fun connect() {
        if (closed) return

        log.debug("Creating low-level REST client to Elasticsearch...")
        val cfg = config.search.elastic!!
        val nodes = cfg.nodes.map {
            val (host, port) = it.split(":")
            HttpHost(host, port.toInt())
        }.toTypedArray()

        val builder = if (cfg.cloudId != null) {
            RestClient.builder(cfg.cloudId)
        } else {
            RestClient.builder(*nodes)
        }

        val listener = SniffOnFailureListener()
        builder.setFailureListener(object: RestClient.FailureListener() {
            override fun onFailure(node: Node) {
                val attrs = if (node.attributes != null) {
                    "[${node.attributes.map { "${it.key}=>${it.value}" }.joinToString(" ")}]"
                } else {
                    "[]"
                }

                log.warn("Elasticsearch node [${node.name}@${node.host} v${node.version}] $attrs has failed executing an action!")
                listener.onFailure(node)
            }
        })

        if (cfg.clientSslPath != null) {
            log.warn("Received client SSL in path [${cfg.clientSslPath}]")

            val sslPath = Paths.get(cfg.clientSslPath!!)
            val sslFactory = CertificateFactory.getInstance("X.509")
            val trustedCa = withContext(Dispatchers.IO) {
                Files.newInputStream(sslPath)
            }.use { sslFactory.generateCertificate(it) }

            val trustStore = KeyStore.getInstance("pkcs12")
            withContext(Dispatchers.IO) {
                trustStore.load(null, null)
            }

            trustStore.setCertificateEntry("ca", trustedCa)
            val sslContext = SSLContexts.custom()
                .loadKeyMaterial(trustStore, null)
                .build()

            builder.setHttpClientConfigCallback {
                it.setSSLContext(sslContext)
                if (cfg.auth != null) {
                    val provider = BasicCredentialsProvider()
                    provider.setCredentials(
                        AuthScope.ANY,
                        UsernamePasswordCredentials(cfg.auth!!.username, cfg.auth!!.password)
                    )

                    it.setDefaultCredentialsProvider(provider)
                }

                it.addInterceptorFirst(SentryApacheHttpClientRequestInterceptor())
                it.addInterceptorLast(SentryApacheHttpClientResponseInterceptor())
                it
            }
        } else {
            builder.setHttpClientConfigCallback {
                if (cfg.auth != null) {
                    val provider = BasicCredentialsProvider()
                    provider.setCredentials(
                        AuthScope.ANY,
                        UsernamePasswordCredentials(cfg.auth!!.username, cfg.auth!!.password)
                    )

                    it.setDefaultCredentialsProvider(provider)
                }

                it.addInterceptorFirst(SentryApacheHttpClientRequestInterceptor())
                it.addInterceptorLast(SentryApacheHttpClientResponseInterceptor())
                it
            }
        }

        val lowLevelClient = builder.build()
        val sniffer = Sniffer.builder(lowLevelClient).setSniffAfterFailureDelayMillis(30000).build()
        listener.setSniffer(sniffer)

        log.info("Built low-level REST client! Now creating async client...")

        val objectMapper = ObjectMapper().registerKotlinModule()
        val transport = RestClientTransport(lowLevelClient, JacksonJsonpMapper(objectMapper))

        _client.value = ElasticsearchAsyncClient(transport)
        log.info("Initialised Elasticsearch client!")

        log.measureSuspendTime("Took %T to collect server metadata!") {
            val info = client.info().await()
            log.debug("Reached to cluster ${info.clusterName()} [${info.clusterUuid()}] with version v${info.version().number()}")
            _cluster.value = info.clusterName() to info.clusterUuid()
            _serverVersion.value = info.version().number()
        }

        try {
            log.info("Now indexing documents from Postgres...")
            ChartedScope.launch {
                createOrUpdateIndexes()
                indexAllData()
            }.join()
        } catch (e: Exception) {
            log.error("unable to index data into elasticsearch:", e)
        }
    }

    override fun close() {
        log.warn("Closing off REST client...")
        client._transport().closeQuietly()
    }

    override suspend fun collect(): ElasticsearchStats {
        val indexStatsMap = mutableMapOf<String, ElasticsearchStats.IndexStat>()
        val indexStats = client.indices().stats {
            it.index(indexes)
        }.await()

        for ((key, value) in indexStats.indices()) {
            val idx = indexes.singleOrNull { it == key } ?: continue
            indexStatsMap[idx] = ElasticsearchStats.IndexStat(
                value.primaries()?.store()?.sizeInBytes()?.toLong() ?: 0,
                value.primaries()?.docs()?.count() ?: 0,
                value.primaries()?.docs()?.deleted() ?: 0,
                value.health()!!.name
            )
        }

        val clusterStats = client.cluster().stats().await()
        val stats = ElasticsearchStats(
            clusterStats.indices().store().sizeInBytes().toLong(),
            clusterStats.indices().docs().count(),
            clusterStats.indices().docs().deleted() ?: 0,
            mapOf(),
            clusterStats.status().name,
            ElasticsearchStats.MemoryStat(
                clusterStats.nodes().os().mem().totalInBytes(),
                clusterStats.nodes().os().mem().freeInBytes(),
                clusterStats.nodes().os().mem().usedInBytes()
            )
        )

        return stats
    }

    override suspend fun search(query: String, attrs: List<String>, limit: Int, offset: Int): JsonObject {
        TODO("Not yet implemented")
    }

    private suspend fun createOrUpdateIndexes() {
        val indexes = mutableListOf("charted_users", "charted_repositories", "charted_organizations")
        if (config.isFeatureEnabled(Feature.WEBHOOKS)) {
            indexes.add("charted_webhook_events")
        }

        log.info("Creating or updating indexes [${indexes.joinToString(", ")}]")
        for (index in indexes) {
            val exists = client.indices().exists { it.index(index) }.await().value()
            if (exists) {
                log.info("Index $index already exists! Updating index mappings...")
                val inputStream = this::class.java.getResourceAsStream("/indexes/mappings/$index.json")
                    ?: continue

                val (_, error) = client.indices().putMapping {
                    it.index(index)
                    it.withJson(inputStream)
                }.awaitOrError(ElasticsearchException::class)

                inputStream.closeQuietly()
                if (error != null) {
                    log.error("Unable to put index mappings to index [$index]:", error)
                } else {
                    log.info("Index [$index] mappings have been saved.")
                }
            } else {
                log.warn("Index [$index] doesn't exist in Elasticsearch!")
                val inputStream = this::class.java.getResourceAsStream("/indexes/$index.json")
                    ?: continue

                val (_, error) = client.indices().create {
                    it.index(index)
                    it.withJson(inputStream)
                }.awaitOrError(ElasticsearchException::class)

                inputStream.closeQuietly()
                if (error != null) {
                    log.error("Unable to create index [$index]:", error)
                } else {
                    log.info("Index [$index] has been created.")
                }
            }
        }
    }

    private suspend fun indexAllData() {}
}
