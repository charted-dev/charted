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

package org.noelware.charted.common.data

import kotlinx.serialization.SerialName

@kotlinx.serialization.Serializable
data class Config(
    val registrations: Boolean = true,

    @SerialName("jwt_secret_key")
    val jwtSecretKey: String = "",

    @SerialName("invite_only")
    val inviteOnly: Boolean = false,
    val telemetry: Boolean = false,
    val analytics: AnalyticsConfig? = null,
    val cassandra: CassandraConfig? = null,

    @SerialName("sentry_dsn")
    val sentryDsn: String? = null,

    @SerialName("base_url")
    val baseUrl: String? = null,

    @SerialName("oci_proxy")
    val ociProxy: OciProxyConfig? = null,
    val features: List<Feature> = listOf(),
    val postgres: PostgresConfig = PostgresConfig(),
    val sessions: SessionsConfig = SessionsConfig(),
    val storage: StorageConfig = StorageConfig(),
    val metrics: Boolean = true,
    val search: SearchConfig = SearchConfig(),
    val server: KtorServerConfig = KtorServerConfig(),
    val debug: Boolean = false,
    val redis: RedisConfig = RedisConfig(),
    val ssl: SSLKeystoreConfig? = null,
    val cdn: CdnConfig = CdnConfig()
) {
    fun isFeatureEnabled(feature: Feature): Boolean = features.contains(feature)
}
