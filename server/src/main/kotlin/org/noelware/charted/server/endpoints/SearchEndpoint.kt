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
import kotlinx.serialization.json.buildJsonArray
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import org.noelware.charted.search.elastic.ElasticsearchBackend
import org.noelware.charted.search.meili.Indexes
import org.noelware.charted.search.meili.MeilisearchBackend
import org.noelware.ktor.body
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Post

@kotlinx.serialization.Serializable
data class SearchBody(
    val query: String,
    val index: String? = null, // this isn't needed in elastic since we do the glob.
    val limit: Int? = null,
    val offset: Int? = null,
    val fields: List<String> = listOf(),
    val strict: Boolean? = null,

    // only in meili, nop in elastic
    val filters: List<String> = listOf(),
    val facets: List<String> = listOf(),
    val attributes: List<String> = listOf(),
    val sort: List<String> = listOf()
)

class SearchEndpoint(
    private val elastic: ElasticsearchBackend? = null,
    private val meili: MeilisearchBackend? = null
): AbstractEndpoint("/search") {
    @Post
    suspend fun search(call: ApplicationCall) {
        val body by call.body<SearchBody>()
        if (elastic != null) {
            val data = elastic.search(
                body.query,
                limit = body.limit ?: 1000,
                offset = body.offset ?: 0,
                fieldsToRequest = body.fields,
                strict = body.strict ?: false
            )

            call.respond(
                HttpStatusCode.OK,
                buildJsonObject {
                    put("success", true)
                    put("data", data)
                }
            )

            return
        }

        if (meili != null) {
            val data = meili.search(
                Indexes.valueOf(body.index!!),
                body.query,
                body.limit ?: 25,
                body.offset ?: 0,
                body.filters,
                body.facets,
                body.attributes,
                body.sort
            )

            call.respond(
                HttpStatusCode.OK,
                buildJsonObject {
                    put("success", true)
                    put("data", data)
                }
            )

            return
        }

        call.respond(
            HttpStatusCode.NotFound,
            buildJsonObject {
                put("success", false)
                put(
                    "errors",
                    buildJsonArray {
                        add(
                            buildJsonObject {
                                put("code", "NO_SEARCH_BACKEND")
                                put("message", "Search endpoint is unavailable due to no search being configured.")
                            }
                        )
                    }
                )
            }
        )
    }
}
