/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware, LLC. <team@noelware.org>
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

package org.noelware.charted.modules.search.elasticsearch

import co.elastic.clients.elasticsearch.ElasticsearchAsyncClient
import co.elastic.clients.elasticsearch.indices.ExistsRequest
import co.elastic.clients.json.jackson.JacksonJsonpMapper
import co.elastic.clients.transport.rest_client.RestClientTransport
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import dev.floofy.utils.java.SetOnce
import dev.floofy.utils.kotlin.ifNotNull
import dev.floofy.utils.slf4j.logging
import io.sentry.Sentry
import kotlinx.atomicfu.AtomicBoolean
import kotlinx.atomicfu.atomic
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.future.await
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withContext
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
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
import org.noelware.charted.common.extensions.closeable.closeQuietly
import org.noelware.charted.common.extensions.formatting.doFormatTime
import org.noelware.charted.common.extensions.sentry.ifSentryEnabled
import org.noelware.charted.common.jackson.KotlinxDatetimeJacksonModule
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.search.elasticsearch.AuthStrategyType
import org.noelware.charted.configuration.kotlin.dsl.search.elasticsearch.AuthenticationStrategy
import org.noelware.charted.models.organizations.Organization
import org.noelware.charted.models.repositories.Repository
import org.noelware.charted.models.users.User
import org.noelware.charted.modules.postgresql.asyncTransaction
import org.noelware.charted.modules.postgresql.entities.OrganizationEntity
import org.noelware.charted.modules.postgresql.entities.RepositoryEntity
import org.noelware.charted.modules.postgresql.entities.UserEntity
import org.noelware.charted.modules.postgresql.extensions.fromEntity
import org.noelware.charted.modules.search.elasticsearch.metrics.ElasticsearchStats
import org.noelware.charted.launch
import org.noelware.charted.modules.tracing.Traceable
import java.io.ByteArrayOutputStream
import java.io.File
import java.security.KeyStore
import java.security.cert.CertificateFactory
import java.util.concurrent.CompletableFuture
import javax.net.ssl.SSLContext

