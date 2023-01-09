/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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

package org.noelware.charted.server.endpoints.v1.api.repositories

import io.ktor.http.*
import org.noelware.charted.server.plugins.PreconditionResult
import org.noelware.charted.server.plugins.SessionsPlugin
import org.noelware.charted.types.responses.ApiError
import org.noelware.ktor.endpoints.AbstractEndpoint

class RepositoriesEndpoints: AbstractEndpoint("/repositories") {
    init {
        install(HttpMethod.Delete, "/repositories/{idOrName}", SessionsPlugin) {
            this += "repo:delete"
            condition { call ->
                val repository = call.getRepositoryEntityByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY, HttpStatusCode.BadRequest)
                call.repoHasPermission(repository, "repo:delete")
            }
        }

        install(HttpMethod.Patch, "/repositories/{idOrName}", SessionsPlugin) {
            this += "repo:update"
            condition { call ->
                val repository = call.getRepositoryEntityByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY, HttpStatusCode.BadRequest)
                call.repoHasPermission(repository, "metadata:update")
            }
        }

        install(HttpMethod.Get, "/repositories/{owner}/{name}", SessionsPlugin) {
            allowNonAuthorizedRequests = true
            this += "repo:view"

            condition { call ->
                val repository = call.getRepositoryEntityByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY, HttpStatusCode.BadRequest)
                call.canAccessRepository(repository)
            }
        }

        install(HttpMethod.Get, "/repositories/{idOrName}", SessionsPlugin) {
            allowNonAuthorizedRequests = true
            this += "repo:view"

            condition { call ->
                val repository = call.getRepositoryEntityByIdOrName() ?: return@condition PreconditionResult.Failed(ApiError.EMPTY, HttpStatusCode.BadRequest)
                call.canAccessRepository(repository)
            }
        }
    }
}
