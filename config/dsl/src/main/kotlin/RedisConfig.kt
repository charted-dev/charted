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

@file:Suppress("MemberVisibilityCanBePrivate")

package org.noelware.charted.configuration.kotlin.dsl

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.common.Buildable
import org.noelware.charted.common.serializers.SecretStringSerializer

/**
 * Represents the configuration for Redis, which charted-server uses for caching
 * objects.
 */
@Serializable
public data class RedisConfig(
    /** Master name for the list of [sentinels]. */
    @SerialName("master")
    val masterName: String? = null,

    /** List of sentinel endpoints to connect to */
    val sentinels: List<String> = listOf(),

    /** Password for server authentication */
    @Serializable(with = SecretStringSerializer::class)
    val password: String? = null,

    /** Database number to use to not clash with other data in the Redis server */
    val index: Int = 5,

    /** Host domain or IP to connect to, this is only for standalone connections. */
    val host: String = "localhost",

    /** Port to connect to, this is only for standalone connections */
    val port: Short = 6379,

    /** If the client connections should use SSL or not */
    val ssl: Boolean = false
) {
    public class Builder: Buildable<RedisConfig> {
        private var sentinels = mutableListOf<String>()

        /** Master name for the list of sentinels. */
        public var masterName: String? = null

        /** Password for server authentication */
        public var password: String? = null

        /** Database number to use to not clash with other data in the Redis server */
        public var index: Int = 5

        /** Host domain or IP to connect to, this is only for standalone connections. */
        public var host: String = "localhost"

        /** Port to connect to, this is only for standalone connections */
        public var port: Short = 6379

        /** If the client connections should use SSL or not */
        public var ssl: Boolean = false

        /**
         * Adds a sentinel address to the list of sentinels. This will automatically
         * use a Sentinel connection over a standalone connection.
         *
         * @param host The host domain or IP to use when connecting to the sentinel
         * @param port Port to bind to
         * @return chained instance of [Builder]
         */
        public fun addSentinelAddress(host: String, port: Short): Builder = addSentinelAddress("$host:$port")

        /**
         * Adds a sentinel address by target (host:port) to the list of sentinels. This will automatically
         * use a Sentinel connection over a standalone connection.
         *
         * @param target Target host:port binding to connect towards
         * @return chained instance of [Builder]
         */
        public fun addSentinelAddress(target: String): Builder {
            if (sentinels.contains(target)) return this

            sentinels.add(target)
            return this
        }

        override fun build(): RedisConfig = RedisConfig(masterName, sentinels, password, index, host, port, ssl)
    }

    public companion object {
        @JvmStatic
        public operator fun invoke(builder: Builder.() -> Unit = {}): RedisConfig = Builder().apply(builder).build()
    }
}
