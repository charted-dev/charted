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

package org.noelware.charted.stats.collectors

import kotlinx.serialization.SerialName
import org.noelware.charted.common.extensions.formatToSize
import org.noelware.charted.configuration.dsl.*
import org.noelware.charted.configuration.dsl.features.Feature
import org.noelware.charted.stats.StatCollector
import org.noelware.remi.core.StorageTrailer
import org.noelware.remi.filesystem.FilesystemStorageTrailer

@kotlinx.serialization.Serializable
data class StorageTrailerStats(
    @SerialName("fs_stats")
    val filesystemStats: FilesystemStats? = null,

    @SerialName("charts_size")
    val chartsSize: Long? = null,
    val name: String
)

@kotlinx.serialization.Serializable
data class FilesystemStats(
    @SerialName("unallocated_space_bytes")
    val unallocatedSpaceBytes: Long,

    @SerialName("unallocated_space")
    val unallocatedSpace: String,

    @SerialName("usable_space_bytes")
    val usableSpaceBytes: Long,

    @SerialName("usable_space")
    val usableSpace: String,

    @SerialName("total_space_bytes")
    val totalSpaceBytes: Long,

    @SerialName("total_space")
    val totalSpace: String,
    val directory: String,
    val drive: String,
    val type: String
)

class StorageTrailerStatCollector(private val trailer: StorageTrailer<*>, private val config: Config): StatCollector<StorageTrailerStats> {
    override suspend fun collect(): StorageTrailerStats {
        val chartsSize = if (config.isFeatureEnabled(Feature.DOCKER_REGISTRY)) {
            null
        } else {
            trailer.list("./tarballs").fold(0L) { acc, o -> acc + o.size }
        }

        val fsStats = if (trailer is FilesystemStorageTrailer) {
            val stats = trailer.stats()
            FilesystemStats(
                stats.unallocatedSpace,
                stats.unallocatedSpace.formatToSize(),
                stats.usableSpace,
                stats.usableSpace.formatToSize(),
                stats.totalSpace,
                stats.totalSpace.formatToSize(),
                trailer.directory,
                stats.drive,
                stats.type
            )
        } else {
            null
        }

        return StorageTrailerStats(
            fsStats,
            chartsSize,
            trailer.name
        )
    }
}
