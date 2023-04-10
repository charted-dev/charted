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

package org.noelware.charted.configuration.kotlin.dsl

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.MultiValidationException
import org.noelware.charted.ValidationException
import org.noelware.charted.common.Buildable
import org.noelware.charted.common.serializers.SecretStringSerializer
import org.noelware.charted.configuration.kotlin.dsl.features.DockerRegistryConfig
import org.noelware.charted.configuration.kotlin.dsl.features.ExperimentalFeature
import org.noelware.charted.configuration.kotlin.dsl.features.Feature
import org.noelware.charted.configuration.kotlin.dsl.metrics.MetricsConfig
import org.noelware.charted.configuration.kotlin.dsl.search.SearchConfig
import org.noelware.charted.configuration.kotlin.dsl.server.KtorServerConfig
import org.noelware.charted.configuration.kotlin.dsl.sessions.SessionsConfig
import org.noelware.charted.configuration.kotlin.dsl.storage.StorageConfig
import org.noelware.charted.utils.randomString

/**
 * Transforms the given [path] into a valid URI that represents a location
 * from the API server.
 *
 * @param path The path to locate towards
 */
@Suppress("HttpUrlsUsage") // not necessary
public fun Config.toApiBaseUrl(path: String = "/"): String {
    if (!path.startsWith('/')) return toApiBaseUrl("/$path")
    return when {
        baseUrl != null -> "$baseUrl$path"
        else -> if (server.ssl != null) {
            "https://${server.host}:${server.ssl.port}$path"
        } else {
            "http://${server.host}:${server.port}$path"
        }
    }
}

/**
 * Transforms the given [path] into a valid URI that represents a location
 * towards a CDN-based endpoint.
 *
 * @param path The path to locate towards
 */
public fun Config.toCdnBaseUrl(path: String = "/"): String {
    if (!path.startsWith('/')) return toCdnBaseUrl("/$path")

    return when {
        cdn != null && cdn.enabled -> toApiBaseUrl("/${cdn.prefix}$path")
        storage.hostAlias != null -> "${storage.hostAlias}$path"
        else -> toApiBaseUrl(path)
    }
}

/**
 * Represents the root configuration object that is loaded via a [ConfigurationHost][org.noelware.charted.configuration.ConfigurationHost] that
 * take cares of (de)serializing this object. **charted-server** supports using YAML and the experimental [Kotlin Scripting](#).
 */
