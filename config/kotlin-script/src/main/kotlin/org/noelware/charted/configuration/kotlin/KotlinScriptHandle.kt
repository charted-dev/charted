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

@file:Suppress("UNUSED")

package org.noelware.charted.configuration.kotlin

import org.noelware.charted.configuration.dsl.Config
import org.noelware.charted.configuration.dsl.DatabaseConfig
import org.noelware.charted.configuration.dsl.RedisConfig
import org.noelware.charted.configuration.dsl.StorageConfig
import org.noelware.charted.configuration.dsl.features.Feature
import org.noelware.charted.configuration.dsl.features.KtorSSLConfig
import org.noelware.charted.configuration.dsl.features.NoelwareAnalyticsConfig
import org.noelware.charted.configuration.dsl.search.SearchConfig
import org.noelware.charted.configuration.dsl.sessions.SessionConfig
import kotlin.script.experimental.annotations.KotlinScript

/**
 * Represents the handle for running the Kotlin script, giving details on how
 * to configure **charted-server**.
 */
@KotlinScript(fileExtension = "charted.kts")
abstract class KotlinScriptHandle {
    // contains the default config if anything hasn't changed
    internal var config: Config = Config()

    /**
     * If registrations should be enabled on the server. This will disable anyone
     * from posting to the PUT /users endpoint, and you will have to register
     * the user yourself through the admin dashboard.
     */
    var registrations: Boolean = true

    /**
     * The secret key for encoding JWT tokens. The server uses JWT tokens
     * for session tokens, and it can't be used as `__CHANGE ME__`. You can use
     * [#loadSecretKeyFromEnvironment][loadSecretKeyFromEnvironment] or [#loadSecretKeyFromSystemProps][loadSecretKeyFromSystemProps]
     * as an alternative.
     */
    var jwtSecretKey: String = "__CHANGE ME__"

    /**
     * If the server can register users through invites or not. This will disable [registrations] and
     * you will have to manually give the person you want registered a link to join the server.
     */
    var inviteOnly: Boolean = false

    /**
     * If the server can send telemetry events to Noelware. This is default to false
     * and is opt-in.
     */
    var telemetry: Boolean = false

    /**
     * The DSN to use to track exceptions to Sentry. You can load up the Sentry DSN using the
     * [#loadSentryDsnFromEnvironment][loadSentryDsnFromEnvironment] to load it up from the
     * system environment variables.
     */
    var sentryDsn: String? = null

    /**
     * The base URL for urls that point to the server. By default, it will use `localhost:<port>`
     * as the base url.
     */
    var baseUrl: String? = null

    /**
     * A list of features that are toggleable.
     */
    val features: MutableList<Feature> = mutableListOf()

    /**
     * Whether the `GET /metrics` endpoint is enabled or not. This will allow you to use
     * Prometheus to keep track of the server metrics.
     */
    var metrics: Boolean = true

    /**
     * Whether debug mode is enabled or not. If so, more logging will be appended to the
     * appenders configured, SQL traces will be enabled, and more.
     */
    var debug: Boolean = false

    /**
     * If the storage trailer should be exposed with the `/cdn` prefix or not.
     */
    var cdn: Boolean = false

    /**
     * Finds the Sentry DSN from the system environment variables. The [key] will determine
     * where to look, it'll check at `CHARTED_SENTRY_DSN` if the key is null.
     */
    fun loadSentryDsnFromEnvironment(key: String? = null) {
        val dsn = System.getenv(key ?: "CHARTED_SENTRY_DSN")
            ?: throw IllegalStateException("System environment variable [${key ?: "CHARTED_SENTRY_DSN"}] was not found.")

        sentryDsn = dsn
    }

    /**
     * Finds the JWT secret key from the system environment variables. The [key] will determine
     * where to look, it'll check at `CHARTED_JWT_SECRET_KEY` if the key is null.
     */
    fun loadSecretKeyFromEnvironment(key: String? = null) {
        val secret = System.getenv(key ?: "CHARTED_JWT_SECRET_KEY")
            ?: throw IllegalStateException("System environment variable [${key ?: "CHARTED_JWT_SECRET_KEY"}] was not found.")

        jwtSecretKey = secret
    }

    /**
     * Finds the JWT secret key from the system properties. It'll check only in `charted.jwt-secret-key`.
     */
    fun loadSecretKeyFromSystemProps() {
        val prop = System.getProperty("charted.jwt-secret-key", "")
        if (prop.isNotEmpty()) {
            jwtSecretKey = prop
        }
    }

    /**
     * Configures the use of [Noelware Analytics](https://analytics.noelware.org) to visualize what charted-server
     * is doing.
     */
    fun analytics(block: NoelwareAnalyticsConfig.Builder.() -> Unit = {}) {
        // TODO: find better way than `copy`
        config = config.copy(analytics = NoelwareAnalyticsConfig.Builder().apply(block).build())
    }

    /**
     * Configures the Postgres connection details.
     */
    fun database(block: DatabaseConfig.Builder.() -> Unit = {}) {
        // TODO: find better way than `copy`
        config = config.copy(postgres = DatabaseConfig.Builder().apply(block).build())
    }

    /**
     * Configures the session configuration, which can configure integrations like
     * GitHub or Noelware Accounts, local LDAP server, and more.
     */
    fun sessions(block: SessionConfig.Builder.() -> Unit = {}) {
        config = config.copy(sessions = SessionConfig.Builder().apply(block).build())
    }

    fun storage(block: StorageConfig.Builder.() -> Unit = {}) {
        config = config.copy(storage = StorageConfig.Builder().apply(block).build())
    }

    fun search(block: SearchConfig.Builder.() -> Unit = {}) {
        config = config.copy(search = SearchConfig.Builder().apply(block).build())
    }

    fun redis(block: RedisConfig.Builder.() -> Unit = {}) {
        config = config.copy(redis = RedisConfig.Builder().apply(block).build())
    }

    fun ssl(keystore: String, password: String? = null) {
        config = config.copy(ssl = KtorSSLConfig(keystore, password ?: ""))
    }
}
