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

package org.noelware.charted.modules.elasticsearch

import co.elastic.clients.elasticsearch.ElasticsearchAsyncClient
import co.elastic.clients.elasticsearch.indices.ExistsRequest
import co.elastic.clients.json.jackson.JacksonJsonpMapper
import co.elastic.clients.transport.rest_client.RestClientTransport
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import dev.floofy.utils.exposed.asyncTransaction
import dev.floofy.utils.java.SetOnce
import dev.floofy.utils.kotlin.ifNotNull
import dev.floofy.utils.slf4j.logging
import io.sentry.Sentry
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.future.await
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import okhttp3.internal.closeQuietly
import okhttp3.internal.toImmutableList
import org.apache.commons.lang3.time.StopWatch
import org.apache.http.HttpHost
import org.apache.http.auth.AuthScope
import org.apache.http.auth.UsernamePasswordCredentials
import org.apache.http.conn.ssl.NoopHostnameVerifier
import org.apache.http.entity.ByteArrayEntity
import org.apache.http.entity.ContentType
import org.apache.http.impl.client.BasicCredentialsProvider
import org.apache.http.message.BasicHeader
import org.apache.http.ssl.SSLContexts
import org.elasticsearch.client.*
import org.elasticsearch.client.sniff.ElasticsearchNodesSniffer
import org.elasticsearch.client.sniff.SniffOnFailureListener
import org.elasticsearch.client.sniff.Sniffer
import org.jetbrains.exposed.sql.transactions.TransactionManager
import org.noelware.charted.ChartedScope
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.features.ServerFeature
import org.noelware.charted.configuration.kotlin.dsl.search.elasticsearch.AuthStrategyType
import org.noelware.charted.configuration.kotlin.dsl.search.elasticsearch.AuthenticationStrategy
import org.noelware.charted.databases.postgres.entities.RepositoryEntity
import org.noelware.charted.databases.postgres.entities.UserEntity
import org.noelware.charted.databases.postgres.models.Organization
import org.noelware.charted.databases.postgres.models.Repository
import org.noelware.charted.databases.postgres.models.User
import org.noelware.charted.extensions.doFormatTime
import org.noelware.charted.extensions.ifSentryEnabled
import org.noelware.charted.extensions.reflection.getAndUseField
import org.noelware.charted.modules.elasticsearch.metrics.ElasticsearchStats
import java.io.ByteArrayOutputStream
import java.io.File
import java.security.KeyStore
import java.security.cert.CertificateFactory
import java.util.concurrent.CompletableFuture
import java.util.concurrent.atomic.AtomicBoolean
import javax.net.ssl.SSLContext

class DefaultElasticsearchModule(private val config: Config, private val json: Json) : ElasticsearchModule {
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
    @Suppress("MemberVisibilityCanBePrivate")
    var sslContext: SSLContext? = _sslContext

