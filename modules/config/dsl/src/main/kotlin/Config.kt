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

package org.noelware.charted.configuration.kotlin.dsl

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.ValidationException
import org.noelware.charted.configuration.kotlin.dsl.features.DockerRegistryConfig
import org.noelware.charted.configuration.kotlin.dsl.metrics.MetricsConfig
import org.noelware.charted.configuration.kotlin.dsl.search.SearchConfig
import org.noelware.charted.configuration.kotlin.dsl.sessions.SessionsConfig
import org.noelware.charted.configuration.kotlin.dsl.tracing.TracingConfig

const val DO_NOT_USE_THIS_VALUE_IN_YOUR_JWT_SECRET_KEY_OR_I_WILL_LAUGH_AT_YOU = "__DO NOT USE THIS AS THE SECRET KEY__"

/**
 * Represents the configuration object that is used to configure the server to your heart's content~!
 * This can be represented as a Kotlin Script or a YAML file, up to your choice!
 *
 * ```kotlin
 * import org.noelware.charted.configuration.kotlin.dsl.*
 * import org.noelware.charted.configuration.kotlin.host.KotlinScriptHost
 * import org.noelware.charted.configuration.yaml.host.ConfigYamlHost
 *
 * fun main() {
 *    val config = KotlinScriptHost.load("./path/to/config.charted.kts")
 *    // => org.noelware.charted.configuration.kotlin.dsl.Config?
 *
 *    val config2 = ConfigYamlHost.load("./path/to/config.yaml")
 *    // => org.noelware.charted.configuration.kotlin.dsl.Config?
 * }
 * ```
 *
 * @param registrations If registrations are enabled on the server. If not, administrators of this
 *                      instance will have to create a user via the `PUT /admin/users/create` endpoint
 *                      or through the built-in admin UI portal the web UI bundles in.
 * @param jwtSecretKey  The secret key for encoding JWTs for all sessions. Please do not use
 *                      the `DO_NOT_USE_THIS_VALUE_IN_YOUR_JWT_SECRET_KEY_OR_I_WILL_LAUGH_AT_YOU` variable,
 *                      or you will be laughed at.
 * @param inviteOnly    If the server is only invited by an invitation system that the admins of the instance
 *                      can invite you, and you can create your user and begin using charted-server~!
 * @param telemetry     If [Noelware Telemetry](https://telemetry.noelware.org) should be enabled on this instance.
 *                      This is completely optional and opt-in if you wish to send out service metadata and error reports
 *                      to the Noelware Team, you can read in the documentation what we collect, and you can visit the
 *                      [open sourced project](https://github.com/Noelware/telemetry) if you're still thinking about not
 *                      using our products. :(
 * @param sentryDsn     If Sentry should be enabled on the server or not. This will send out error reports to the Sentry server
 *                      you decide to use.
 * @param baseUrl       The base URL for formatting purposes in the chart's download URL. It will default to `http(s)://<server.host>:<server.port>`
 *                      by default.
 * @param debug         If debug mode is enabled on the server. You will be granted access to the `/debug` REST handler for seeing
 *                      what the server is doing and whatnot.
 * @param clickhouse    **charted-server** uses ClickHouse to store analytical features (i.e, audit logs and webhook events) to be as fast for reading
 *                      and processing many documents at once.
 * @param database      **charted-server** uses PostgreSQL to store metadata about each user, repository, and organization (or other features if needed), so this
 *                      is highly recommended to be configured.
 * @param server        Server configuration to update default settings that the server will choose or adds additional/security headers, and more~!
 */
