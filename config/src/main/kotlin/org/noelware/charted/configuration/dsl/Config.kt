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

package org.noelware.charted.configuration.dsl

import kotlinx.serialization.SerialName
import org.noelware.charted.configuration.dsl.features.Feature
import org.noelware.charted.configuration.dsl.features.KtorSSLConfig
import org.noelware.charted.configuration.dsl.features.NoelwareAnalyticsConfig
import org.noelware.charted.configuration.dsl.features.SMTPConfig
import org.noelware.charted.configuration.dsl.search.SearchConfig
import org.noelware.charted.configuration.dsl.sessions.SessionConfig
import org.noelware.charted.configuration.dsl.tracing.TracingConfig

@kotlinx.serialization.Serializable
data class Config(
    val registrations: Boolean = true,

    @SerialName("jwt_secret_key")
    val jwtSecretKey: String = "",

    @SerialName("invite_only")
    val inviteOnly: Boolean = false,
    val telemetry: Boolean = false,

    @SerialName("sentry_dsn")
    val sentryDsn: String? = null,

    @SerialName("base_url")
    val baseUrl: String? = null,
    val metrics: Boolean = true,
    val debug: Boolean = false,
    val cdn: Boolean = false,

    val analytics: NoelwareAnalyticsConfig? = null,
    val cassandra: CassandraConfig? = null,
    val features: List<Feature> = listOf(),
    val postgres: DatabaseConfig = DatabaseConfig(),
    val sessions: SessionConfig = SessionConfig(),
    val storage: StorageConfig = StorageConfig(),
    val tracing: TracingConfig = TracingConfig(),
    val search: SearchConfig = SearchConfig(),
    val server: KtorServerConfig = KtorServerConfig(),
    val redis: RedisConfig = RedisConfig(),
    val email: SMTPConfig? = null,
    val ssl: KtorSSLConfig? = null
) {
    fun isFeatureEnabled(feature: Feature) = features.contains(feature)
}
