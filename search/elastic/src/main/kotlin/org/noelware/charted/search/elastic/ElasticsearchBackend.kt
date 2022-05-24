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

import dev.floofy.utils.exposed.asyncTransaction
import dev.floofy.utils.koin.inject
import dev.floofy.utils.slf4j.logging
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withContext
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toInstant
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.json.*
import org.apache.http.HttpHost
import org.apache.http.auth.AuthScope
import org.apache.http.auth.UsernamePasswordCredentials
import org.apache.http.entity.ByteArrayEntity
import org.apache.http.entity.ContentType
import org.apache.http.entity.StringEntity
import org.apache.http.impl.client.BasicCredentialsProvider
import org.apache.http.ssl.SSLContexts
import org.elasticsearch.client.*
import org.elasticsearch.client.sniff.SniffOnFailureListener
import org.elasticsearch.client.sniff.Sniffer
import org.jetbrains.exposed.dao.with
import org.jetbrains.exposed.sql.selectAll
import org.noelware.charted.database.entity.OrganizationMemberEntity
import org.noelware.charted.database.entity.RepositoryMemberEntity
import org.noelware.charted.database.tables.Organizations
import org.noelware.charted.database.tables.Repositories
import org.noelware.charted.database.tables.Users
import org.noelware.charted.search.elastic.interceptor.ApacheSentryRequestInterceptor
import org.noelware.charted.search.elastic.interceptor.ApacheSentryResponseInterceptor
import java.io.ByteArrayOutputStream
import java.io.Closeable
import java.nio.file.Files
import java.nio.file.Paths
import java.security.KeyStore
import java.security.cert.CertificateFactory

@OptIn(ExperimentalSerializationApi::class)
class ElasticsearchBackend(config: ElasticsearchConfig): Closeable {
    private val log by logging<ElasticsearchBackend>()
    private val json by inject<Json>()

    val serverVersion: String
    val clusterName: String
    val clusterUUID: String

    private val client: RestClient

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

        val info = client.performRequest(Request("GET", "/"))
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

