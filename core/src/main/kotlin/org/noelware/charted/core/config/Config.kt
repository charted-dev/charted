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

package org.noelware.charted.core.config

import kotlinx.serialization.SerialName

@kotlinx.serialization.Serializable
data class Config(
    @SerialName("jwt_secret_key")
    val jwtSecretKey: String = "",

    @SerialName("registrations")
    val registrations: Boolean = true,

    @SerialName("invite_only")
    val inviteOnly: Boolean = false,

    @SerialName("sentry_dsn")
    val sentryDsn: String? = null,
    val analytics: AnalyticsConfig? = null,
    val database: PostgresConfig = PostgresConfig(),
    val instatus: InstatusConfig? = null,
    val storage: StorageConfig? = null,
    val server: KtorServerConfig = KtorServerConfig(),
    val engine: ChartEngineConfig? = null,
    val search: SearchConfig = SearchConfig(),
    val redis: RedisConfig = RedisConfig()
)
