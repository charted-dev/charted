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
import org.noelware.charted.modules.elasticsearch.metrics.ElasticsearchStats
import java.io.Closeable

/**
 * Represents a module for interacting with Elasticsearch for search capabilities.
 */
interface ElasticsearchModule: Closeable {
    /** Returns all the indexes that Elasticsearch is responsible for */
    val indexes: List<String>

    /**
     * Returns the Elasticsearch server version that the cluster is running on. charted-server requires the
     * cluster to be using Elasticsearch 8.
     */
    val serverVersion: String

    /**
     * Returns the Elasticsearch cluster's name that was collected when the client was
     * being connected.
     */
    val clusterName: String

    /**
     * Returns the Elasticsearch cluster's UUId that was collected when the
     * client was being collected.
     */
    val clusterUUID: String

    /**
     * Returns if the service was currently closed.
     */
    val closed: Boolean

    /**
     * Returns a reference of the [asynchronous client][ElasticsearchAsyncClient].
     */
    val client: ElasticsearchAsyncClient

    /** Connects to Elasticsearch! */
    suspend fun connect()

    /**
     * Returns the [statistics object][ElasticsearchStats] for the Elasticsearch cluster that is
     * used for this interface.
     */
    suspend fun stats(): ElasticsearchStats
}