    /** Returns all the indexes that Elasticsearch is responsible for */
    override val indexes: List<String>
        get() {
            val indexes = mutableListOf("charted-users", "charted-repositories", "charted-organizations")
            if (config.features.contains(ServerFeature.AUDIT_LOGS)) {
                indexes.add("charted-audit-logs")
            }

            if (config.features.contains(ServerFeature.WEBHOOKS)) {
                indexes.add("charted-webhook-events")
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
            if (it.startsWith("http")) {
                return@map HttpHost.create(it)
            }

            val mapping = it.split(":", limit = 2)
            if (mapping.size != 2) {
                throw IllegalStateException("Node mapping should be in the 'host:port' format")
            }

            HttpHost(mapping.first(), mapping.last().toInt(), config.ssl?.ifNotNull { "https" })
        }

        val builder = if (config.auth.type == AuthStrategyType.Cloud) {
            log.info("Authentication strategy configured is Elastic Cloud!")
            RestClient.builder((config.auth as AuthenticationStrategy.Cloud).id)
        } else {
            log.info("Authentication strategy configured is not Elastic Cloud")
            RestClient.builder(*nodes.toTypedArray())
        }

        val listener = SniffOnFailureListener()
        builder.setFailureListener(object : RestClient.FailureListener() {
            override fun onFailure(node: Node) {
                val attrs = if (node.attributes != null) {
                    "[${node.attributes.map { "${it.key}=>${it.value}" }.joinToString(" ")}]"
                } else {
                    "[]"
                }

                val nodeName = if (node.name == null) "(unknown)" else node.name
                log.warn("Elasticsearch node [$nodeName@${node.host ?: "(unknown)"} ${node.version?.let { "v$this" }} $attrs has failed executing an action!")
                listener.onFailure(node)
            }
        })

        builder.setHttpClientConfigCallback { hc ->
            when (config.auth) {
                is AuthenticationStrategy.Basic -> {
                    val provider = BasicCredentialsProvider()
                    provider.setCredentials(AuthScope.ANY, UsernamePasswordCredentials((config.auth as AuthenticationStrategy.Basic).username, (config.auth as AuthenticationStrategy.Basic).password))

                    hc.setDefaultCredentialsProvider(provider)
                }

                is AuthenticationStrategy.ApiKey -> {
                    hc.setDefaultHeaders(listOf(BasicHeader("Authorization", "ApiKey ${(config.auth as AuthenticationStrategy.ApiKey).key}")))
                }

                else -> {}
            }

            if (config.ssl != null) {
                log.info("Configuring using SSL for the Elasticsearch transport...")

                val file = File(config.ssl!!.caPath)
                if (!file.exists()) {
                    throw IllegalStateException("File [$file] was not found")
                }

                if (!file.isFile) {
                    throw IllegalStateException("Path [$file] was not a file")
                }

                if (file.extension == "p12") {
                    log.info("File extension was [p12], assuming it is a keystore!")

                    val keystore = KeyStore.getInstance("pkcs12")
                    keystore.load(file.inputStream(), config.ssl?.keystorePassword?.toCharArray())

                    sslContext = SSLContexts.custom()
                        .loadKeyMaterial(keystore, config.ssl!!.keystorePassword?.toCharArray())
                        .build()

                    if (!config.ssl!!.validateHostnames) {
                        hc.setSSLHostnameVerifier(NoopHostnameVerifier.INSTANCE)
                    }

                    hc.setSSLContext(_sslContext)
                } else {
                    log.info("Assuming current path is a certificate!")

                    val factory = CertificateFactory.getInstance("X.509")
                    val trustedCa = factory.generateCertificate(file.inputStream())
                    val trustStore = KeyStore.getInstance("pkcs12")

                    trustStore.load(null, config.ssl!!.keystorePassword?.toCharArray())
                    trustStore.setCertificateEntry("ca", trustedCa)

                    sslContext = SSLContexts.custom()
                        .loadKeyMaterial(trustStore, config.ssl!!.keystorePassword?.toCharArray())
                        .build()

                    if (!config.ssl!!.validateHostnames) {
                        hc.setSSLHostnameVerifier(NoopHostnameVerifier.INSTANCE)
                    }

                    hc.setSSLContext(_sslContext)
                }
            }

            hc
        }

        val lowLevelClient = builder.build()
        val sniffer = Sniffer.builder(lowLevelClient)
            .setNodesSniffer(
                ElasticsearchNodesSniffer(
                    lowLevelClient, ElasticsearchNodesSniffer.DEFAULT_SNIFF_REQUEST_TIMEOUT,
                    if (config.ssl != null) ElasticsearchNodesSniffer.Scheme.HTTPS else ElasticsearchNodesSniffer.Scheme.HTTP,
                ),
            )
            .setSniffAfterFailureDelayMillis(30000).build()

        listener.setSniffer(sniffer)

        sw.suspend()
        log.info("Built low-level REST client in ${sw.doFormatTime()}, checking connection...")

        sw.resume()

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
                createOrUpdateIndexes()
                indexData()
            }
        } catch (e: Exception) {
            ifSentryEnabled { Sentry.captureException(e) }
            log.error("Unable to index all documents into Elasticsearch, data might be loss!", e)
        }
    }

    /**
     * Returns the [statistics object][ElasticsearchStats] for the Elasticsearch cluster that is
     * used for this interface.
     */
    override suspend fun stats(): ElasticsearchStats {
        val indexes = mutableMapOf<String, ElasticsearchStats.IndexStats>()
        val nodes = mutableMapOf<String, ElasticsearchStats.NodeStats>()

        // First, we need to make a request to the Index Stats API
        // (https://www.elastic.co/guide/en/elasticsearch/reference/current/indices-stats.html)
        val indices = client.indices().stats {
            it.index(this@DefaultElasticsearchModule.indexes)
            it
        }.await()

        for ((name, stats) in indices.indices()) {
            val uuid = stats.uuid()
            val size = stats.total()?.store()?.sizeInBytes()?.toLong() ?: 0L
            val health = stats.health()?.jsonValue() ?: "(unknown)"
            val documents = stats.total()?.docs()?.count() ?: 0L
            val deletedDocuments = stats.total()?.docs()?.deleted() ?: 0L
            val queries = stats.total()?.search()?.queryTotal() ?: 0L
            val queryTime = stats.total()?.search()?.queryTimeInMillis() ?: -1L

            indexes[name] = ElasticsearchStats.IndexStats(
                queryTime,
                deletedDocuments,
                documents,
                queries,
                health,
                size,
                uuid!!,
            )
        }

        // Now, we need to collect node stats (because why not!)
        // (https://www.elastic.co/guide/en/elasticsearch/reference/8.5/cluster-nodes-stats.html)
        val nodesStats = client.nodes().stats().await()
        for ((node, stats) in nodesStats.nodes()) {
            val shards = stats.indices()?.shards()?.totalCount() ?: 0L
            val documents = stats.indices()?.docs()?.count() ?: 0L
            val deletedDocuments = stats.indices()?.docs()?.deleted() ?: 0L
            val totalIndexes = stats.indices()?.indexing()?.indexTotal() ?: 0L
            val avgIndexTime = stats.indices()?.indexing()?.indexTimeInMillis() ?: -1L
            val cpuPercentage = stats.os()?.cpu()?.percent()?.toDouble() ?: -1.0

            nodes[stats.name() ?: node] = ElasticsearchStats.NodeStats(
                avgIndexTime,
                deletedDocuments,
                cpuPercentage,
                totalIndexes,
                documents,
                shards,
            )
        }

        return ElasticsearchStats(indexes, nodes)
    }

    override suspend fun indexUser(user: User) {
        log.info("Now indexing user @${user.username} [${user.id}] into Elasticsearch!")

        val resp = client.index {
            it.document(json.encodeToString(user.toJsonObject()))
            it.index("charted-users")

            it
        }.await()

        log.info("Successfully indexed user @${user.username} [${user.id}] (version ${resp.version()})")
    }

    override suspend fun indexRepository(repository: Repository) {
        log.info("Now indexing repository ${repository.name} [${repository.id}] into Elasticsearch!")

        val resp = client.index {
            it.document(json.encodeToString(repository))
            it.index("charted-repositories")

            it
        }.await()

        log.info("Successfully indexed repository ${repository.name} [${repository.id}] (version ${resp.version()})")
    }

    override suspend fun indexOrganization(org: Organization) {
        log.info("Now indexing organization ${org.name} [${org.id}] into Elasticsearch!")

        val resp = client.index {
            it.document(json.encodeToString(org.toJsonObject()))
            it.index("charted-organizations")

            it
        }.await()

        log.info("Successfully indexed repository ${org.name} [${org.id}] (version ${resp.version()})")
    }

    override suspend fun unindexUser(user: User) {
        log.warn("Un-indexing user @${user.username} [${user.id}]")

        client.delete {
            it.index("charted-users")
            it.id(user.id.toString())

            it
        }.await()
    }

    override suspend fun unindexRepository(repository: Repository) {
        log.warn("Un-indexing repository ${repository.name} [${repository.id}]")

        client.delete {
            it.index("charted-repositories")
            it.id(repository.id.toString())

            it
        }.await()
    }

    override suspend fun unindexOrganization(org: Organization) {
        log.warn("Un-indexing repository ${org.name} [${org.id}]")

        client.delete {
            it.index("charted-organizations")
            it.id(org.id.toString())

            it
        }.await()
    }

    private suspend fun createOrUpdateIndexes() {
        log.info("Attempting to check if indexes [${indexes.joinToString(", ")}] exist or not in Elasticsearch!")

        for (index in indexes) {
            val req = ExistsRequest.Builder().apply {
                index(index)
            }.build()

            val res = client.indices().exists(req).await()
            if (res.value()) {
                log.info("~> Index {$index} exists in Elasticsearch! Updating index mappings...")
                val stream = this::class.java.getResourceAsStream("/mappings/$index.json")
                if (stream == null) {
                    log.warn("Index {$index} doesn't contain any mappings in resources, skipping")
                    continue
                }

                val mapper = client._transport().jsonpMapper()
                client.indices().putMapping {
                    it.index(index)
                    it.withJson(mapper.jsonProvider().createParser(stream), mapper)

                    it
                }.await()
            } else {
                log.warn("~> Index {$index} doesn't exist in Elasticsearch! Creating index...")

                val stream = this::class.java.getResourceAsStream("/mappings/$index.json")
                if (stream == null) {
                    log.warn("Index {$index} doesn't contain any mappings in resources, skipping")
                    continue
                }

                val mapper = client._transport().jsonpMapper()
                client.indices().create {
                    it.index(index)
                    it.withJson(mapper.jsonProvider().createParser(stream), mapper)

                    it
                }.await()
            }
        }
    }

    private suspend fun indexData() {
        // Check if the database is initialized or not. Useful for tests, not
        // for production.
        if (!TransactionManager.isInitialized()) {
            log.warn("Missing [TransactionManager], assuming we are in test mode.")
            return
        }

        log.info("Performing indexing on indexes [${indexes.joinToString(", ")}]")

        // We need access to the low level REST client, and the only way (so far) is to
        // use reflection to get the value. Yeah, very icky, I know. :(
        val restClient: RestClient = (client._transport() as RestClientTransport).getAndUseField("restClient")!!
        val sw = StopWatch.createStarted()

        for (index in indexes) {
            when (index) {
                "charted-users" -> {
                    if (sw.isSuspended) sw.resume()
                    val users = asyncTransaction(ChartedScope) {
                        UserEntity.all().map { entity -> User.fromEntity(entity) }.toImmutableList()
                    }

                    if (users.isEmpty()) {
                        continue
                    }

                    log.info("Indexing [${users.size}] users in index {$index}...")
                    val baos = ByteArrayOutputStream()
                    for (entity in users) {
                        withContext(Dispatchers.IO) {
                            baos.write("{\"index\":{\"_id\":${entity.id}}}".toByteArray())
                            baos.write('\n'.code)
                            baos.write(json.encodeToString(entity.toJsonObject()).toByteArray())
                            baos.write('\n'.code)
                        }
                    }

                    runBulkRequest(sw, restClient, index, baos)
                }

                "charted-repositories" -> {
                    if (sw.isSuspended) sw.resume()

                    val repositories = asyncTransaction(ChartedScope) {
                        RepositoryEntity.all().map { entity -> Repository.fromEntity(entity) }.toImmutableList()
                    }

                    if (repositories.isEmpty()) {
                        continue
                    }

                    log.info("Indexing ${repositories.size} repositories in index {$index}!")
                    val baos = ByteArrayOutputStream()
                    for (repo in repositories) {
                        withContext(Dispatchers.IO) {
                            baos.write("""{"index":{"_id":${repo.id}}}""".toByteArray())
                            baos.write('\n'.code)
                            baos.write(json.encodeToString(repo).toByteArray())
                            baos.write('\n'.code)
                        }
                    }

                    runBulkRequest(sw, restClient, index, baos)
                }

                else -> log.warn("Index {$index} doesn't support indexing at this time.")
            }
        }
    }

    private suspend fun runBulkRequest(sw: StopWatch, restClient: RestClient, index: String, baos: ByteArrayOutputStream) {
        val request = Request("POST", "/$index/_bulk")
        request.entity = ByteArrayEntity(baos.toByteArray(), ContentType.APPLICATION_JSON)

        val fut = CompletableFuture<Unit>()
        restClient.performRequestAsync(
            request,
            object: ResponseListener {
                override fun onFailure(exception: java.lang.Exception?) {
                    fut.completeExceptionally(exception)
                }

                override fun onSuccess(response: Response) {
                    sw.suspend()
                    log.info("Performed request [$request] with status line [${response.statusLine}][${sw.doFormatTime()}]")
                    fut.complete(Unit)
                }
            },
        )

        fut.await()
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
