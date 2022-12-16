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

package org.noelware.charted.configuration.kotlin.dsl.features

import kotlinx.serialization.Serializable

@Serializable
data class DockerRegistryConfig(
    val headers: Map<String, String> = mapOf(),
    val host: String = "127.0.0.1",
    val port: Int = 5000
) {
    class Builder: org.noelware.charted.common.Builder<DockerRegistryConfig> {
        private val headers: MutableMap<String, String> = mutableMapOf()
        var host: String = "127.0.0.1"
        var port: Int = 5000

        fun header(key: String, value: String): Builder {
            headers[key] = value
            return this
        }

        override fun build(): DockerRegistryConfig = DockerRegistryConfig(headers, host, port)
    }
}
