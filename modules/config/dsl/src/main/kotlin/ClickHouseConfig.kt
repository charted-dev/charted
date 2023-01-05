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

package org.noelware.charted.configuration.kotlin.dsl

import kotlinx.serialization.Serializable

/**
 * Represents the configuration for connecting to ClickHouse.
 *
 * @param database            The database name to connect to. (default `charted`)
 * @param username            The username for authentication, default is `null`.
 * @param password            The password for authentication, default is `null`.
 * @param host                The host to connect to, default is `localhost`
 * @param port                The port to connect to, default is 8123.
 */
@Serializable
public data class ClickHouseConfig(
    val database: String = "charted",
    val username: String? = null,
    val password: String? = null,
    val host: String = "127.0.0.1",
    val port: Int = 8123
) {
    @Suppress("MemberVisibilityCanBePrivate")
    public class Builder: org.noelware.charted.common.Builder<ClickHouseConfig> {
        /** Username for connecting to ClickHouse if authentication is enabled. */
        public var username: String? = null

        /** Password for connecting to ClickHouse if authentication is enabled. */
        public var password: String? = null

        /** The database name when storing data (default: "charted") */
        public var database: String = "charted"

        /** The connection host to connect to ClickHouse (default: "127.0.0.1") */
        public var host: String = "127.0.0.1"

        /** Connection port to connect to ClickHouse (default: 8123) */
        public var port: Int = 8123

        override fun build(): ClickHouseConfig = ClickHouseConfig(
            database,
            username,
            password,
            host,
            port,
        )
    }

    public companion object {
        @JvmStatic
        public operator fun invoke(builder: Builder.() -> Unit = {}): ClickHouseConfig = ClickHouseConfig.Builder().apply(builder).build()
    }
}