@Serializable
data class Config(
    val registrations: Boolean = true,

    @SerialName("jwt_secret_key")
    val jwtSecretKey: String = DO_NOT_USE_THIS_VALUE_IN_YOUR_JWT_SECRET_KEY_OR_I_WILL_LAUGH_AT_YOU,

    @SerialName("invite_only")
    val inviteOnly: Boolean = false,
    val telemetry: Boolean = false,

    @SerialName("sentry_dsn")
    val sentryDsn: String? = null,

    @SerialName("base_url")
    val baseUrl: String? = null,
    val debug: Boolean = false,

    @SerialName("docker_registry")
    val dockerRegistry: DockerRegistryConfig? = null,
    val clickhouse: ClickHouseConfig? = null,
    val database: DatabaseConfig = DatabaseConfig(),
    val features: List<ServerFeature> = listOf(),
    val sessions: SessionsConfig = SessionsConfig(),
    val storage: StorageConfig = StorageConfig(),
    val metrics: MetricsConfig = MetricsConfig(),
    val tracing: TracingConfig? = null,
    val server: KtorServerConfig = KtorServerConfig(),
    val search: SearchConfig? = null,
    val redis: RedisConfig = RedisConfig(),
    val smtp: SMTPConfig? = null
) {
    init {
        if (jwtSecretKey == DO_NOT_USE_THIS_VALUE_IN_YOUR_JWT_SECRET_KEY_OR_I_WILL_LAUGH_AT_YOU) {
            throw IllegalStateException("haha! You used the default secret key! Please change it.")
        }

        if (registrations && inviteOnly) {
            throw ValidationException("body.registrations|invite_only", "registrations and invite_only are mutually exclusive")
        }

        if (features.contains(ServerFeature.DOCKER_REGISTRY) && dockerRegistry == null) {
            throw ValidationException("body.docker_registry", "The docker_registry feature must have a docker registry configuration object connecting to a valid OCI registry")
        }
    }

    /**
     * Represents a builder class for building a [Config] object.
     */
    open class Builder: org.noelware.charted.common.Builder<Config> {
        var registrations: Boolean = true
        var jwtSecretKey: String = DO_NOT_USE_THIS_VALUE_IN_YOUR_JWT_SECRET_KEY_OR_I_WILL_LAUGH_AT_YOU
        var inviteOnly: Boolean = false
        var telemetry: Boolean = false
        var sentryDsn: String? = null
        var baseUrl: String? = null
        var debug: Boolean = false

        private var _dockerRegistry: DockerRegistryConfig? = null
        private var _clickhouse: ClickHouseConfig? = null
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

        fun dockerRegistry(builder: DockerRegistryConfig.Builder.() -> Unit): Builder {
            _dockerRegistry = DockerRegistryConfig.Builder().apply(builder).build()
            return this
        }

        fun clickhouse(builder: ClickHouseConfig.Builder.() -> Unit = {}): Builder {
            _clickhouse = ClickHouseConfig.Builder().apply(builder).build()
            return this
        }

        fun feature(feature: ServerFeature): Builder {
            if (_features.contains(feature)) return this

            _features.add(feature)
            return this
        }

        fun database(builder: DatabaseConfig.Builder.() -> Unit = {}): Builder {
            _database = DatabaseConfig.Builder().apply(builder).build()
            return this
        }

        fun storage(builder: StorageConfig.Builder.() -> Unit = {}): Builder {
            _storage = StorageConfig.Builder().apply(builder).build()
            return this
        }

        fun sessions(builder: SessionsConfig.Builder.() -> Unit = {}): Builder {
            _sessions = SessionsConfig.Builder().apply(builder).build()
            return this
        }

        fun metrics(builder: MetricsConfig.Builder.() -> Unit = {}): Builder {
            _metrics = MetricsConfig.Builder().apply(builder).build()
            return this
        }

        fun tracing(builder: TracingConfig.Builder.() -> Unit = {}): Builder {
            _tracing = TracingConfig.Builder().apply(builder).build()
            return this
        }

        fun server(builder: KtorServerConfig.Builder.() -> Unit = {}): Builder {
            _server = KtorServerConfig.Builder().apply(builder).build()
            return this
        }

        fun search(builder: SearchConfig.Builder.() -> Unit = {}): Builder {
            _search = SearchConfig.Builder().apply(builder).build()
            return this
        }

        fun redis(builder: RedisConfig.Builder.() -> Unit = {}): Builder {
            _redis = RedisConfig.Builder().apply(builder).build()
            return this
        }

        fun smtp(from: String, builder: SMTPConfig.Builder.() -> Unit = {}): Builder {
            _smtp = SMTPConfig.Builder(from).apply(builder).build()
            return this
        }

        override fun build(): Config = Config(
            registrations,
            jwtSecretKey,
            inviteOnly,
            telemetry,
            sentryDsn,
            baseUrl,
            debug,
            _dockerRegistry,
            _clickhouse,
            _database,
            _features.toList(),
            _sessions,
            _storage,
            _metrics,
            _tracing,
            _server,
            _search,
            _redis,
            _smtp
        )
    }
}
