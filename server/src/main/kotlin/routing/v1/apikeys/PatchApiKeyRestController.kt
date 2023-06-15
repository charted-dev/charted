/*
 * 🐻‍❄️📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
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

package org.noelware.charted.server.routing.v1.apikeys

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.util.*
import io.swagger.v3.oas.models.PathItem
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.modules.openapi.kotlin.dsl.*
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.modules.postgresql.controllers.apikeys.ApiKeysDatabaseController
import org.noelware.charted.modules.postgresql.controllers.apikeys.PatchApiKeyPayload
import org.noelware.charted.modules.postgresql.controllers.getByIdOrNameOrNull
import org.noelware.charted.modules.postgresql.tables.ApiKeyTable
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.openapi.describeResource

class PatchApiKeyRestController(private val controller: ApiKeysDatabaseController): RestController("/apikeys/{idOrName}", HttpMethod.Patch) {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions)
    }

    override suspend fun call(call: ApplicationCall) {
        val idOrName = call.parameters.getOrFail("idOrName")

        // We query it first to see if it exists or not
        val apikey = controller.getByIdOrNameOrNull(idOrName, ApiKeyTable::name) ?: return call.respond(
            HttpStatusCode.NotFound,
            ApiResponse.err(
                "API_KEY_NOT_FOUND",
                "API key with name or ID '$idOrName' was not found",
            ),
        )

        controller.update(call, apikey.id, call.receive())
        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    companion object: ResourceDescription by describeResource("/apikeys/{idOrName}", {
        patch {
            description = "Patches an API key resource with a given name or snowflake ID"

            idOrName()
            requestBody {
                json {
                    schema<PatchApiKeyPayload>()
                }
            }

            addAuthenticationResponses()
            accepted {
                json {
                    schema(ApiResponse.ok())
                }
            }

            notFound {
                json {
                    schema(
                        ApiResponse.err(
                            "API_KEY_NOT_FOUND",
                            "API key with name 'noel-is-cute' was not found",
                        ),
                    )
                }
            }
        }
    })
}
