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

package org.noelware.charted.search.elasticsearch.index

import kotlinx.serialization.json.*

data class Index(val name: String, val settings: JsonObject) {
    companion object {
        val USERS = Index(
            "charted-users",
            buildJsonObject {
                putJsonObject("settings") {
                    putJsonObject("index") {
                        put("number_of_shards", 5)
                        put("number_of_replicas", 3)
                        put("gc_deletes", "30s")
                    }
                }

                putJsonObject("mappings") {
                    putJsonObject("properties") {
                        putJsonObject("description") {
                            put("type", "text")
                        }

                        putJsonObject("username") {
                            put("type", "text")
                        }

                        putJsonObject("name") {
                            put("type", "text")
                        }
                    }
                }
            }
        )

        val REPOSITORIES = Index(
            "charted-repositories",
            buildJsonObject {
                putJsonObject("settings") {
                    putJsonObject("index") {
                        put("number_of_shards", 5)
                        put("number_of_replicas", 3)
                        put("gc_deletes", "30s")
                    }
                }
            }
        )

        val ORGANIZATIONS = Index(
            "charted-organizations",
            buildJsonObject {
                putJsonObject("settings") {
                    putJsonObject("index") {
                        put("number_of_shards", 5)
                        put("number_of_replicas", 3)
                        put("gc_deletes", "30s")
                    }
                }
            }
        )

        val AUDIT_LOGS = Index(
            "charted-audit-logs",
            buildJsonObject {
                putJsonObject("settings") {
                    putJsonObject("index") {
                        put("number_of_shards", 5)
                        put("number_of_replicas", 3)
                        put("gc_deletes", "30s")
                    }
                }
            }
        )

        val WEBHOOK_EVENTS = Index(
            "charted-webhook-events",
            buildJsonObject {
                putJsonObject("settings") {
                    putJsonObject("index") {
                        put("number_of_shards", 5)
                        put("number_of_replicas", 3)
                        put("gc_deletes", "30s")
                    }
                }
            }
        )

        val indexes = listOf(
            USERS,
            REPOSITORIES,
            ORGANIZATIONS
        )
    }
}
