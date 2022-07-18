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

@kotlinx.serialization.Serializable
data class ElasticsearchConfig(
    @SerialName("client_ssl_path")
    val clientSslPath: String? = null,

    @SerialName("cloud_id")
    val cloudId: String? = null,
    val nodes: List<String> = listOf(),
    val auth: ElasticBasicAuth? = null
)

@kotlinx.serialization.Serializable
data class ElasticBasicAuth(
    val username: String,
    val password: String
)

@kotlinx.serialization.Serializable
data class MeilisearchConfig(
    val endpoint: String = "http://127.0.0.1:7700",
    val masterKey: String? = null
)

@kotlinx.serialization.Serializable
data class SearchConfig(
    val enabled: Boolean = false,
    val elastic: ElasticsearchConfig? = null,
    val meili: MeilisearchConfig? = null
)
