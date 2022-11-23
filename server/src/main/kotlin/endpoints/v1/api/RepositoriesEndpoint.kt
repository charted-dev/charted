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

@file:Suppress("unused")

package org.noelware.charted.server.endpoints.v1.api

import com.github.benmanes.caffeine.cache.Caffeine
import dev.floofy.utils.exposed.asyncTransaction
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.coroutines.CoroutineName
import kotlinx.coroutines.plus
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.jetbrains.exposed.sql.Op
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.jetbrains.exposed.sql.and
import org.noelware.charted.ChartedInfo
import org.noelware.charted.ChartedScope
import org.noelware.charted.databases.clickhouse.ClickHouseConnection
import org.noelware.charted.databases.postgres.entities.RepositoryEntity
import org.noelware.charted.databases.postgres.entities.UserEntity
import org.noelware.charted.databases.postgres.flags.RepositoryFlags
import org.noelware.charted.databases.postgres.models.Repository
import org.noelware.charted.databases.postgres.models.RepositoryMember
import org.noelware.charted.databases.postgres.models.bitfield
import org.noelware.charted.databases.postgres.tables.RepositoryTable
import org.noelware.charted.databases.postgres.tables.UserTable
import org.noelware.charted.extensions.regexp.toNameRegex
import org.noelware.charted.modules.email.EmailService
import org.noelware.charted.modules.helm.charts.HelmChartModule
import org.noelware.charted.modules.storage.StorageHandler
import org.noelware.charted.server.CoroutinesBasedCaffeineCache
import org.noelware.charted.server.plugins.PreconditionResult
import org.noelware.charted.server.plugins.SessionsPlugin
import org.noelware.charted.server.plugins.currentUser
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.endpoints.*
import kotlin.time.Duration.Companion.hours
import kotlin.time.toJavaDuration

@Serializable
data class MainRepositoryResponse(
    val message: String = "Welcome to the Repositories API!",

    @SerialName("docs_url")
    val docsUrl: String = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/repositories"
)