        runBlocking {
            createIndexes()
            indexAllData()
        }
    }

    override fun close() {
        log.warn("Disconnecting from Elasticsearch...")
        client.close()
    }

    private suspend fun createIndexes() {
        log.info("Creating Elasticsearch indexes if they don't exist...")

        for ((index, settings) in INDEX_SETTINGS) {
            log.info("Does index $index exist?...")
            val res1 = client.performRequest(Request("HEAD", "/$index"))

            // TODO: update index mappings
            if (res1.statusLine.statusCode == 200) {
                log.warn("Index $index already exists! Updating index mappings...")
                val map = INDEX_MAPPINGS_WITHOUT_SETTINGS[index]!!

                val req1 = Request("PUT", "/$index/_mapping")
                req1.setJsonEntity(json.encodeToString(JsonObject.serializer(), map))

                val res2 = client.performRequest(req1)
                if (res2.statusLine.statusCode !in 200..300) {
                    val body = json.decodeFromStream(JsonObject.serializer(), res2.entity.content)
                    log.warn("Unable to create a request to \"PUT /$index/_mapping\" - $body")
                } else {
                    log.info("Index $index's mappings are updated.")
                }

                continue
            }

            log.warn("Index $index doesn't exist!")
            val request = Request("PUT", "/$index")
            request.setJsonEntity(json.encodeToString(JsonObject.serializer(), settings))

            val res2 = client.performRequest(request)
            if (res2.statusLine.statusCode !in 200..300) {
                val body = json.decodeFromStream(JsonObject.serializer(), res2.entity.content)
                log.warn("Unable to create a request to \"PUT /$index\" - $body")
            } else {
                log.info("Index $index now exists in Elasticsearch :D")
            }

            indexAllData()
        }
    }

    fun info(): Map<String, JsonObject> {
        val dataMap = mutableMapOf<String, JsonObject>()
        val res = client.performRequest(Request("GET", "/charted-users,charted-repos,charted-repo-members,charted-org-members,charted-orgs/_stats"))
        val data = json.decodeFromStream(JsonObject.serializer(), res.entity.content)

        for (index in listOf("charted-users", "charted-repos", "charted-repo-members", "charted-orgs", "charted-org-members")) {
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
        query: String,
        limit: Int = 25,
        offset: Int = 0,
        fieldsToRequest: List<String> = listOf(),
        strict: Boolean = false
    ): JsonObject {
        val request = Request("POST", "/charted-*/_search")
        val searchQuery = buildJsonObject {
            put("from", offset)
            put("size", limit)
            put(
                "query",
                buildJsonObject {
                    if (strict || fieldsToRequest.size > 1) {
                        put(
                            "bool",
                            buildJsonObject {
                                putJsonArray("must") {
                                    addJsonObject {
                                        putJsonObject("match") {
                                            for (field in fieldsToRequest) {
                                                put(field, query)
                                            }
                                        }
                                    }
                                }
                            }
                        )
                    } else {
                        put(
                            "query",
                            buildJsonObject {
                                put(
                                    "match",
                                    buildJsonObject {
                                        put(fieldsToRequest.first(), query)
                                    }
                                )
                            }
                        )
                    }
                }
            )
        }

        request.setJsonEntity(json.encodeToString(JsonObject.serializer(), searchQuery))
        val res = client.performRequest(request)

        if (res.statusLine.statusCode !in 200..300) {
            val payload = json.decodeFromStream(JsonObject.serializer(), res.entity.content)
            throw IllegalStateException("Unable to request to POST /charted-*/_search: $payload")
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

    suspend fun indexAllData() {
        log.debug("Indexing all data...")

        for (index in Indexes.all()) {
            log.debug("Indexing data for ${index.index}!")

            when (index) {
                Indexes.ORGANIZATION -> {
                    val data = asyncTransaction {
                        Organizations.selectAll().toList()
                    }

                    if (data.isEmpty()) {
                        log.warn("Skipping index ${index.index} due to no data being available.")
                        continue
                    }

                    val jsonBody = StringBuilder()
                    for (d in data) {
                        val id = d[Organizations.id].value
                        val entity = buildJsonObject {
                            put("name", d[Organizations.displayName])
                            put("handle", d[Organizations.handle])
                            put("owner_id", d[Organizations.owner].value)
                            put("created_at", d[Organizations.createdAt].toInstant(TimeZone.currentSystemDefault()).toString())
                            put("updated_at", d[Organizations.updatedAt].toInstant(TimeZone.currentSystemDefault()).toString())
                            put("verified_publisher", d[Organizations.verifiedPublisher])
                        }

                        jsonBody.appendLine("{\"index\":\"{\"_id\":$id}}")
                        jsonBody.appendLine(json.encodeToString(JsonObject.serializer(), entity))
                    }

                    val request = Request("POST", "/${index.index}/_bulk")
                    request.entity = StringEntity(jsonBody.toString(), "application/json; charset=utf-8")

                    val res = client.performRequest(request)
                    if (res.statusLine.statusCode !in 200..300) {
                        val body = json.decodeFromStream(JsonObject.serializer(), res.entity.content)
                        log.warn("Unable to send a request to \"POST /${index.index}/_bulk\": $body")
                    } else {
                        log.debug("Indexed all data for index [${index.index}]")
                    }
                }

                Indexes.ORGANIZATION_MEMBER -> {
                    val data = asyncTransaction {
                        OrganizationMemberEntity
                            .all()
                            .with(OrganizationMemberEntity::account)
                            .toList()
                    }

                    if (data.isEmpty()) {
                        log.warn("Skipping index ${index.index} due to no data being available.")
                        continue
                    }

                    val jsonBody = StringBuilder()
                    for (d in data) {
                        val id = d.id.value
                        val entity = buildJsonObject {
                            put(
                                "name",
                                if (d.displayName != null)
                                    JsonPrimitive(d.displayName)
                                else if (d.account.name != null)
                                    JsonPrimitive(d.account.name)
                                else
                                    JsonNull
                            )

                            put("username", d.account.username)
                            put("description", d.account.description)
                            put("joined_at", d.joinedAt.toInstant(TimeZone.currentSystemDefault()).toString())
                            put("updated_at", d.updatedAt.toInstant(TimeZone.currentSystemDefault()).toString())
                            put("display_name", d.displayName)
                        }

                        jsonBody.appendLine("{\"index\":\"{\"_id\":$id}}")
                        jsonBody.appendLine(json.encodeToString(JsonObject.serializer(), entity))
                    }

                    val request = Request("POST", "/${index.index}/_bulk")
                    request.entity = StringEntity(jsonBody.toString(), "application/json; charset=utf-8")

                    val res = client.performRequest(request)
                    if (res.statusLine.statusCode !in 200..300) {
                        val body = json.decodeFromStream(JsonObject.serializer(), res.entity.content)
                        log.warn("Unable to send a request to \"POST /${index.index}/_bulk\": $body")
                    } else {
                        log.debug("Indexed all data for index [${index.index}]")
                    }
                }

                Indexes.REPOSITORY -> {
                    val data = asyncTransaction {
                        Repositories.selectAll().toList()
                    }

                    if (data.isEmpty()) {
                        log.warn("Skipping index ${index.index} due to no data being available.")
                        continue
                    }

                    val jsonBody = StringBuilder()
                    for (d in data) {
                        val id = d[Repositories.id].value
                        val entity = buildJsonObject {
                            put("name", d[Repositories.name])
                            put("flags", d[Repositories.flags])
                            put("owner_id", d[Repositories.ownerId])
                            put("deprecated", d[Repositories.deprecated])
                            put("description", d[Repositories.description])
                            put("created_at", d[Repositories.createdAt].toInstant(TimeZone.currentSystemDefault()).toString())
                            put("updated_at", d[Repositories.updatedAt].toInstant(TimeZone.currentSystemDefault()).toString())
                        }

                        jsonBody.appendLine("{\"index\":\"{\"_id\":$id}}")
                        jsonBody.appendLine(json.encodeToString(JsonObject.serializer(), entity))
                    }

                    val request = Request("POST", "/${index.index}/_bulk")
                    request.entity = StringEntity(jsonBody.toString(), "application/json; charset=utf-8")

                    val res = client.performRequest(request)
                    if (res.statusLine.statusCode !in 200..300) {
                        val body = json.decodeFromStream(JsonObject.serializer(), res.entity.content)
                        log.warn("Unable to send a request to \"POST /${index.index}/_bulk\": $body")
                    } else {
                        log.debug("Indexed all data for index [${index.index}]")
                    }
                }

                Indexes.REPOSITORY_MEMBER -> {
                    val data = asyncTransaction {
                        RepositoryMemberEntity
                            .all()
                            .with(RepositoryMemberEntity::account)
                            .toList()
                    }

                    if (data.isEmpty()) {
                        log.warn("Skipping index ${index.index} due to no data being available.")
                        continue
                    }

                    val jsonBody = StringBuilder()
                    for (d in data) {
                        val id = d.id.value
                        val entity = buildJsonObject {
                            put(
                                "name",
                                if (d.displayName != null)
                                    JsonPrimitive(d.displayName)
                                else if (d.account.name != null)
                                    JsonPrimitive(d.account.name)
                                else
                                    JsonNull
                            )

                            put("username", d.account.username)
                            put("description", d.account.description)
                            put("joined_at", d.joinedAt.toInstant(TimeZone.currentSystemDefault()).toString())
                            put("updated_at", d.updatedAt.toInstant(TimeZone.currentSystemDefault()).toString())
                            put("display_name", d.displayName)
                        }

                        jsonBody.appendLine("{\"index\":\"{\"_id\":$id}}")
                        jsonBody.appendLine(json.encodeToString(JsonObject.serializer(), entity))
                    }

                    val request = Request("POST", "/${index.index}/_bulk")
                    request.entity = StringEntity(jsonBody.toString(), "application/json; charset=utf-8")

                    val res = client.performRequest(request)
                    if (res.statusLine.statusCode !in 200..300) {
                        val body = json.decodeFromStream(JsonObject.serializer(), res.entity.content)
                        log.warn("Unable to send a request to \"POST /${index.index}/_bulk\": $body")
                    } else {
                        log.debug("Indexed all data for index [${index.index}]")
                    }
                }

                Indexes.USER -> {
                    val data = asyncTransaction {
                        Users.selectAll().toList()
                    }

                    if (data.isEmpty()) {
                        log.warn("Skipping index ${index.index} due to no data being available.")
                        continue
                    }

                    val baos = ByteArrayOutputStream()
                    for (d in data) {
                        val id = d[Users.id].value
                        val entity = buildJsonObject {
                            put("name", d[Users.name])
                            put("flags", d[Users.flags])
                            put("username", d[Users.username])
                            put("created_at", d[Users.createdAt].toInstant(TimeZone.currentSystemDefault()).toString())
                            put("updated_at", d[Users.updatedAt].toInstant(TimeZone.currentSystemDefault()).toString())
                            put("description", d[Users.description])
                        }

                        withContext(Dispatchers.IO) {
                            baos.write("{\"index\":{\"_id\":\"$id\"}}".toByteArray())
                            baos.write('\n'.code)
                            baos.write(json.encodeToString(JsonObject.serializer(), entity).toByteArray())
                            baos.write('\n'.code)
                        }
                    }

                    val request = Request("POST", "/${index.index}/_bulk")
                    request.entity = ByteArrayEntity(baos.toByteArray(), ContentType.APPLICATION_JSON)

                    val res = client.performRequest(request)
                    if (res.statusLine.statusCode !in 200..300) {
                        val body = json.decodeFromStream(JsonObject.serializer(), res.entity.content)
                        log.warn("Unable to send a request to \"POST /${index.index}/_bulk\": $body")
                    } else {
                        log.debug("Indexed all data for index [${index.index}]")
                    }
                }
            }
        }
    }
}
