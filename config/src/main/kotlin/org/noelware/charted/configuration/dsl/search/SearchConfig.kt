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

package org.noelware.charted.configuration.dsl.search

@kotlinx.serialization.Serializable
data class SearchConfig(
    val elastic: ElasticsearchConfig? = null,
    val meili: MeilisearchConfig? = null
) {
    class Builder {
        private var elasticsearch: ElasticsearchConfig? = null
        private var meilisearch: MeilisearchConfig? = null

        fun elasticsearch(block: ElasticsearchConfig.Builder.() -> Unit = {}): Builder {
            if (meilisearch != null) throw IllegalStateException("Configuration key 'meilisearch' can't be used with 'elasticsearch'")
            if (elasticsearch != null) return this

            elasticsearch = ElasticsearchConfig.Builder().apply(block).build()
            return this
        }

        fun meilisearch(block: MeilisearchConfig.Builder.() -> Unit = {}): Builder {
            if (elasticsearch != null) throw IllegalStateException("Configuration key 'elasticsearch' can't be used with 'meilisearch'")
            if (meilisearch != null) return this

            meilisearch = MeilisearchConfig.Builder().apply(block).build()
            return this
        }

        fun build(): SearchConfig = SearchConfig(elasticsearch, meilisearch)
    }
}
