/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

@file:Suppress("unused")

package org.noelware.charted.configuration.kotlin.dsl

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.ValidationException
import org.noelware.charted.configuration.kotlin.dsl.features.ServerFeature
import org.noelware.charted.configuration.kotlin.dsl.metrics.MetricsConfig
import org.noelware.charted.configuration.kotlin.dsl.search.SearchConfig
import org.noelware.charted.configuration.kotlin.dsl.sessions.SessionsConfig
import org.noelware.charted.configuration.kotlin.dsl.storage.StorageConfig
import org.noelware.charted.configuration.kotlin.dsl.tracing.TracingConfig

private const val DO_NOT_USE_THIS_VALUE_IN_YOUR_JWT_SECRET_KEY_OR_I_WILL_LAUGH_AT_YOU: String = "__DO NOT USE THIS AS THE SECRET KEY__"

public fun Config.toApiBaseUrl(path: String = "/"): String = when {
    baseUrl != null -> baseUrl + path
    else -> "http${if (server.ssl != null) "s" else ""}://${server.host}:${server.port}$path"
}

public fun Config.toCdnBaseUrl(path: String = "/"): String = when {
    cdn != null && cdn.enabled -> toApiBaseUrl("/${cdn.prefix}$path")
    storage.hostAlias != null -> storage.hostAlias + path
    else -> toApiBaseUrl(path)
}

/**
 * Represents the root configuration object that is loaded via the [ConfigurationHost][org.noelware.charted.configuration.host.ConfigurationHost]
 * that takes care of the initialization of deserializing the configuration to this given object. **charted-server** supports YAML and the experimental
 * Kotlin Script configuration hosts that you can use with:
 *
 * ```kotlin
 * import org.noelware.charted.configuration.kotlin.host.KotlinScriptHost
 * import org.noelware.charted.configuration.yaml.YamlConfigurationHost
 * import org.noelware.charted.configuration.kotlin.dsl.*
 *
 * fun main() {
 *   // Using the experimental Kotlin Script loaded
 *   val config = KotlinScriptHost.load("./path/to/config.charted.kts")
 *   // => org.noelware.charted.configuration.kotlin.dsl.Config?
 *
 *   val config2 = YamlConfigurationHost().load("./path/to/charted.yaml")
 *   // => org.noelware.charted.configuration.kotlin.dsl.Config?
 * }
 * ```
 *
 * @param registrations If the server should openly accept registrations or not. If not, the administrators
 *                      of this instance will have to create users with the administration portal if the
 *                      web UI is enabled or with the `PUT /admin/users` endpoint to create one.
 * @param jwtSecretKey  The secret key used for encoding JSON Web Tokens for all sessions. Please set this variable to something,
 *                      or you will get a thrown exception that the server will laugh at you, it has feelings you know!
 * @param inviteOnly    If the server can also generate invitations on the fly to let users register behalf of that invitation. This
 *                      implies that the `config.smtp` configuration key is also configured and that [registrations] is set to `false`.
 * @param telemetry     If [Noelware Telemetry](https://telemetry.noelware.org) should be enabled on this instance. This is completely
 *                      optional and opt-in if you wish! The Noelware team collects only anonymous packets and nothing that reveals your IP address,
 *                      user information, container metadata, etc. You can view the source of the Telemetry server that Noelware created to combat
 *                      and address those issues: https://github.com/Noelware/telemetry-server.
 * @param sentryDsn     Whether Sentry should be enabled on the server or not. This will send out error reports and tracing-related data to
 *                      the Sentry server that the DSN is based off.
 * @param swaggerUi     If the server should embed using [Swagger]() as a source of the API documentation since charted-server supports the OpenAPI specification for documentation
 *                      of all the endpoints available.
 * @param baseUrl       The base URL of the server for detecting how URLs should be structured. By default, it will be `http(s)://{host}:{port}`
 * @param debug         If debug utilities should be enabled or not. This will allow you to access the `/debug` REST handler, which exposes a lot of metrics.
 *
 */
