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

package org.noelware.charted.modules.metrics.collectors

import com.google.protobuf.Value
import io.ktor.server.application.*
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.ChartedInfo
import org.noelware.charted.modules.analytics.kotlin.dsl.Struct
import org.noelware.charted.modules.analytics.kotlin.dsl.put
import org.noelware.charted.modules.analytics.kotlin.dsl.toGrpcValue

@Serializable
data class ServerInfoMetrics(
    val distribution: ChartedInfo.Distribution,

    @SerialName("ktor_version")
    val ktorVersion: String,

    @SerialName("commit_sha")
    val commitHash: String,
    val requests: Long,

    @SerialName("build_date")
    val buildDate: String,
    val product: String,
    val version: String,
    val vendor: String
): org.noelware.analytics.jvm.server.serialization.Serializable {
    override fun toGrpcValue(): Value = Struct {
        put(this, ServerInfoMetrics::distribution)
        put(this, ServerInfoMetrics::ktorVersion)
        put(this, ServerInfoMetrics::commitHash)
        put(this, ServerInfoMetrics::buildDate)
        put(this, ServerInfoMetrics::requests)
        put(this, ServerInfoMetrics::product)
        put(this, ServerInfoMetrics::version)
        put(this, ServerInfoMetrics::vendor)
    }.toGrpcValue()

    class Collector(private val getRequestCounter: () -> Long): org.noelware.charted.modules.metrics.Collector<ServerInfoMetrics> {
        override val name: String = "server"
        override suspend fun supply(): ServerInfoMetrics {
            val ktorVersion = Application::class.java.`package`.implementationVersion!!
            return ServerInfoMetrics(
                ChartedInfo.distribution,
                ktorVersion,
                ChartedInfo.commitHash,
                getRequestCounter(),
                ChartedInfo.buildDate,
                "charted-server",
                ChartedInfo.version,
                "Noelware, LLC.",
            )
        }
    }
}
