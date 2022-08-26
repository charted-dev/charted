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

package org.noelware.charted.server.endpoints

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import org.noelware.charted.common.data.Config
import org.noelware.charted.common.data.responses.Response
import org.noelware.charted.elasticsearch.ElasticsearchService
import org.noelware.charted.search.meilisearch.MeilisearchClient
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Post

class SearchEndpoint(
    private val meilisearch: MeilisearchClient? = null,
    private val elasticsearch: ElasticsearchService? = null,
    private val config: Config
): AbstractEndpoint("/search") {
    @Post
    suspend fun search(call: ApplicationCall) {
        val enabled = config.search.elastic != null || config.search.meili != null
        if (!enabled) {
            call.respond(
                HttpStatusCode.NotFound,
                Response.err("SEARCH_NOT_ENABLED", "The search API is not enabled due to no backend being configured.")
            )

            return
        }

        call.respond(HttpStatusCode.NotImplemented)
    }
}
