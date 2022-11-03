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

import kotlinx.serialization.Serializable

/**
 * Represents the configuration object that is used to configure the server to your heart's content~!
 * This can be represented as a Kotlin Script or a YAML file, up to your choice!
 *
 * ```kotlin
 * import org.noelware.charted.configuration.kotlin.dsl.*
 * import org.noelware.charted.configuration.kotlin.host.KotlinScriptHost
 *
 * fun main() {
 *    val config = KotlinScriptHost.load("./path/to/config.charted.kts")
 *    // => org.noelware.charted.configuration.kotlin.dsl.Config?
 * }
 * ```
 */
@Serializable
data class Config(
    val database: DatabaseConfig = DatabaseConfig(),
    val storage: StorageConfig = StorageConfig(),
    val server: KtorServerConfig = KtorServerConfig()
) {
    /**
     * Represents a builder class for building a [Config] object.
     */
    class Builder: org.noelware.charted.common.Builder<Config> {
        private var _database: DatabaseConfig = DatabaseConfig()
        private var _storage: StorageConfig = StorageConfig()
        private var _server: KtorServerConfig = KtorServerConfig()

        fun database(builder: DatabaseConfig.Builder.() -> Unit = {}): Builder {
            _database = DatabaseConfig.Builder().apply(builder).build()
            return this
        }

        fun storage(builder: StorageConfig.Builder.() -> Unit = {}): Builder {
            _storage = StorageConfig.Builder().apply(builder).build()
            return this
        }

        fun server(builder: KtorServerConfig.Builder.() -> Unit = {}): Builder {
            _server = KtorServerConfig.Builder().apply(builder).build()
            return this
        }

        override fun build(): Config = Config(_database)
    }
}
