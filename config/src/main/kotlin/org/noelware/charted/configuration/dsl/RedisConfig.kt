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

package org.noelware.charted.configuration.dsl

import kotlinx.serialization.Serializable

@Serializable
data class RedisConfig(
    val sentinels: List<String> = listOf(),
    val master: String? = null,
    val password: String? = null,
    val index: Long = 5,
    val host: String = "localhost",
    val port: Long = 6379,
    val ssl: Boolean = false
) {
    class Builder {
        private var sentinels = mutableListOf<String>()
        var masterName: String? = null
        var password: String? = null
        var index: Long = 5
        var host: String = "localhost"
        var port: Long = 6379
        var ssl: Boolean = false

        fun addSentinel(host: String, port: Int): Builder = addSentinel("$host:$port")
        fun addSentinel(sentinel: String): Builder {
            if (sentinels.contains(sentinel)) return this

            sentinels.add(sentinel)
            return this
        }

        fun build(): RedisConfig = RedisConfig(sentinels, masterName, password, index, host, port, ssl)
    }
}
