package org.noelware.charted.server.metrics

import org.noelware.charted.database.clickhouse.ClickHouseConnection

class PrometheusHandler(private val clickhouse: ClickHouseConnection? = null)