@Serializable
public data class Config(
    /** List of enabled experimental features on the server. */
    @SerialName("experimental_features")
    val experimentalFeatures: List<ExperimentalFeature> = listOf(),

    /** Host domain or IP to connect to the [email service gRPC server](https://github.com/charted-dev/email-service). */
    @SerialName("emails_grpc_endpoint")
    val emailsGrpcEndpoint: String? = null,

    /**
     * Whether if registrations is enabled on the server. If this is set to `false`, then all
     * registrations will have to happen via administration approval with:
     *
     *   - `charted accounts create` CLI command
     *   - administration portal if a [web UI](https://charts.noelware.org/docs/web/current) is present
     *
     * > **NOTE**: [registrations] and [inviteOnly] are mutually exclusive from each other. Both cannot be set
     * > to `true`.
     */
    val registrations: Boolean = true,

    /**
     * Configuration for an external Docker Registry server. Requires the [ExperimentalFeature.ExternalOciRegistry] feature
     * to be enabled.
     */
    @SerialName("external_registry")
    val dockerRegistry: DockerRegistryConfig? = null,

    /** Secret key for encoding JWT tokens for session-based authentication */
    @SerialName("jwt_secret_key")
    @Serializable(with = SecretStringSerializer::class)
    val jwtSecretKey: String = randomString(32),

    /**
     * Whether if this instance is on an invite-only basis or not. If this is set to `true`, then
     * you will have to manually create invites to people who want to join the instance via:
     *
     *  - `charted accounts invite` CLI command
     *  - administration portal if a [web UI](https://charts.noelware.org/docs/web/current) is present
     *
     * > **NOTE**: [registrations] and [inviteOnly] are mutually exclusive from each other. Both cannot be set
     * > to `true`.
     */
    @SerialName("invite_only")
    val inviteOnly: Boolean = false,

    /** List of stable features that are enabled on the server. */
    val features: List<Feature> = listOf(),

    /**
     * If Noelware's telemetry services are enabled on this instance. This is completely optional
     * and disabled by default, but if you wish to send events to Noelware to help us improve **charted-server**,
     * then that would be amazing!~ Also, our [telemetry collector](https://telemetry.noelware.org) is 100% open
     * source and all packets are anonymous, we don't collect usernames, ips, container metadata, etc.
     */
    val telemetry: Boolean = false,

    /**
     * Configuration for configuration [Noelware Analytics](https://analytics.noelware.org)'s [gRPC Protocol Service](https://analytics.noelware.org/docs/protocol-server/current)
     * that will export all metrics towards Noelware Analytics.
     */
    val analytics: NoelwareAnalyticsConfig? = null,

    /**
     * If the server should send exception and tracing metadata to a Sentry project via its DSN.
     */
    @SerialName("sentry_dsn")
    @Serializable(with = SecretStringSerializer::class)
    val sentryDsn: String? = null,

    /**
     * Metrics configuration
     */
    val metrics: MetricsConfig = MetricsConfig(),

    /**
     * Search configuration
     */
    val search: SearchConfig? = null,

    /**
     * Sessions configuration
     */
    val sessions: SessionsConfig = SessionsConfig.Local,

    /**
     * Server configuration to configure
     */
    val server: KtorServerConfig = KtorServerConfig(),

    /**
     * Storage configuration to configure
     */
    val storage: StorageConfig = StorageConfig(),

    /**
     * Whether if the server should host [Swagger UI](https://swagger.io/tools/swagger-ui) on the `/_swagger` endpoint or not.
     */
    @SerialName("swagger_ui")
    val swagger: Boolean = true,

    /**
     * Configuration for PostgreSQL, the main database for holding persistent metadata
     * about all API server entities.
     */
    val database: DatabaseConfig = DatabaseConfig(),

    /**
     * URI that is used as the "base URL" for entities that require linking API server entities
     * to a normalized URL.
     */
    @SerialName("base_url")
    val baseUrl: String? = null,

    /**
     * Configuration for Redis for caching entities, rate-limits (if enabled), and
     * sessions.
     */
    val redis: RedisConfig = RedisConfig(),

    /**
     * If debugging mode should be enabled on the server. Albeit, this doesn't do anything
     * most of the cases, but helps debugs issues that might occur.
     */
    val debug: Boolean = false,

    /**
     * Configuration for exporting all local-metadata as a CDN endpoint.
     */
    val cdn: CdnConfig? = null
) {
    init {
        if (registrations && inviteOnly) {
            throw MultiValidationException(
                listOf(
                    ValidationException("body.registrations", "`registrations` and `invite_only` is mutually exclusive"),
                    ValidationException("body.invite_only", "`registrations` and `invite_only` is mutually exclusive"),
                ),
            )
        }
    }

    public companion object {
        public operator fun invoke(builder: Builder.() -> Unit): Config = Builder().apply(builder).build()
    }

    @Suppress("MemberVisibilityCanBePrivate")
    public open class Builder: Buildable<Config> {
        /** List of enabled experimental features on the server. */
        private val experimentalFeatures = mutableListOf<ExperimentalFeature>()

        /** Host domain or IP to connect to the [email service gRPC server](https://github.com/charted-dev/email-service). */
        public var emailsGrpcEndpoint: String? = null

        /**
         * Whether if registrations is enabled on the server. If this is set to `false`, then all
         * registrations will have to happen via administration approval with:
         *
         *   - `charted accounts create` CLI command
         *   - administration portal if a [web UI](https://charts.noelware.org/docs/web/current) is present
         *
         * > **NOTE**: [registrations] and [inviteOnly] are mutually exclusive from each other. Both cannot be set
         * > to `true`.
         */
        public var registrations: Boolean = true

        /**
         * Configuration for an external Docker Registry server. Requires the [ExperimentalFeature.ExternalOciRegistry] feature
         * to be enabled.
         */
        private var dockerRegistry: DockerRegistryConfig? = null

        /** Secret key for encoding JWT tokens for session-based authentication */
        public var jwtSecretKey: String = randomString(32)

        /**
         * Whether if this instance is on an invite-only basis or not. If this is set to `true`, then
         * you will have to manually create invites to people who want to join the instance via:
         *
         *  - `charted accounts invite` CLI command
         *  - administration portal if a [web UI](https://charts.noelware.org/docs/web/current) is present
         *
         * > **NOTE**: [registrations] and [inviteOnly] are mutually exclusive from each other. Both cannot be set
         * > to `true`.
         */
        public var inviteOnly: Boolean = false

        /** List of stable features that are enabled on the server. */
        private val features = mutableListOf<Feature>()

        /**
         * If Noelware's telemetry services are enabled on this instance. This is completely optional
         * and disabled by default, but if you wish to send events to Noelware to help us improve **charted-server**,
         * then that would be amazing!~ Also, our [telemetry collector](https://telemetry.noelware.org) is 100% open
         * source and all packets are anonymous, we don't collect usernames, ips, container metadata, etc.
         */
        public var telemetry: Boolean = false

        /**
         * Configuration for configuration [Noelware Analytics](https://analytics.noelware.org)'s [gRPC Protocol Service](https://analytics.noelware.org/docs/protocol-server/current)
         * that will export all metrics towards Noelware Analytics.
         */
        private var analytics: NoelwareAnalyticsConfig? = null

        /**
         * If the server should send exception and tracing metadata to a Sentry project via its DSN.
         */
        public var sentryDsn: String? = null

        /**
         * Server configuration to configure
         */
        private var server: KtorServerConfig = KtorServerConfig()

        /**
         * Session configuration
         */
        private var sessions: SessionsConfig = SessionsConfig()

        /**
         * Search configuration
         */
        private var search: SearchConfig? = null

        /**
         * Metrics configuration
         */
        private var metrics: MetricsConfig = MetricsConfig()

        /**
         * Storage configuration to configure
         */
        private var storage: StorageConfig = StorageConfig()

        /**
         * Whether if the server should host [Swagger UI](https://swagger.io/tools/swagger-ui) on the `/_swagger` endpoint or not.
         */
        public var swagger: Boolean = true

        /**
         * Configuration for PostgreSQL, the main database for holding persistent metadata
         * about all API server entities.
         */
        private var database: DatabaseConfig = DatabaseConfig()

        /**
         * URI that is used as the "base URL" for entities that require linking API server entities
         * to a normalized URL.
         */
        public var baseUrl: String? = null

        /**
         * Configuration for Redis for caching entities, rate-limits (if enabled), and
         * sessions.
         */
        private var redis: RedisConfig = RedisConfig()

        /**
         * If debugging mode should be enabled on the server. Albeit, this doesn't do anything
         * most of the cases, but helps debugs issues that might occur.
         */
        public var debug: Boolean = false

        /**
         * Configuration for exporting all local-metadata as a CDN endpoint.
         */
        private var cdn: CdnConfig? = null

        public fun experimentalFeature(vararg features: ExperimentalFeature): Builder {
            for (feature in features) {
                if (experimentalFeatures.contains(feature)) continue
                experimentalFeatures.add(feature)
            }

            return this
        }

        public fun features(vararg allFeatures: Feature): Builder {
            for (feature in allFeatures) {
                if (features.contains(feature)) continue
                features.add(feature)
            }

            return this
        }

        public fun dockerRegistry(builder: DockerRegistryConfig.Builder.() -> Unit = {}): Builder {
            // Enable it if it is not present
            experimentalFeature(ExperimentalFeature.ExternalOciRegistry)

            dockerRegistry = DockerRegistryConfig.Builder().apply(builder).build()
            return this
        }

        public fun analytics(builder: NoelwareAnalyticsConfig.Builder.() -> Unit = {}): Builder {
            analytics = NoelwareAnalyticsConfig.Builder().apply(builder).build()
            return this
        }

        public fun metrics(builder: MetricsConfig.Builder.() -> Unit = {}): Builder {
            metrics = MetricsConfig.Builder().apply(builder).build()
            return this
        }

        public fun storage(builder: StorageConfig.Builder.() -> Unit = {}): Builder {
            storage = StorageConfig.Builder().apply(builder).build()
            return this
        }

        public fun database(builder: DatabaseConfig.Builder.() -> Unit = {}): Builder {
            database = DatabaseConfig.Builder().apply(builder).build()
            return this
        }

        public fun server(builder: KtorServerConfig.Builder.() -> Unit = {}): Builder {
            server = KtorServerConfig.Builder().apply(builder).build()
            return this
        }

        public fun sessions(builder: SessionsConfig.Builder.() -> Unit = {}): Builder {
            sessions = SessionsConfig.Builder().apply(builder).build()
            return this
        }

        public fun search(builder: SearchConfig.Builder.() -> Unit = {}): Builder {
            search = SearchConfig.Builder().apply(builder).build()
            return this
        }

        public fun redis(builder: RedisConfig.Builder.() -> Unit = {}): Builder {
            redis = RedisConfig.Builder().apply(builder).build()
            return this
        }

        public fun cdn(builder: CdnConfig.Builder.() -> Unit = {}): Builder {
            cdn = CdnConfig.Builder().apply(builder).build()
            return this
        }

        override fun build(): Config = Config(
            experimentalFeatures,
            emailsGrpcEndpoint,
            registrations,
            dockerRegistry,
            jwtSecretKey,
            inviteOnly,
            features,
            telemetry,
            analytics,
            sentryDsn,
            metrics,
            search,
            sessions,
            server,
            storage,
            swagger,
            database,
            baseUrl,
            redis,
            debug,
            cdn,
        )
    }
}
