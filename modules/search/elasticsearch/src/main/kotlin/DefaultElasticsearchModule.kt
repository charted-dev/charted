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
import dev.floofy.utils.java.SetOnce
import dev.floofy.utils.slf4j.logging
import kotlinx.atomicfu.AtomicBoolean
import kotlinx.atomicfu.atomic
import kotlinx.serialization.json.Json
import org.apache.commons.lang3.time.StopWatch
import org.noelware.charted.common.extensions.closeable.closeQuietly
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.models.organizations.Organization
import org.noelware.charted.models.repositories.Repository
import org.noelware.charted.models.users.User
import org.noelware.charted.modules.search.elasticsearch.metrics.ElasticsearchStats
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

    override suspend fun indexUser(user: User) {
        TODO("Not yet implemented")
    }

    override suspend fun indexRepository(repository: Repository) {
        TODO("Not yet implemented")
    }

    override suspend fun indexOrganization(org: Organization) {
        TODO("Not yet implemented")
    }

    override suspend fun indexAllData() {
        TODO("Not yet implemented")
    }

    override suspend fun stats(): ElasticsearchStats {
        TODO("Not yet implemented")
    }

    override val serverVersion: String
        get() = _serverVersion.value

    /**
     * Returns the Elasticsearch cluster's name that was collected when the client was
     * being connected.
     */
    val clusterName: String
        get() = _clusterInfo.value.first

    /**
     * Returns the Elasticsearch cluster's UUId that was collected when the
     * client was being collected.
     */
    val clusterUUID: String
        get() = _clusterInfo.value.second

    override suspend fun init() {
        if (closed) {
            log.warn("Elasticsearch module is closed and the connection is no longer available.")
            return
        }

        val sw = StopWatch.createStarted()
        log.info("Creating low-level REST client...")
    }

    override fun close() {
        if (_closed.compareAndSet(expect = false, update = true)) {
            log.warn("Closing off REST client...")
            _client.value._transport().closeQuietly()
        }
    }
}
