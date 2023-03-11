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

import kotlinx.serialization.Serializable
import org.noelware.charted.common.Buildable

/**
 * Represents the configuration for connecting to PostgreSQL.
 *
 * @param username Username for connecting to Postgres if authentication is enabled.
 * @param password Password for connecting to Postgres if authentication is enabled.
 * @param database The database name when storing data (default: "charted")
 * @param schema   If your database is not in the `public` schema, this is where you would set it.
 * @param host     The connection host to connect to Postgres (default: "127.0.0.1")
 * @param port     Connection port to connect to Postgres (default: 5432)
 */
@Serializable
public data class DatabaseConfig(
    val password: String? = null,
    val username: String? = null,
    val database: String = "charted",
    val schema: String? = null,
    val host: String = "127.0.0.1",
    val port: Int = 5432
) {
    @Suppress("MemberVisibilityCanBePrivate")
    public class Builder: Buildable<DatabaseConfig> {
        /** Username for connecting to Postgres if authentication is enabled. */
        public var username: String? = null

        /** Password for connecting to Postgres if authentication is enabled. */
        public var password: String? = null

        /** The database name when storing data (default: "charted") */
        public var database: String = "charted"

        /** If your database is not in the `public` schema, this is where you would set it. */
        public var schema: String? = null

        /** The connection host to connect to Postgres (default: "127.0.0.1") */
        public var host: String = "127.0.0.1"

        /** Connection port to connect to Postgres (default: 5432) */
        public var port: Int = 5432

        override fun build(): DatabaseConfig = DatabaseConfig(
            username,
            password,
            database,
            schema,
            host,
            port,
        )
    }

    public companion object {
        @JvmStatic
        public operator fun invoke(builder: Builder.() -> Unit = {}): DatabaseConfig = Builder().apply(builder).build()
    }
}
