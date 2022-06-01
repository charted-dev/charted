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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.common.config

import kotlinx.serialization.SerialName

/**
 * The main configuration file to configure **charted-server**. This file must be
 * in a YAML file.
 *
 * @param jwtSecretKey The secret key for all JWT tokens.
 * @param registrations If registrations are enabled on the server. If not, only the administrators can create
 * accounts.
 *
 * @param inviteOnly If the server is in "invite-only," in which you will need to have an invitation link to register.
 * @param clickhouse Configuration for ClickHouse, where data like audit logs and webhook events are stored.
 * @param sentryDsn The DSN to configure Sentry.
 * @param postgres The configuration for setting up PostgreSQL, the main database.
 * @param storage The configuration for setting up storage.
 * @param metrics If Prometheus metrics should be enabled.
 */
@kotlinx.serialization.Serializable
data class Config(
    val registrations: Boolean = true,

    @SerialName("jwt_secret_key")
    val jwtSecretKey: String = "",

    @SerialName("invite_only")
    val inviteOnly: Boolean = false,
    val clickhouse: ClickHouseConfig = ClickHouseConfig(),
    val analytics: AnalyticsConfig = AnalyticsConfig(),

    @SerialName("sentry_dsn")
    val sentryDsn: String? = null,

    @SerialName("oci_proxy")
    val ociProxy: OciProxyConfig = OciProxyConfig(),
    val postgres: PostgresConfig = PostgresConfig(),
    val storage: StorageConfig = StorageConfig(),
    val metrics: Boolean = true,
    val search: SearchConfig = SearchConfig(),
    val server: ServerConfig = ServerConfig(),
    val engine: ChartEngineClass = ChartEngineClass.CHARTS,
    val debug: Boolean = false,
    val redis: RedisConfig = RedisConfig()
)

@kotlinx.serialization.Serializable
enum class ChartEngineClass {
    @SerialName("oci")
    OCI,

    @SerialName("charts")
    CHARTS;
}
