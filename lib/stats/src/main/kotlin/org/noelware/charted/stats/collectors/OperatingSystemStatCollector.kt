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
import org.noelware.charted.common.OperatingSystem
import org.noelware.charted.stats.StatCollector
import java.io.File
import java.lang.management.ManagementFactory
import java.nio.charset.Charset

@kotlinx.serialization.Serializable
data class OsStats(
    @SerialName("system_load_avg")
    val systemLoadAverage: Double,
    val processors: Int,
    val version: String,
    val distro: String?,
    val arch: String,
    val name: String
)

class OperatingSystemStatCollector: StatCollector<OsStats> {
    private val os = ManagementFactory.getOperatingSystemMXBean()

    override suspend fun collect(): OsStats {
        val distro: String? = if (OperatingSystem.current().isLinux) {
            val releases = File("/etc/os-release").readText(Charset.defaultCharset())
            val data = releases.split("\n\r?".toRegex()).associate { value ->
                val (key, v) = value.split("=")

                key to v.replace("\"", "")
            }

            "${data["NAME"]} (${data["ID"]})"
        } else {
            null
        }

        return OsStats(
            os.systemLoadAverage,
            os.availableProcessors,
            os.version,
            distro,
            os.arch,
            os.name
        )
    }
}
