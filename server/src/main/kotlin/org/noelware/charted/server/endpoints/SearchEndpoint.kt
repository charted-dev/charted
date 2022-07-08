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

package org.noelware.charted.server.endpoints

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.serialization.json.*
import org.noelware.charted.common.data.Config
import org.noelware.charted.search.elasticsearch.ElasticsearchClient
import org.noelware.charted.search.meilisearch.MeilisearchClient
import org.noelware.ktor.body
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Post

@kotlinx.serialization.Serializable
data class SearchBody(
    val query: String,
    val limit: Int = 25,
    val offset: Int = 0
)

class SearchEndpoint(
    private val meilisearch: MeilisearchClient? = null,
    private val elasticsearch: ElasticsearchClient? = null,
    private val config: Config
): AbstractEndpoint("/search") {
    @Post
    suspend fun search(call: ApplicationCall) {
        if (!config.search.enabled) {
            call.respond(
                HttpStatusCode.NotFound,
                buildJsonObject {
                    put("success", false)
                    putJsonArray("errors") {
                        addJsonObject {
                            put("code", "SEARCH_NOT_ENABLED")
                            put("message", "The search API is not enabled due to no backend being configured.")
                        }
                    }
                }
            )

            return
        }

        val body by call.body<SearchBody>()
        if (elasticsearch != null) {
            val result = elasticsearch.search(body.query, body.limit, body.offset)
            call.respond(
                HttpStatusCode.OK,
                buildJsonObject {
                    put("success", true)
                    put("data", result)
                }
            )

            return
        }
    }
}
