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

package org.noelware.charted.server.endpoints.v1.api.admin

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.ChartedInfo
import org.noelware.charted.databases.postgres.metrics.PostgresServerStats
import org.noelware.charted.modules.elasticsearch.metrics.ElasticsearchStats
import org.noelware.charted.modules.metrics.collectors.JvmProcessInfoMetrics
import org.noelware.charted.modules.metrics.collectors.JvmThreadsMetrics
import org.noelware.charted.modules.metrics.collectors.OperatingSystemMetrics
import org.noelware.charted.modules.metrics.collectors.ServerInfoMetrics
import org.noelware.charted.modules.redis.metrics.RedisServerStats

@Serializable
data class MainAdminResponse(
    val message: String = "Welcome to the Admin API!",

    @SerialName("docs_url")
    val docsUrl: String = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/admin"
)

@Serializable
data class AdminStatsResponse(
    val elasticsearch: ElasticsearchStats? = null,
    val postgres: PostgresServerStats? = null,
    val threads: JvmThreadsMetrics? = null,
    val process: JvmProcessInfoMetrics? = null,
    val server: ServerInfoMetrics,
    val redis: RedisServerStats? = null,
    val os: OperatingSystemMetrics? = null
)
