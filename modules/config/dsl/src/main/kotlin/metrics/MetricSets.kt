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

package org.noelware.charted.configuration.kotlin.dsl.metrics

import kotlinx.serialization.Serializable
import org.noelware.charted.configuration.kotlin.dsl.metrics.keys.ElasticsearchMetricKeys
import org.noelware.charted.configuration.kotlin.dsl.metrics.keys.PostgresMetricKeys
import org.noelware.charted.configuration.kotlin.dsl.metrics.keys.RedisMetricKeys

@Serializable
public data class MetricSets(
    val elasticsearch: List<ElasticsearchMetricKeys> = listOf(),
    val postgres: List<PostgresMetricKeys> = listOf(),
    val redis: List<RedisMetricKeys> = listOf()
) {
    public class Builder : org.noelware.charted.common.Builder<MetricSets> {
        private val _elasticsearch: MutableList<ElasticsearchMetricKeys> = mutableListOf()
        private val _postgres: MutableList<PostgresMetricKeys> = mutableListOf()
        private val _redis: MutableList<RedisMetricKeys> = mutableListOf()

        public fun elasticsearch(vararg keys: ElasticsearchMetricKeys): Builder {
            _elasticsearch.addAll(keys)
            return this
        }

        public fun postgres(key: PostgresMetricKeys): Builder {
            _postgres.add(key)
            return this
        }

        public fun redis(key: RedisMetricKeys): Builder {
            _redis.add(key)
            return this
        }

        override fun build(): MetricSets = MetricSets(_elasticsearch, _postgres, _redis)
    }
}
