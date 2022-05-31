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
 *  Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package org.noelware.charted.search.elastic

import kotlinx.serialization.json.*

/**
 * Represents the settings for an index.
 */
val INDEX_SETTINGS = buildJsonObject {
    put("number_of_replicas", 2)
    put("number_of_shards", 5)

    putJsonObject("analysis") {
        putJsonObject("analyzer") {
            putJsonObject("ngram") {
                put("tokenizer", "ngram_tokenizer")
            }
        }
    }

    putJsonObject("tokenizer") {
        putJsonObject("ngram_tokenizer") {
            put("type", "ngram")
            put("min_gram", 3)
            put("max_gram", 3)
            putJsonArray("token_chars") {
                add("letter")
                add("digit")
            }
        }
    }
}

/**
 * Represents the index settings and mappings for a specific index.
 */
object ElasticsearchIndexSettings {
    val USERS = buildJsonObject {
        putJsonObject("mappings") {
            putJsonObject("properties") {
            }
        }
    }

    val REPOSITORIES = buildJsonObject {
        putJsonObject("mappings") {
            putJsonObject("properties") {
            }
        }
    }

    val ORGANIZATIONS = buildJsonObject {
        putJsonObject("mappings") {
            putJsonObject("properties") {
            }
        }
    }

    val RELEASES = buildJsonObject {
        putJsonObject("mappings") {
            putJsonObject("properties") {
            }
        }
    }

    val AUDIT_LOGS = buildJsonObject {
        putJsonObject("mappings") {
            putJsonObject("properties") {
            }
        }
    }

    val WEBHOOKS = buildJsonObject {
        putJsonObject("mappings") {
            putJsonObject("properties") {
            }
        }
    }
}
