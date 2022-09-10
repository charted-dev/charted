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

@kotlinx.serialization.Serializable
data class CassandraConfig(
    val username: String? = null,
    val password: String? = null,
    val keyspace: String = "charted",
    val nodes: List<String> = listOf()
) {
    class Builder {
        private val nodes = mutableListOf<String>()
        var username: String? = null
        var password: String? = null
        var keyspace: String = "charted"

        fun addNode(host: String, port: Int): Builder = addNode("$host:$port")
        fun addNode(node: String): Builder {
            if (nodes.contains(node)) return this

            nodes.add(node)
            return this
        }

        fun build(): CassandraConfig = CassandraConfig(username, password, keyspace, nodes)
    }
}
