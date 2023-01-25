package org.noelware.charted.server.internal.metrics

import org.noelware.charted.ChartedInfo
import org.noelware.charted.modules.metrics.Collector
import org.noelware.charted.modules.metrics.collectors.ServerInfoMetrics
import org.noelware.charted.server.requests

object ServerInfoMetricsCollector: Collector<ServerInfoMetrics> {
    override val name: String = "server"
    override suspend fun supply(): ServerInfoMetrics = ServerInfoMetrics(
        ChartedInfo.distribution,
        "???",
        ChartedInfo.commitHash,
        requests,
        ChartedInfo.buildDate,
        "charted-server",
        ChartedInfo.version,
        "Noelware, LLC.",
    )
}
