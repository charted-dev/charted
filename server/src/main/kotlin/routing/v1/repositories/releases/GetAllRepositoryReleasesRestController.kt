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

package org.noelware.charted.server.routing.v1.repositories.releases

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.util.*
import org.jetbrains.exposed.dao.id.EntityID
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.features.ExperimentalFeature
import org.noelware.charted.configuration.kotlin.dsl.features.Feature
import org.noelware.charted.models.repositories.RepositoryRelease
import org.noelware.charted.modules.openapi.kotlin.dsl.json
import org.noelware.charted.modules.openapi.kotlin.dsl.ok
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.postgresql.controllers.get
import org.noelware.charted.modules.postgresql.controllers.repositories.RepositoryDatabaseController
import org.noelware.charted.modules.postgresql.controllers.repositories.releases.RepositoryReleaseDatabaseController
import org.noelware.charted.modules.postgresql.tables.RepositoryReleaseTable
import org.noelware.charted.modules.postgresql.tables.RepositoryTable
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.plugins.sessions.PreconditionResult
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.plugins.sessions.preconditions.canAccessRepository
import org.noelware.charted.server.routing.APIVersion
import org.noelware.charted.server.routing.RestController
import kotlinx.datetime.LocalDateTime
import org.noelware.charted.server.routing.openapi.ResourceDescription
import org.noelware.charted.server.routing.openapi.describeResource
import kotlin.reflect.typeOf

class GetAllRepositoryReleasesRestController(
    private val repositories: RepositoryDatabaseController,
    private val releases: RepositoryReleaseDatabaseController,
    private val config: Config
): RestController("/repositories/{id}/releases") {
    override val apiVersion: APIVersion = APIVersion.V1
    override fun Route.init() {
        install(Sessions) {
            allowNonAuthorizedRequests = true

            condition { call -> canAccessRepository(call, false) }
            condition { call ->
                if (config.features.contains(Feature.DockerRegistry) || config.experimentalFeatures.contains(
                        ExperimentalFeature.ExternalOciRegistry,
                    )
                ) {
                    call.respond(HttpStatusCode.NotFound)
                    return@condition PreconditionResult.Failed()
                }

                PreconditionResult.Success
            }
        }
    }

    override suspend fun call(call: ApplicationCall) {
        val repo = repositories.get(call.parameters.getOrFail<Long>("id"))
        val releases = releases.all(RepositoryReleaseTable::repository to EntityID(repo.id, RepositoryTable))

        call.respond(HttpStatusCode.OK, ApiResponse.ok(releases))
    }

    companion object: ResourceDescription by describeResource("/repositories/{id}/releases", {
        get {
            description = "Retrieve all repository releases"

            pathParameter {
                description = "Repository ID to lookup"
                name = "id"

                schema<Long>()
            }

            addAuthenticationResponses()
            ok {
                json {
                    schema(
                        typeOf<ApiResponse.Ok<List<RepositoryRelease>>>(),
                        listOf(
                            RepositoryRelease(
                                "# v0.0.1-beta\n* Added new stuff",
                                LocalDateTime.parse("2023-04-08T02:37:53.741502369"),
                                LocalDateTime.parse("2023-04-08T02:37:53.741502369"),
                                "0.0.1-beta",
                                1234,
                            ),
                        ),
                    )
                }
            }
        }
    })
}