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

package org.noelware.charted.configuration.kotlin.dsl.search.elasticsearch

import kotlinx.serialization.Serializable

@Serializable
data class ElasticsearchConfig(
    val nodes: List<String> = listOf("127.0.0.1:9200"),
    val auth: AuthenticationStrategy = AuthenticationStrategy.None,
    val ssl: ElasticsearchSSLConfig? = null
) {
    class Builder: org.noelware.charted.common.Builder<ElasticsearchConfig> {
        private val nodes = mutableListOf<String>()
        private var _auth: AuthenticationStrategy = AuthenticationStrategy.None
        private var ssl: ElasticsearchSSLConfig? = null

        fun ssl(builder: ElasticsearchSSLConfig.Builder.() -> Unit = {}): Builder {
            ssl = ElasticsearchSSLConfig.Builder().apply(builder).build()
            return this
        }

        fun auth(strategy: AuthenticationStrategy): Builder {
            _auth = strategy
            return this
        }

        fun node(addr: String): Builder {
            nodes.add(addr)
            return this
        }

        fun node(host: String, port: Int): Builder {
            nodes.add("$host:$port")
            return this
        }

        override fun build(): ElasticsearchConfig = ElasticsearchConfig(nodes, _auth, ssl)
    }
}