@Serializable
public data class Config(
    val registrations: Boolean = true,

    @SerialName("jwt_secret_key")
    val jwtSecretKey: String = DO_NOT_USE_THIS_VALUE_IN_YOUR_JWT_SECRET_KEY_OR_I_WILL_LAUGH_AT_YOU,

    @SerialName("invite_only")
    val inviteOnly: Boolean = false,
    val telemetry: Boolean = false,

    @SerialName("sentry_dsn")
    val sentryDsn: String? = null,

    @SerialName("swagger_ui")
    val swaggerUi: Boolean = true,

    @SerialName("base_url")
    val baseUrl: String? = null,
    val debug: Boolean = false,

    val clickhouse: ClickHouseConfig? = null,
    val analytics: NoelwareAnalyticsConfig? = null,
    val database: DatabaseConfig = DatabaseConfig(),
    val features: List<ServerFeature> = listOf(),
    val sessions: SessionsConfig = SessionsConfig(),
    val storage: StorageConfig = StorageConfig(),
    val metrics: MetricsConfig = MetricsConfig(),
    val tracing: TracingConfig? = null,
    val server: KtorServerConfig = KtorServerConfig(),
    val search: SearchConfig? = null,
    val redis: RedisConfig = RedisConfig(),
    val smtp: SMTPConfig? = null,
    val cdn: CdnProxyConfig? = null
) {
    init {
        if (jwtSecretKey == DO_NOT_USE_THIS_VALUE_IN_YOUR_JWT_SECRET_KEY_OR_I_WILL_LAUGH_AT_YOU) {
            throw IllegalStateException("haha! You used the default secret key! Please change it.")
        }

        if (registrations && inviteOnly) {
            throw ValidationException("body.registrations|invite_only", "registrations and invite_only are mutually exclusive")
        }
    }

    /**
     * Represents a builder class for building a [Config] object.
     */
    @Suppress("MemberVisibilityCanBePrivate")
    public open class Builder : org.noelware.charted.common.Builder<Config> {
        public var registrations: Boolean = true
        public var jwtSecretKey: String = DO_NOT_USE_THIS_VALUE_IN_YOUR_JWT_SECRET_KEY_OR_I_WILL_LAUGH_AT_YOU
        public var inviteOnly: Boolean = false
        public var swaggerUi: Boolean = true
        public var telemetry: Boolean = false
        public var sentryDsn: String? = null
        public var baseUrl: String? = null
        public var debug: Boolean = false

        private var _clickhouse: ClickHouseConfig? = null
        private var _analytics: NoelwareAnalyticsConfig? = null
        private var _features: MutableList<ServerFeature> = mutableListOf()
        private var _database: DatabaseConfig = DatabaseConfig()
        private var _sessions: SessionsConfig = SessionsConfig()
        private var _storage: StorageConfig = StorageConfig()
        private var _metrics: MetricsConfig = MetricsConfig()
        private var _tracing: TracingConfig? = null
        private var _server: KtorServerConfig = KtorServerConfig()
        private var _search: SearchConfig? = null
        private var _redis: RedisConfig = RedisConfig()
        private var _smtp: SMTPConfig? = null
        private var _cdn: CdnProxyConfig? = null

        public fun clickhouse(builder: ClickHouseConfig.Builder.() -> Unit = {}): Builder {
            _clickhouse = ClickHouseConfig.Builder().apply(builder).build()
            return this
        }

        public fun analytics(builder: NoelwareAnalyticsConfig.Builder.() -> Unit = {}): Builder {
            _analytics = NoelwareAnalyticsConfig.Builder().apply(builder).build()
            return this
        }

        public fun feature(feature: ServerFeature): Builder {
            if (_features.contains(feature)) return this

            _features.add(feature)
            return this
        }

        public fun database(builder: DatabaseConfig.Builder.() -> Unit = {}): Builder {
            _database = DatabaseConfig.Builder().apply(builder).build()
            return this
        }

        public fun storage(builder: StorageConfig.Builder.() -> Unit = {}): Builder {
            _storage = StorageConfig.Builder().apply(builder).build()
            return this
        }

        public fun sessions(builder: SessionsConfig.Builder.() -> Unit = {}): Builder {
            _sessions = SessionsConfig.Builder().apply(builder).build()
            return this
        }

        public fun metrics(builder: MetricsConfig.Builder.() -> Unit = {}): Builder {
            _metrics = MetricsConfig.Builder().apply(builder).build()
            return this
        }

        public fun tracing(builder: TracingConfig.Builder.() -> Unit = {}): Builder {
            _tracing = TracingConfig.Builder().apply(builder).build()
            return this
        }

        public fun server(builder: KtorServerConfig.Builder.() -> Unit = {}): Builder {
            _server = KtorServerConfig.Builder().apply(builder).build()
            return this
        }

        public fun search(builder: SearchConfig.Builder.() -> Unit = {}): Builder {
            _search = SearchConfig.Builder().apply(builder).build()
            return this
        }

        public fun redis(builder: RedisConfig.Builder.() -> Unit = {}): Builder {
            _redis = RedisConfig.Builder().apply(builder).build()
            return this
        }

        public fun smtp(from: String, builder: SMTPConfig.Builder.() -> Unit = {}): Builder {
            _smtp = SMTPConfig.Builder(from).apply(builder).build()
            return this
        }

        public fun cdn(builder: CdnProxyConfig.Builder.() -> Unit = {}): Builder {
            _cdn = CdnProxyConfig.Builder().apply(builder).build()
            return this
        }

        override fun build(): Config = Config(
            registrations,
            jwtSecretKey,
            inviteOnly,
            telemetry,
            sentryDsn,
            swaggerUi,
            baseUrl,
            debug,
            _clickhouse,
            _analytics,
            _database,
            _features.toList(),
            _sessions,
            _storage,
            _metrics,
            _tracing,
            _server,
            _search,
            _redis,
            _smtp,
        )
    }

    public companion object {
        public operator fun invoke(block: Builder.() -> Unit = {}): Config = Builder().apply(block).build()
    }
}