class DefaultElasticsearchModule(
    private val json: Json,
    private val config: Config
): ElasticsearchModule {
    private val _serverVersion: SetOnce<String> = SetOnce()
    private val _clusterInfo: SetOnce<Pair</* name */ String, /* uuid */ String>> = SetOnce()
    private val _closed: AtomicBoolean = atomic(false)
    private val _client: SetOnce<ElasticsearchAsyncClient> = SetOnce()
    private val log by logging<DefaultElasticsearchModule>()

    /**
     * Represents the SSL context to use to create the REST client. This is primarily used in tests
     * and shouldn't be touched at all.
     */
    @Suppress("MemberVisibilityCanBePrivate")
    var sslContext: SSLContext? = null

    /** Returns all the indexes that Elasticsearch is responsible for */
    val indexes: List<String>
        get() = listOf("charted-users", "charted-repositories", "charted-organizations")

    override val closed: Boolean
        get() = _closed.value

    override val serverVersion: String
        get() = _serverVersion.value

    override val clusterName: String
        get() = _clusterInfo.value.first

    override val clusterUUID: String
        get() = _clusterInfo.value.second

    override val client: ElasticsearchAsyncClient
        get() = _client.value

    override suspend fun init() {
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

            val mapping = it.split(':', limit = 2)
            if (mapping.size != 2) {
                throw IllegalStateException("Node host mapping should be in 'host:port' format")
            }

            HttpHost(mapping.first(), mapping.last().toInt(), config.ssl?.ifNotNull { "https" })
        }

        val builder = if (config.auth.type == AuthStrategyType.Cloud) {
            RestClient.builder((config.auth as AuthenticationStrategy.Cloud).id)
        } else {
            RestClient.builder(*nodes.toTypedArray())
        }

        val listener = SniffOnFailureListener()
        builder.setFailureListener(object: RestClient.FailureListener() {
            override fun onFailure(node: Node) {
                val attributes = node.attributes.ifNotNull {
                    "[${map { "${it.key}=>${it.value}" }.joinToString(" ")}]"
                } ?: "[]"

                val nodeName = node.name ?: "(unknown)"
                val nodeVersion = node.version?.let { "v$this" } ?: "(unknown)"
                log.warn("Node [$nodeName@${node.host ?: "(unknown)"} $nodeVersion] $attributes has failed execution an request")
                listener.onFailure(node)
            }
        })

        builder.setHttpClientConfigCallback { hc ->
            when (config.auth) {
                is AuthenticationStrategy.Basic -> {
                    val provider = BasicCredentialsProvider()
                    val auth = config.auth as AuthenticationStrategy.Basic

                    provider.setCredentials(
                        AuthScope.ANY,
                        UsernamePasswordCredentials(
                            auth.username,
                            auth.password,
                        ),
                    )

                    hc.setDefaultCredentialsProvider(provider)
                }

                is AuthenticationStrategy.ApiKey -> {
                    val auth = config.auth as AuthenticationStrategy.ApiKey
                    hc.setDefaultHeaders(
                        listOf(
                            BasicHeader("Authorization", "ApiKey ${auth.key}"),
                        ),
                    )
                }

                else -> {}
            }

            if (config.ssl != null) {
                log.info("Configuring SSL for this ES client...")

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
                    runBlocking {
                        withContext(Dispatchers.IO) {
                            keystore.load(file.inputStream(), config.ssl?.keystorePassword?.toCharArray())
                        }
                    }

                    val sslContext = SSLContexts.custom()
                        .loadKeyMaterial(keystore, config.ssl!!.keystorePassword?.toCharArray())
                        .build()

                    if (!config.ssl!!.validateHostnames) {
                        hc.setSSLHostnameVerifier(NoopHostnameVerifier.INSTANCE)
                    }

                    hc.setSSLContext(sslContext)
                } else {
                    log.info("Assuming current path is a certificate!")

                    val factory = CertificateFactory.getInstance("X.509")
                    val trustedCa = factory.generateCertificate(file.inputStream())
                    val trustStore = KeyStore.getInstance("pkcs12")

                    runBlocking {
                        withContext(Dispatchers.IO) {
                            trustStore.load(null, config.ssl!!.keystorePassword?.toCharArray())
                        }
                    }

                    trustStore.setCertificateEntry("ca", trustedCa)

                    val sslContext = SSLContexts.custom()
                        .loadKeyMaterial(trustStore, config.ssl!!.keystorePassword?.toCharArray())
                        .build()

                    if (!config.ssl!!.validateHostnames) {
                        hc.setSSLHostnameVerifier(NoopHostnameVerifier.INSTANCE)
                    }

                    hc.setSSLContext(sslContext)
                }
            }

            hc
        }

        val llc = builder.build()
        val sniffer = Sniffer.builder(llc).setNodesSniffer(
            ElasticsearchNodesSniffer(
                llc,
                ElasticsearchNodesSniffer.DEFAULT_SNIFF_REQUEST_TIMEOUT,
                if (config.ssl != null) ElasticsearchNodesSniffer.Scheme.HTTPS else ElasticsearchNodesSniffer.Scheme.HTTP,
            ),
        ).setSniffAfterFailureDelayMillis(30000).build()
        listener.setSniffer(sniffer)

        sw.suspend()
        log.info("Built low-level REST client in ${sw.doFormatTime()}, checking availability...")
        sw.resume()

        val objectMapper = ObjectMapper().registerKotlinModule().registerModule(KotlinxDatetimeJacksonModule())
        val transport = RestClientTransport(llc, JacksonJsonpMapper(objectMapper))
        _client.value = ElasticsearchAsyncClient(transport)

        sw.suspend()
        log.info("Built high-level REST client in ${sw.doFormatTime()}!")
        sw.resume()

        val info = client.info().await()
        sw.stop()

        log.info("Reached to cluster ${info.clusterName()} [${info.clusterUuid()}] with ES version v${info.version().number()} (${info.version().buildType()}/${info.version().buildFlavor()}) in ${sw.doFormatTime()}")
        _clusterInfo.value = info.clusterName() to info.clusterUuid()
        _serverVersion.value = info.version().number()

        try {
            log.info("Starting to index documents...")
            createOrUpdateIndices()

            ChartedScope.launch {
                indexAllData()
            }
        } catch (e: Exception) {
            ifSentryEnabled { Sentry.captureException(e) }
            log.error("Unable to index all documents into Elasticsearch, data might be loss!", e)
        }
    }

    @Traceable(operation = "metrics.collect")
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
            val shards = stats.indices()?.shardStats()?.totalCount() ?: 0L
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
            it.document(user)
            it.index("charted-users")

            it
        }.await()

        log.info("Successfully indexed user @${user.username} [${user.id}] (version ${resp.version()})")
    }

    override suspend fun indexRepository(repository: Repository) {
        log.info("Now indexing repository ${repository.name} [${repository.id}] into Elasticsearch!")

        val resp = client.index {
            it.document(repository)
            it.index("charted-repositories")

            it
        }.await()

        log.info("Successfully indexed repository ${repository.name} [${repository.id}] (version ${resp.version()})")
    }

    override suspend fun indexOrganization(org: Organization) {
        log.info("Now indexing organization ${org.name} [${org.id}] into Elasticsearch!")

        val resp = client.index {
            it.document(org)
            it.index("charted-organizations")

            it
        }.await()

        log.info("Successfully indexed repository ${org.name} [${org.id}] (version ${resp.version()})")
    }

    override suspend fun unindexOrganization(org: Organization) {
        log.warn("Un-indexing repository ${org.name} [${org.id}]")
        client.delete {
            it.index("charted-organizations")
            it.id(org.id.toString())

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

    override suspend fun unindexUser(user: User) {
        log.warn("Un-indexing user @${user.username} [${user.id}]")

        client.delete {
            it.index("charted-users")
            it.id(user.id.toString())

            it
        }.await()
    }

    override suspend fun indexAllData() {
        if (!TransactionManager.isInitialized()) {
            log.warn("Missing [TransactionManager], will not perform indexing")
            return
        }

        log.info("Performing indexing on indexes [${indexes.joinToString(", ")}]")
        val sw = StopWatch.createStarted()
        for (index in indexes) {
            when (index) {
                "charted-users" -> {
                    if (sw.isSuspended) sw.resume()

                    val chunkSize = Integer.parseInt(System.getProperty("org.noelware.charted.elasticsearch.chunkSize", "1000"))
                    val users = asyncTransaction {
                        UserEntity.all().chunked(chunkSize)
                    }

                    // If there is no users available, we will not do anything
                    if (users.isEmpty()) continue
                    for ((idx, all) in users.withIndex()) {
                        log.info("Inserting ${all.size} users from chunk #$idx")

                        val baos = ByteArrayOutputStream()
                        for (user in all) {
                            withContext(Dispatchers.IO) {
                                baos.write("""{"index":{"_id":${user.id.value}}}""".toByteArray())
                                baos.write('\n'.code)
                                baos.write(json.encodeToString(User.fromEntity(user)).toByteArray())
                                baos.write('\n'.code)
                            }
                        }

                        runBulkRequest(sw, index, baos)
                    }
                }

                "charted-repositories" -> {
                    if (sw.isSuspended) sw.resume()

                    val chunkSize = Integer.parseInt(System.getProperty("org.noelware.charted.elasticsearch.chunkSize", "1000"))
                    val repos = asyncTransaction {
                        RepositoryEntity.all().chunked(chunkSize)
                    }

                    if (repos.isEmpty()) continue
                    for ((idx, all) in repos.withIndex()) {
                        log.info("Inserting ${all.size} repositories from chunk #$idx")

                        val baos = ByteArrayOutputStream()
                        for (repo in all) {
                            withContext(Dispatchers.IO) {
                                baos.write("""{"index":{"_id":${repo.id.value}}}""".toByteArray())
                                baos.write('\n'.code)
                                baos.write(json.encodeToString(Repository.fromEntity(repo)).toByteArray())
                                baos.write('\n'.code)
                            }
                        }

                        runBulkRequest(sw, index, baos)
                    }
                }

                "charted-organizations" -> {
                    if (sw.isSuspended) sw.resume()

                    val chunkSize = Integer.parseInt(System.getProperty("org.noelware.charted.elasticsearch.chunkSize", "1000"))
                    val orgs = asyncTransaction {
                        OrganizationEntity.all().chunked(chunkSize)
                    }

                    if (orgs.isEmpty()) continue
                    for ((idx, all) in orgs.withIndex()) {
                        log.info("Inserting ${all.size} repositories from chunk #$idx")

                        val baos = ByteArrayOutputStream()
                        for (org in all) {
                            withContext(Dispatchers.IO) {
                                baos.write("""{"index":{"_id":${org.id.value}}}""".toByteArray())
                                baos.write('\n'.code)
                                baos.write(json.encodeToString(Organization.fromEntity(org)).toByteArray())
                                baos.write('\n'.code)
                            }
                        }

                        runBulkRequest(sw, index, baos)
                    }
                }

                else -> log.warn("Index {$index} doesn't support indexing at this time")
            }
        }
    }

    override fun close() {
        if (_closed.compareAndSet(expect = false, update = true)) {
            log.warn("Closing off REST client...")
            _client.value._transport().closeQuietly()
        }
    }

    private suspend fun createOrUpdateIndices() {
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

    private suspend fun runBulkRequest(
        sw: StopWatch,
        index: String,
        baos: ByteArrayOutputStream
    ) {
        val restClient: RestClient = (client._transport() as RestClientTransport).restClient()
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
}
