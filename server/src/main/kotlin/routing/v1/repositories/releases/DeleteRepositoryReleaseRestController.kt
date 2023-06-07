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

import io.github.z4kn4fein.semver.VersionFormatException
import io.github.z4kn4fein.semver.toVersion
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.util.*
import io.swagger.v3.oas.models.PathItem
import org.jetbrains.exposed.sql.and
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.flags.ApiKeyScope
import org.noelware.charted.modules.helm.charts.HelmChartModule
import org.noelware.charted.modules.openapi.VersionConstraint
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.modules.postgresql.controllers.repositories.RepositoryDatabaseController
import org.noelware.charted.modules.postgresql.controllers.repositories.releases.RepositoryReleaseDatabaseController
import org.noelware.charted.modules.postgresql.tables.RepositoryReleaseTable
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.plugins.sessions.preconditions.canAccessRepository
import org.noelware.charted.server.plugins.sessions.preconditions.canDeleteMetadata
import org.noelware.charted.server.routing.RestController

class DeleteRepositoryReleaseRestController(
    private val repositories: RepositoryDatabaseController,
    private val releases: RepositoryReleaseDatabaseController,
    private val charts: HelmChartModule? = null
): RestController("/repositories/{id}/releases/{version}", HttpMethod.Delete) {
    override fun Route.init() {
        install(Sessions) {
            this += ApiKeyScope.Repositories.Releases.Delete

            condition(::canAccessRepository)
            condition { call -> canDeleteMetadata(call, repositories) }
        }
    }

    override suspend fun call(call: ApplicationCall) {
        val id = call.parameters.getOrFail<Long>("id")
        val version = call.parameters.getOrFail("version")
        val repo = repositories.getEntityOrNull(id)!!

        try {
            version.toVersion(true)
        } catch (e: VersionFormatException) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "INVALID_SEMVER",
                    "Version provided '$version' was not a valid SemVer value",
                ),
            )
        }

        val release = releases.getEntityOrNull {
            (RepositoryReleaseTable.repository eq repo.id) and (RepositoryReleaseTable.tag eq version)
        } ?: return call.respond(HttpStatusCode.NotFound)

        releases.delete(release.id.value)
        charts?.deleteReleaseTarball(repo.owner, repo.id.value, release.tag)

        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    override fun toPathDsl(): PathItem = toPaths("/repositories/{id}/releases/{version}") {
        delete {
            description = "Deletes a repository release, and the tarball if it exists"

            pathParameter {
                description = "Repository ID to lookup"
                name = "id"

                schema<Long>()
            }

            pathParameter {
                description = "Valid SemVer version to lookup the release for"
                name = "version"

                schema<VersionConstraint>()
            }
        }
    }
}
