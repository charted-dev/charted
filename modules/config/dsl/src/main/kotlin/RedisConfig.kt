/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

@Serializable
public data class RedisConfig(
    val sentinels: List<String> = listOf(),
    val master: String? = null,
    val password: String? = null,
    val index: Int = 5,
    val host: String = "localhost",
    val port: Int = 6379,
    val ssl: Boolean = false
) {
    @Suppress("MemberVisibilityCanBePrivate")
    public class Builder : org.noelware.charted.common.Builder<RedisConfig> {
        private var sentinels = mutableListOf<String>()
        public var masterName: String? = null
        public var password: String? = null
        public var index: Int = 5
        public var host: String = "localhost"
        public var port: Int = 6379
        public var ssl: Boolean = false

        public fun addSentinel(host: String, port: Int): Builder = addSentinel("$host:$port")
        public fun addSentinel(sentinel: String): Builder {
            if (sentinels.contains(sentinel)) return this

            sentinels.add(sentinel)
            return this
        }

        override fun build(): RedisConfig = RedisConfig(sentinels, masterName, password, index, host, port, ssl)
    }

    public companion object {
        public operator fun invoke(config: RedisConfig): RedisConfig = config

        @JvmStatic
        public operator fun invoke(builder: Builder.() -> Unit = {}): RedisConfig = Builder().apply(builder).build()
    }
}
