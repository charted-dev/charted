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

/**
 * Represents the configuration for connecting to ClickHouse.
 *
 * @param createDbIfNotExists If we should create the [database] if it doesn't exist. (default `false`)
 * @param database The database name to connect to. (default `charted`)
 * @param username The username for authentication, default is `null`.
 * @param password The password for authentication, default is `null`.
 * @param host The host to connect to, default is `localhost`
 * @param port The port to connect to, default is 9000.
 */
@kotlinx.serialization.Serializable
data class ClickHouseConfig(
    @SerialName("create_db_if_not_exists")
    val createDbIfNotExists: Boolean = false,
    val database: String = "charted",
    val username: String? = null,
    val password: String? = null,
    val host: String = "localhost",
    val port: Int = 8123
)
