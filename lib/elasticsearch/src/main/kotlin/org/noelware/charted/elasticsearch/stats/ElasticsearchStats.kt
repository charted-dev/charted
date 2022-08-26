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

package org.noelware.charted.elasticsearch.stats

import kotlinx.serialization.SerialName

@kotlinx.serialization.Serializable
data class ElasticsearchStats(
    @SerialName("size_in_bytes")
    val sizeInBytes: Long,
    val documents: Long,
    val deleted: Long,
    val indexes: Map<String, IndexStat>,
    val health: String,
    val memory: MemoryStat
) {
    @kotlinx.serialization.Serializable
    data class IndexStat(
        @SerialName("size_in_bytes")
        val sizeInBytes: Long,
        val documents: Long,
        val deleted: Long,
        val health: String
    )

    @kotlinx.serialization.Serializable
    data class MemoryStat(
        @SerialName("total_bytes")
        val totalBytes: Long,

        @SerialName("free_bytes")
        val freeBytes: Long,

        @SerialName("used_bytes")
        val usedBytes: Long
    )
}
