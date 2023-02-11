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

package org.noelware.charted.configuration.kotlin.dsl.search

import kotlinx.serialization.Serializable
import org.noelware.charted.ValidationException
import org.noelware.charted.configuration.kotlin.dsl.search.elasticsearch.ElasticsearchConfig

@Serializable
public data class SearchConfig(
    val elasticsearch: ElasticsearchConfig? = null,
    val meilisearch: MeilisearchConfig? = null
) {
    init {
        if (elasticsearch != null && meilisearch != null) {
            throw ValidationException("config.search", "`elasticsearch` and `meilisearch` are mutually exclusive")
        }
    }

    public class Builder : org.noelware.charted.common.Builder<SearchConfig> {
        private var _elasticsearch: ElasticsearchConfig? = null
        private var _meilisearch: MeilisearchConfig? = null

        public fun elasticsearch(builder: ElasticsearchConfig.Builder.() -> Unit = {}): Builder {
            _elasticsearch = ElasticsearchConfig.Builder().apply(builder).build()
            return this
        }

        public fun meilisearch(builder: MeilisearchConfig.Builder.() -> Unit = {}): Builder {
            _meilisearch = MeilisearchConfig.Builder().apply(builder).build()
            return this
        }

        override fun build(): SearchConfig = SearchConfig(_elasticsearch)
    }
}