class RepositoriesEndpoint(
    private val storage: StorageHandler,
    private val charts: HelmChartModule? = null,
    private val emails: EmailService? = null,
    private val clickhouse: ClickHouseConnection? = null
): AbstractEndpoint("/repositories") {
    // Not used at the moment since I don't really know how to design this, yet.
    private val repositoriesCache: CoroutinesBasedCaffeineCache<Long, Repository> = CoroutinesBasedCaffeineCache(
        ChartedScope + CoroutineName("Server-RepositoryCache"),
        Caffeine
            .newBuilder()
            .expireAfterAccess(1.hours.toJavaDuration())
            .buildAsync()
    )

    init {
        // +==============================+
        // Repository Endpoints
        // +==============================+
        install(HttpMethod.Delete, "/repositories/{id}", SessionsPlugin) {
            this += "repo:delete"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

        install(HttpMethod.Patch, "/repositories/{id}", SessionsPlugin) {
            this += "repo:update"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

        install(HttpMethod.Get, "/repositories/{owner}/{name}", SessionsPlugin) {
            allowNonAuthorizedRequests = true
            this += "repo:access"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

        install(HttpMethod.Get, "/repositories/{id}", SessionsPlugin) {
            allowNonAuthorizedRequests = true
            this += "repo:access"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

        // +==============================+
        // Repository Webhooks Endpoints
        // +==============================+
//        install(HttpMethod.Delete, "/repositories/{id}/webhooks/{webhookId}", SessionsPlugin) {
//            this += "repo:webhooks:delete"
//
//            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
//        }
//
//        install(HttpMethod.Patch, "/repositories/{id}/webhooks/{webhookId}", SessionsPlugin) {
//            this += "repo:webhooks:update"
//
//            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
//        }
//
//        install(HttpMethod.Put, "/repositories/{id}/webhooks", SessionsPlugin) {
//            this += "repo:webhooks:create"
//
//            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
//        }
//
//        install(HttpMethod.Get, "/repositories/{id}/webhooks", SessionsPlugin) {
//            this += "repo:webhooks:list"
//
//            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
//        }

        // +==============================+
        // Repository Releases Endpoints
        // +==============================+
        install(HttpMethod.Delete, "/repositories/{id}/releases/{releaseId}", SessionsPlugin) {
            this += "repo:release:delete"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

        install(HttpMethod.Patch, "/repositories/{id}/releases/{releaseId}", SessionsPlugin) {
            this += "repo:release:update"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

        install(HttpMethod.Put, "/repositories/{id}/releases", SessionsPlugin) {
            this += "repo:release:create"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

        // +==============================+
        // Repository Member Endpoints
        // +==============================+
        install(HttpMethod.Delete, "/repositories/{id}/members/invites/{inviteId}", SessionsPlugin) {
            this += "repo:members:invites:delete"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

        install(HttpMethod.Head, "/repositories/{id}/members/invites/{inviteId}", SessionsPlugin) {
            this += "repo:members:invites:access"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

        install(HttpMethod.Put, "/repositories/{id}/members/invites", SessionsPlugin) {
            this += "repo:members:invites:create"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

        install(HttpMethod.Get, "/repositories/{id}/members/invites", SessionsPlugin) {
            this += "repo:members:invites"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

        install(HttpMethod.Delete, "/repositories/{id}/members/{memberId}", SessionsPlugin) {
            this += "repo:member:delete"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

        install(HttpMethod.Patch, "/repositories/{id}/members/{memberId}", SessionsPlugin) {
            this += "repo:member:update"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

        install(HttpMethod.Put, "/repositories/{id}/members", SessionsPlugin) {
            this += "repo:member:create"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

        install(HttpMethod.Get, "/repositories/{id}/members", SessionsPlugin) {
            allowNonAuthorizedRequests = true
            this += "repo:member:view"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }
    }

    @Get
    suspend fun main(call: ApplicationCall) = call.respond(HttpStatusCode.OK, ApiResponse.ok(MainRepositoryResponse()))

    // +==============================+
    // Repository Endpoints
    // +==============================+

    /**
     * Returns a [Repository] entity by the given id.
     * @statusCode 200 If the repository by the given ID was found
     * @statusCode 400 If the ID was not a valid snowflake.
     * @statusCode 404 If the repository with the given ID was not found
     */
    @Get("/{id}")
    suspend fun getById(call: ApplicationCall) {
        val id = call.parameters["id"]?.toLongOrNull()
            ?: return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "INVALID_REPOSITORY_ID",
                    "You will need to specify a Snowflake."
                )
            )

        val repository =
            asyncTransaction(ChartedScope) {
                RepositoryEntity.find { RepositoryTable.id eq id }.firstOrNull()?.let { entity ->
                    Repository.fromEntity(entity)
                }
            } ?: return call.respond(
                HttpStatusCode.NotFound,
                ApiResponse.err("UNKNOWN_REPOSITORY", "Unable to find repository by ID [$id]")
            )

        call.respond(HttpStatusCode.OK, ApiResponse.ok(repository))
    }

    /**
     * Returns a [Repository] entity by the given owner name/id and repository name/id. This can
     * return a 404 status code if:
     *
     * - **owner name/id** was not found in the database
     * - **repository name/id** was not found in the database
     *
     * This method can also return a private repository if the given user (that uses a session token or api key) has permission
     * to view the repository (requires the `repo:access` API key scope!).
     *
     * @statusCode 200 If the repository was found with the given owner and repository name/id
     * @statusCode 400 If the ID was not a valid snowflake.
     * @statusCode 403 If the repository is private and the user doesn't have permission to view.
     * @statusCode 404 If the repository was not found in the database, or the owner had deleted their account or was not found.
     */
    @Get("/{owner}/{name}")
    suspend fun getByOwnerAndRepoName(call: ApplicationCall) {
        val ownerID = call.parameters["owner"]!!
        val repoID = call.parameters["name"]!!
        val owner = asyncTransaction(ChartedScope) {
            UserEntity.find {
                if (ownerID.toNameRegex(false).matches()) {
                    UserTable.username eq ownerID
                } else {
                    UserTable.id eq ownerID.toLong()
                }
            }.firstOrNull()?.id?.value
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            ApiResponse.err(
                "UNKNOWN_REPO_OWNER",
                "Repository owner with ID or name [$ownerID] doesn't exist"
            )
        )

        val REPO_QUERY: Op<Boolean> = if (repoID.toNameRegex(false, 24).matches()) {
            RepositoryTable.name eq repoID
        } else {
            RepositoryTable.id eq repoID.toLong()
        }

        val repo = asyncTransaction(ChartedScope) {
            RepositoryEntity.find {
                REPO_QUERY and (RepositoryTable.owner eq owner)
            }.firstOrNull()?.let { entity -> Repository.fromEntity(entity) }
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            ApiResponse.err("UNKNOWN_REPOSITORY", "Unable to find repository by ID or name [$repoID]")
        )

        call.respond(HttpStatusCode.OK, ApiResponse.ok(repo))
    }

    @Patch("/{id}")
    suspend fun updateRepo(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Delete("/{id}")
    suspend fun deleteRepo(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    // +==============================+
    // Repository Icon Endpoints
    // +==============================+
    @Get("/{id}/icons/current.png")
    suspend fun getCurrentIcon(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Get("/{id}/icons/{icon}")
    suspend fun getRepositoryIcon(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Post("/{id}/icons")
    suspend fun uploadRepositoryIcon(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    // +==============================+
    // Repository Member Endpoints
    // +==============================+

    /**
     * Returns all the repository members that have their visibility set to `PUBLIC` if this is an
     * unauthorized request. If the request is an authorized request and the user has permission to view the
     * repository (if it is private), then it'll show all members.
     *
     * @statusCode 200 List of [RepositoryMember] entities
     * @statusCode 400 If the repository ID is not a valid snowflake
     * @statusCode 403 If the user in the authorized request doesn't have permission to view the private repository
     * @statusCode 500 If any database errors had occurred.
     */
    @Get("/{id}/members")
    suspend fun getRepoMembers(call: ApplicationCall) {
        val id = call.parameters["id"]?.toLongOrNull()
            ?: return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "INVALID_REPOSITORY_ID",
                    "You will need to specify a Snowflake."
                )
            )

        val repoMembers = asyncTransaction(ChartedScope) {
            RepositoryEntity.find { RepositoryTable.id eq id }.firstOrNull()?.let { entity ->
                entity.members.map { e -> RepositoryMember.fromEntity(e) }
            }
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            ApiResponse.err("UNKNOWN_REPOSITORY", "Unable to find repository by ID [$id]")
        )

        call.respond(HttpStatusCode.OK, ApiResponse.ok(repoMembers))
    }

    /**
     * Invites a member to the repository to gain some level of access to it. If the email service
     * isn't available, the user is automatically joined without any invitation link, otherwise, an invitation
     * link will be sent in their email.
     *
     * @statusCode 200 If the invitation email was sent successfully
     * @statusCode 201 The user was automatically invited as a member if [emails] is not available.
     * @statusCode 400 If the repository ID is not a valid snowflake
     * @statusCode 401 If the user doesn't have permission to invite members
     * @statusCode 403 If the user in the authorized request doesn't have permission to view the private repository
     * @statusCode 500 If any database errors had occurred.
     */
    @Put("/{id}/members")
    suspend fun inviteMember(call: ApplicationCall) {
        if (emails == null) {
        }

        call.respond(HttpStatusCode.NotImplemented)
    }

    @Get("/{id}/members/invites")
    suspend fun getAllMemberInvites(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Put("/{id}/members/invites")
    suspend fun createMemberInvite(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Head("/{id}/members/invites/{inviteId}")
    suspend fun checkIfMemberInviteIsAvailable(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Delete("/{id}/members/invites/{inviteId}")
    suspend fun deleteMemberInvite(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Get("/{id}/members/{memberId}")
    suspend fun getRepoMember(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Patch("/{id}/members/{memberId}")
    suspend fun updateMember(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Delete("/{id}/members/{memberId}")
    suspend fun kickMember(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    // +==============================+
    // Repository Releases Endpoints
    // +==============================+

    /**
     * Returns all the repository's releases that had been created that have permission to create
     * releases.
     *
     * @statusCode 200 All the releases that this repository had created.
     * @statusCode 400 If the ID was not a valid snowflake.
     * @statusCode 403 If the repository is private and the user doesn't have permission to view the repository or the releases.
     * @statusCode 404 If the repository was not found in the database.
     */
    @Get("/{id}/releases")
    suspend fun getReleases(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    /**
     * Returns a single repository release by its version.
     *
     * @statusCode 200 The release by the version if it was found
     * @statusCode 400 If the ID was not a valid snowflake.
     * @statusCode 403 If the repository is private and the user doesn't have permission to view the repository or the release.
     * @statusCode 404 If the repository OR release was not found in the database.
     */
    @Get("/{id}/releases/{version}")
    suspend fun getRelease(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    /**
     * Creates a new [RepositoryRelease] entity.
     *
     * @statusCode 201 The repository release entity that was created
     * @statusCode 400 If the ID was not a valid snowflake.
     * @statusCode 401 If the repository release by the version already existed
     * @statusCode 403 If the repository is private and the user doesn't have permission to view the repository
     *                 or create releases.
     * @statusCode 500 If any database errors had occurred while creating the repository release
     */
    @Put("/{id}/releases")
    suspend fun createRelease(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    /**
     * Update the release entity's metadata in the database.
     *
     * @statusCode 202 If the release entity's metadata was updated successfully
     * @statusCode 400 If the ID was not a valid snowflake.
     * @statusCode 401 If the repository release's metadata is readonly
     * @statusCode 403 If the repository is private and the user doesn't have permission to view the repository
     *                 or to update releases.
     * @statusCode 500 If any database errors had occurred while creating the repository release
     */
    @Patch("/{id}/releases/{releaseId}")
    suspend fun updateRelease(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    /**
     * Delete a release entity from the database.
     *
     * @statusCode 202 If the release entity was deleted successfully
     * @statusCode 400 If the ID was not a valid snowflake.
     * @statusCode 403 If the repository is private and the user doesn't have permission to view the repository
     *                 or delete releases.
     * @statusCode 500 If any database errors had occurred while creating the repository release
     */
    @Delete("/{id}/releases/{releaseId}")
    suspend fun deleteRelease(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    // +==============================+
    // Repository Webhooks Endpoints
    // +==============================+

    @Get("/{id}/webhooks")
    suspend fun getRepoWebhooks(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Get("/{id}/webhooks/{webhookId}")
    suspend fun getRepoWebhook(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Put("/{id}/webhooks")
    suspend fun createWebhook(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Patch("/{id}/webhooks/{webhookId}")
    suspend fun patchWebhook(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Delete("/{id}/webhooks/{webhookId}")
    suspend fun deleteWebhook(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Get("/{id}/webhooks/{webhookId}/events/{eventId}")
    suspend fun getRepoWebhookEvent(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    @Delete("/{id}/webhooks/{webhookId}/events/{eventId}")
    suspend fun deleteWebhookEvent(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    // +==============================+
    // Private helper functions
    // +==============================+

    private suspend fun checkRepositoryPermissionOnCurrentUser(call: ApplicationCall): PreconditionResult {
        val id = call.parameters["id"]?.toLongOrNull()
            ?: return run {
                call.respond(
                    HttpStatusCode.BadRequest,
                    ApiResponse.err(
                        "INVALID_REPOSITORY_ID",
                        "Provided repository ID was not a valid snowflake"
                    )
                )

                PreconditionResult.Failed()
            }

        val repository = asyncTransaction(ChartedScope) {
            RepositoryEntity.find { RepositoryTable.id eq id }.firstOrNull()
        } ?: return run {
            call.respond(
                HttpStatusCode.NotFound,
                ApiResponse.err(
                    "UNKNOWN_REPOSITORY",
                    "Repository with ID [$id] was not found."
                )
            )

            PreconditionResult.Failed()
        }

        // Check if the repository is private and the user is a member
        // of that repository.
        val bitfield = RepositoryFlags(repository.flags)
        if (bitfield.has("PRIVATE")) {
            // Check if the user is a member of that organization
            if (!repository.members.any { it.account.id.value == call.currentUser!!.id }) {
                call.respond(
                    HttpStatusCode.Forbidden,
                    ApiResponse.err(
                        "INVALID_REPOSITORY_ACCESS",
                        "You don't have access to view this repository"
                    )
                )

                return PreconditionResult.Failed()
            }
        }

        return PreconditionResult.Success
    }
}
