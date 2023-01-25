package org.noelware.charted.modules.metrics.collectors

import com.google.protobuf.Value
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.DistributionType
import org.noelware.charted.modules.analytics.kotlin.dsl.Struct
import org.noelware.charted.modules.analytics.kotlin.dsl.put
import org.noelware.charted.modules.analytics.kotlin.dsl.toGrpcValue

@Serializable
data class ServerInfoMetrics(
    val distribution: DistributionType,

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
        put(this, ServerInfoMetrics::requests)
        put(this, ServerInfoMetrics::buildDate)
        put(this, ServerInfoMetrics::product)
        put(this, ServerInfoMetrics::version)
        put(this, ServerInfoMetrics::vendor)
    }.toGrpcValue()
}
