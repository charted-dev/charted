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

package org.noelware.charted.modules.elasticsearch

import co.elastic.clients.elasticsearch.ElasticsearchAsyncClient
import co.elastic.clients.json.jackson.JacksonJsonpMapper
import co.elastic.clients.transport.rest_client.RestClientTransport
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import dev.floofy.utils.slf4j.logging
import io.sentry.Sentry
import kotlinx.coroutines.future.await
import kotlinx.coroutines.launch
import kotlinx.serialization.json.Json
import okhttp3.internal.closeQuietly
import org.apache.commons.lang3.time.StopWatch
import org.apache.http.HttpHost
import org.apache.http.auth.AuthScope
import org.apache.http.auth.UsernamePasswordCredentials
import org.apache.http.impl.client.BasicCredentialsProvider
import org.apache.http.message.BasicHeader
import org.elasticsearch.client.Node
import org.elasticsearch.client.RestClient
import org.elasticsearch.client.sniff.SniffOnFailureListener
import org.elasticsearch.client.sniff.Sniffer
import org.noelware.charted.ChartedScope
import org.noelware.charted.common.SetOnce
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.ServerFeature
import org.noelware.charted.configuration.kotlin.dsl.search.elasticsearch.AuthStrategyType
import org.noelware.charted.configuration.kotlin.dsl.search.elasticsearch.AuthenticationStrategy
import org.noelware.charted.extensions.doFormatTime
import org.noelware.charted.extensions.ifSentryEnabled
import java.util.concurrent.atomic.AtomicBoolean
import javax.net.ssl.SSLContext

class DefaultElasticsearchModule(private val config: Config, private val json: Json): ElasticsearchModule {
    private val _serverVersion: SetOnce<String> = SetOnce()
    private val _clusterInfo: SetOnce<Pair<String, String>> = SetOnce()
    private val _closed: AtomicBoolean = AtomicBoolean(false)
    private val _client: SetOnce<ElasticsearchAsyncClient> = SetOnce()
    private var _sslContext: SSLContext? = null
    private val log by logging<DefaultElasticsearchModule>()

    /**
     * Represents the SSL context to use to create the REST client. This is primarily used in tests
     * and shouldn't be touched at all.
     */
    var sslContext: SSLContext? = _sslContext

    /** Returns all the indexes that Elasticsearch is responsible for */
    override val indexes: List<String>
        get() {
            val indexes = mutableListOf("charted-users", "charted-repositories", "charted-organizations")
            if (config.features.contains(ServerFeature.AUDIT_LOGS)) {
                indexes.add("charted-audit-logs")
            }

            if (config.features.contains(ServerFeature.WEBHOOKS)) {
                indexes.add("charted-webhooks")
            }

            if (config.features.contains(ServerFeature.DOCKER_REGISTRY)) {
                indexes.add("charted-oci-containers")
            }

            return indexes
        }

    /**
     * Returns the Elasticsearch server version that the cluster is running on. charted-server requires the
     * cluster to be using Elasticsearch 8.
     */
    override val serverVersion: String
        get() = _serverVersion.value

    /**
     * Returns the Elasticsearch cluster's name that was collected when the client was
     * being connected.
     */
    override val clusterName: String
        get() = _clusterInfo.value.first

    /**
     * Returns the Elasticsearch cluster's UUId that was collected when the
     * client was being collected.
     */
    override val clusterUUID: String
        get() = _clusterInfo.value.second

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

