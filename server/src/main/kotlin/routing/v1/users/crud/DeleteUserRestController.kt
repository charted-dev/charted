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

package org.noelware.charted.server.routing.v1.users.crud

import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.sentry.Sentry
import io.sentry.kotlin.SentryContext
import io.swagger.v3.oas.models.PathItem
import kotlinx.coroutines.launch
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.jetbrains.exposed.sql.deleteWhere
import org.noelware.charted.ChartedScope
import org.noelware.charted.common.types.responses.ApiResponse
import org.noelware.charted.models.flags.ApiKeyScope
import org.noelware.charted.modules.helm.charts.HelmChartModule
import org.noelware.charted.modules.openapi.kotlin.dsl.schema
import org.noelware.charted.modules.openapi.toPaths
import org.noelware.charted.modules.postgresql.asyncTransaction
import org.noelware.charted.modules.postgresql.controllers.users.UserDatabaseController
import org.noelware.charted.modules.postgresql.tables.RepositoryTable
import org.noelware.charted.modules.search.SearchModule
import org.noelware.charted.modules.sessions.AbstractSessionManager
import org.noelware.charted.server.extensions.addAuthenticationResponses
import org.noelware.charted.server.extensions.currentUser
import org.noelware.charted.server.plugins.sessions.Sessions
import org.noelware.charted.server.routing.RestController

class DeleteUserRestController(
    private val charts: HelmChartModule? = null,
    private val search: SearchModule? = null,
    private val sessions: AbstractSessionManager,
    private val controller: UserDatabaseController
): RestController("/users", HttpMethod.Delete) {
    override fun Route.init() {
        install(Sessions) {
            this += ApiKeyScope.User.Delete
            assertSessionOnly = true
        }
    }

    override suspend fun call(call: ApplicationCall) {
        val currentUser = call.currentUser!!

        // As the scope can be cancelled, this should be moved to its own worker pool
        // where it can be cached via Redis and be picked back up once the server
        // starts again, so we don't have any corrupted data.
        ChartedScope.launch(
            if (Sentry.isEnabled()) SentryContext() + ChartedScope.coroutineContext else ChartedScope.coroutineContext,
        ) {
            // Delete the user, which will delete all of their organizations
            // except their repositories since repositories can be tied to both
            // organization and a user. So, we can do that after.
            controller.delete(currentUser.id)

            // Delete all the repositories owned by this user
            asyncTransaction {
                RepositoryTable.deleteWhere { owner eq currentUser.id }
            }
        }

        // As this can take a while and network failures are prone (if not using
        // the filesystem storage driver), deleting all the repository metadata
        // will be pushed to a separate background job
        //
        // ...but for now, we do this the hard way and run this in the
        // same coroutine as this method is being executed from.
        //
        // but in the future and when charted-server supports High Availability,
        // I plan to have this called in a separate worker pool.
        sessions.revokeAll(currentUser.id)
        charts?.destroyIndexYaml(currentUser.id)
        search?.unindexUser(currentUser)

        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    override fun toPathDsl(): PathItem = toPaths("/users") {
        delete {
            description = "Deletes the current authentication user"

            addAuthenticationResponses()
            response(HttpStatusCode.Accepted) {
                contentType(ContentType.Application.Json) {
                    schema(ApiResponse.ok())
                }
            }
        }
    }
}
