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

class ElasticSearchBackend

// import co.elastic.clients.elasticsearch.ElasticsearchClient
// import co.elastic.clients.elasticsearch._types.HealthStatus
// import co.elastic.clients.elasticsearch.core.SearchRequest
// import co.elastic.clients.json.jackson.JacksonJsonpMapper
// import co.elastic.clients.transport.rest_client.RestClientTransport
// import dev.floofy.utils.slf4j.logging
// import org.apache.http.HttpHost
// import org.elasticsearch.client.RestClient
// import org.noelware.charted.search.ISearchBackend
// import org.noelware.charted.search.Indexable
// import org.noelware.charted.search.SearchBuilder
//
// class ElasticSearchBackend(config: ElasticsearchConfig): ISearchBackend {
//    private val log by logging<ElasticSearchBackend>()
//    private val client: ElasticsearchClient
//
//    init {
//        log.debug("Creating connection to Elasticsearch with nodes [${config.nodes.joinToString(", ")}]")
//
//        val restClient = (if (config.cloudId != null) {
//            RestClient.builder(config.cloudId)
//        } else {
//            val httpHosts = config.nodes.map { val (host, port) = it.split(":"); HttpHost(host, Integer.parseInt(port)) }
//            RestClient.builder(*httpHosts.toTypedArray())
//        }).build()
//
//        val transport = RestClientTransport(restClient, JacksonJsonpMapper())
//        client = ElasticsearchClient(transport)
//
//        log.debug("Created client! Now testing connection...")
//        val health = client.cluster().health()
//        log.debug("Cluster ${health.clusterName()}'s health is currently ${health.status().jsonValue()}")
//
//        if (health.status() != HealthStatus.Green) {
//            log.debug("Cluster ${health.clusterName()}'s current health status: ${health.status().jsonValue()}! Some data will be unavailable.")
//        }
//    }
//
//    /**
//     * Creates a search and returns a list of all the indexables that the
//     * backend found.
//     *
//     * @param query The query that the user requested
//     * @param search The builder object to add additional values.
//     * @return A list of [T].
//     */
//    override fun <T: Indexable> search(index: String, query: String, search: SearchBuilder.() -> Unit): List<T> {
//        throw IllegalStateException("owo")
//    }
//
//    /*
//    SearchResponse<Product> response = esClient.search(s -> s
//    .index("products")
//    .query(q -> q
//        .match(t -> t
//            .field("name")
//            .query(searchText)
//        )
//    ),
//    Product.class
// );
//
// TotalHits total = response.hits().total();
// boolean isExactResult = total.relation() == TotalHitsRelation.Eq;
//
// if (isExactResult) {
//    logger.info("There are " + total.value() + " results");
// } else {
//    logger.info("There are more than " + total.value() + " results");
// }
//
// List<Hit<Product>> hits = response.hits().hits();
// for (Hit<Product> hit: hits) {
//    Product product = hit.source();
//    logger.info("Found product " + product.getSku() + ", score " + hit.score());
// }
//     */
//
//    /**
//     * Indexes an indexable object into the search backend.
//     * @param index The index to insert it into
//     * @param data The data object to insert.
//     */
//    override fun <T : Indexable> index(index: String, data: T) {
//        TODO("Not yet implemented")
//    }
//
//    /**
//     * Queries all the data from PostgreSQL and Redis to index all users,
//     * repositories, organizations, and organization/repository members.
//     */
//    override fun indexAll() {
//        TODO("Not yet implemented")
//    }
//
//    /**
//     * Closes this stream and releases any system resources associated
//     * with it. If the stream is already closed then invoking this
//     * method has no effect.
//     *
//     *
//     *  As noted in [AutoCloseable.close], cases where the
//     * close may fail require careful attention. It is strongly advised
//     * to relinquish the underlying resources and to internally
//     * *mark* the `Closeable` as closed, prior to throwing
//     * the `IOException`.
//     *
//     * @throws java.io.IOException if an I/O error occurs
//     */
//    override fun close() {
//        TODO("Not yet implemented")
//    }
// }