    /** Connects to Elasticsearch! */
    override suspend fun connect() {
        if (closed) {
            log.warn("Elasticsearch module is closed and the connection is no longer available.")
            return
        }

        val sw = StopWatch.createStarted()
        log.info("Creating low-level REST client...")

        val config = config.search!!.elasticsearch!!
        val nodes = config.nodes.map {
            val mapping = it.split(":", limit = 2)
            if (mapping.size != 2) {
                throw IllegalStateException("Node mapping should be in the 'host:port' format")
            }

            HttpHost(mapping.first(), mapping.last().toInt())
        }

        val builder = if (config.auth.type == AuthStrategyType.Cloud) {
            log.info("Authentication strategy configured is Elastic Cloud!")
            RestClient.builder((config.auth as AuthenticationStrategy.Cloud).id)
        } else {
            log.info("Authentication strategy configured is not Elastic Cloud")
            RestClient.builder(*nodes.toTypedArray())
        }

        val listener = SniffOnFailureListener()
        builder.setFailureListener(object: RestClient.FailureListener() {
            override fun onFailure(node: Node) {
                val attrs = if (node.attributes != null) {
                    "[${node.attributes.map { "${it.key}=>${it.value}" }.joinToString(" ")}]"
                } else {
                    "[]"
                }

                val nodeName = if (node.name == null) "(unknown)" else node.name
                log.warn("Elasticsearch node [$nodeName@${node.host} v${node.version}] $attrs has failed executing an action!")
                listener.onFailure(node)
            }
        })

        // For now, we don't do SSL for the Elasticsearch client, but we plan to once we know how to
        // lay it out.
        builder.setHttpClientConfigCallback {
            when (config.auth) {
                is AuthenticationStrategy.Basic -> {
                    val provider = BasicCredentialsProvider()
                    provider.setCredentials(AuthScope.ANY, UsernamePasswordCredentials((config.auth as AuthenticationStrategy.Basic).username, (config.auth as AuthenticationStrategy.Basic).password))

                    it.setDefaultCredentialsProvider(provider)
                }

                is AuthenticationStrategy.ApiKey -> {
                    it.setDefaultHeaders(listOf(BasicHeader("Authorization", "ApiKey ${(config.auth as AuthenticationStrategy.ApiKey).key}")))
                }

                else -> {}
            }

            it
        }

        val lowLevelClient = builder.build()
        val sniffer = Sniffer.builder(lowLevelClient).setSniffAfterFailureDelayMillis(30000).build()
        listener.setSniffer(sniffer)

        sw.stop()
        log.info("Built low-level REST client in ${sw.doFormatTime()}, checking connection...")

        sw.reset()
        sw.start()

        val objectMapper = ObjectMapper().registerKotlinModule()
        val transport = RestClientTransport(lowLevelClient, JacksonJsonpMapper(objectMapper))
        _client.value = ElasticsearchAsyncClient(transport)

        log.info("Initialized the async client for Elasticsearch!")

        val info = client.info().await()
        sw.stop()

        log.info("Reached to cluster ${info.clusterName()} [${info.clusterUuid()}] with Elasticsearch version v${info.version().number()} (${info.version().buildType()}/${info.version().buildFlavor()} distribution) in ${sw.doFormatTime()}")
        _clusterInfo.value = info.clusterName() to info.clusterUuid()
        _serverVersion.value = info.version().number()

        try {
            log.info("Now indexing documents from databases...")

            // Start in a background coroutine so other components can load since
            // indexing in Elasticsearch can take a while ;-;
            ChartedScope.launch {
                // createOrUpdateIndexes()
                // indexData()
            }
        } catch (e: Exception) {
            ifSentryEnabled { Sentry.captureException(e) }
            log.error("Unable to index all documents into Elasticsearch, data might be loss!", e)
        }
    }

    /**
     * Closes this stream and releases any system resources associated
     * with it. If the stream is already closed then invoking this
     * method has no effect.
     *
     * As noted in [AutoCloseable.close], cases where the
     * close may fail require careful attention. It is strongly advised
     * to relinquish the underlying resources and to internally
     * *mark* the `Closeable` as closed, prior to throwing
     * the `IOException`.
     *
     * @throws java.io.IOException if an I/O error occurs
     */
    override fun close() {
        if (_closed.compareAndSet(false, true)) {
            log.warn("Closing off REST client...")
            client._transport().closeQuietly()
        }
    }
}
