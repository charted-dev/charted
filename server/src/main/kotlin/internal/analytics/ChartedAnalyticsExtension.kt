/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.server.internal.analytics

import com.google.protobuf.Value
import kotlinx.coroutines.runBlocking
import org.noelware.analytics.jvm.server.extensions.Extension
import org.noelware.analytics.jvm.server.serialization.Serializable
import org.noelware.charted.databases.postgres.metrics.PostgresServerStats
import org.noelware.charted.modules.analytics.kotlin.dsl.*
import org.noelware.charted.modules.elasticsearch.metrics.ElasticsearchStats
import org.noelware.charted.modules.metrics.MetricsSupport
import org.noelware.charted.modules.metrics.collectors.JvmProcessInfoMetrics
import org.noelware.charted.modules.metrics.collectors.JvmThreadsMetrics
import org.noelware.charted.modules.metrics.collectors.OperatingSystemMetrics
import org.noelware.charted.modules.redis.metrics.RedisServerStats

class ChartedAnalyticsExtension(private val metrics: MetricsSupport) : Extension<ChartedAnalyticsExtension.Data> {
    /**
     * Returns the name of this [Extension] to be used in the final result when
     * sending out this extension's data.
     */
    override fun name(): String = "server"

    /**
     * This method is called to supply the data that is available to be ingested to the Analytics Server
     * or any other third-party you allow.
     */
    override fun supply(): Data {
        val stats = runBlocking { metrics.collect() }
        return Data(
            if (stats.containsKey("elasticsearch")) stats["elasticsearch"] as ElasticsearchStats else null,
            stats["postgres"] as PostgresServerStats,
            stats["process"] as JvmProcessInfoMetrics,
            stats["threads"] as JvmThreadsMetrics,
            stats["redis"] as RedisServerStats,
            stats["os"] as OperatingSystemMetrics,
        )
    }

    data class Data(
        val elasticsearch: ElasticsearchStats?,
        val postgres: PostgresServerStats,
        val process: JvmProcessInfoMetrics,
        val threads: JvmThreadsMetrics,
        val redis: RedisServerStats,
        val os: OperatingSystemMetrics
    ) : Serializable {
        override fun toGrpcValue(): Value = Struct {
            put(this, Data::elasticsearch)
            put(this, Data::postgres)
            put(this, Data::redis)
        }.toGrpcValue()
    }
}
