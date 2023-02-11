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

package org.noelware.charted.configuration.kotlin.dsl.search.elasticsearch

import kotlinx.serialization.Serializable

@Serializable
public data class ElasticsearchConfig(
    val nodes: List<String> = listOf("127.0.0.1:9200"),
    val auth: AuthenticationStrategy = AuthenticationStrategy.None,
    val ssl: ElasticsearchSSLConfig? = null
) {
    @Suppress("MemberVisibilityCanBePrivate")
    public class Builder : org.noelware.charted.common.Builder<ElasticsearchConfig> {
        private val nodes = mutableListOf<String>()
        private var _auth: AuthenticationStrategy = AuthenticationStrategy.None
        private var ssl: ElasticsearchSSLConfig? = null

        public fun ssl(builder: ElasticsearchSSLConfig.Builder.() -> Unit = {}): Builder {
            ssl = ElasticsearchSSLConfig.Builder().apply(builder).build()
            return this
        }

        public fun auth(strategy: AuthenticationStrategy): Builder {
            _auth = strategy
            return this
        }

        public fun node(addr: String): Builder {
            nodes.add(addr)
            return this
        }

        public fun node(host: String, port: Int): Builder = node("http${if (ssl != null) "s" else ""}://$host:$port")

        override fun build(): ElasticsearchConfig = ElasticsearchConfig(nodes, _auth, ssl)
    }

    public companion object {
        @JvmStatic
        public operator fun invoke(builder: Builder.() -> Unit = {}): ElasticsearchConfig = Builder().apply(builder).build()
    }
}
