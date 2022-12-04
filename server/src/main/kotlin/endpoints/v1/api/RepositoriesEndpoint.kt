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
import io.github.z4kn4fein.semver.VersionFormatException
import io.github.z4kn4fein.semver.toVersion
import io.ktor.http.*
import io.ktor.http.content.*
import io.ktor.server.application.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.util.*
import kotlinx.coroutines.CoroutineName
import kotlinx.coroutines.plus
import kotlinx.datetime.Clock
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.jetbrains.exposed.sql.*
import org.jetbrains.exposed.sql.SqlExpressionBuilder.eq
import org.noelware.charted.*
import org.noelware.charted.configuration.kotlin.dsl.Config
import org.noelware.charted.configuration.kotlin.dsl.ServerFeature
import org.noelware.charted.databases.clickhouse.ClickHouseConnection
import org.noelware.charted.databases.postgres.entities.RepositoryEntity
import org.noelware.charted.databases.postgres.entities.RepositoryMemberEntity
import org.noelware.charted.databases.postgres.entities.UserEntity
import org.noelware.charted.databases.postgres.flags.RepositoryFlags
import org.noelware.charted.databases.postgres.models.Repository
import org.noelware.charted.databases.postgres.models.RepositoryMember
import org.noelware.charted.databases.postgres.models.bitfield
import org.noelware.charted.databases.postgres.tables.RepositoryMemberTable
import org.noelware.charted.databases.postgres.tables.RepositoryTable
import org.noelware.charted.databases.postgres.tables.RepositoryTable.deprecated
import org.noelware.charted.databases.postgres.tables.UserTable
import org.noelware.charted.extensions.regexp.toNameRegex
import org.noelware.charted.modules.avatars.AvatarFetchUtil
import org.noelware.charted.modules.email.EmailService
import org.noelware.charted.modules.helm.charts.HelmChartModule
import org.noelware.charted.modules.storage.StorageHandler
import org.noelware.charted.server.CoroutinesBasedCaffeineCache
import org.noelware.charted.server.createKtorContentWithByteArray
import org.noelware.charted.server.createKtorContentWithInputStream
import org.noelware.charted.server.plugins.PreconditionResult
import org.noelware.charted.server.plugins.SessionsPlugin
import org.noelware.charted.server.plugins.currentUser
import org.noelware.charted.types.helm.RepoType
import org.noelware.charted.types.responses.ApiResponse
import org.noelware.ktor.body
import org.noelware.ktor.endpoints.*
import kotlin.time.Duration.Companion.hours
import kotlin.time.toJavaDuration

@Serializable
data class MainRepositoryResponse(
    val message: String = "Welcome to the Repositories API!",

    @SerialName("docs_url")
    val docsUrl: String = "https://charts.noelware.org/docs/server/${ChartedInfo.version}/api/repositories"
)

@Serializable
data class UpdateRepositoryBody(
    val description: String? = null,
    val deprecated: Boolean? = null,
    val private: Boolean? = null,
    val name: String? = null,
    val type: RepoType? = null
) {
    init {
        if (description != null && description.length > 64) {
            throw StringOverflowException("body.description", 64, description.length)
        }

        if (name != null) {
            if (!name.toNameRegex(true, 24).matches()) {
                throw ValidationException("body.name", "Repository name can only contain letters, digits, dashes, or underscores.")
            }
        }
    }
}

@Serializable
data class InviteRepositoryOrOrganizationMemberBody(@SerialName("member_id") val memberId: Long)

private val repositoryMemberAttribute: AttributeKey<RepositoryMember> = AttributeKey("Repository Member")
private val ApplicationCall.repositoryMember: RepositoryMember?
    get() = attributes.getOrNull(repositoryMemberAttribute)

class RepositoriesEndpoint(
    private val storage: StorageHandler,
    private val config: Config,
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
        // Repository Transfer Endpoints
        // +==============================+
        install(HttpMethod.Post, "/repositories/{id}/transfer", SessionsPlugin) {
            this += "repo:transfer"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

        // +==============================+
        // Repository Webhooks Endpoints
        // +==============================+
        install(HttpMethod.Delete, "/repositories/{id}/webhooks/{webhookId}/events/{eventId}", SessionsPlugin) {
            this += "repo:webhooks:events:delete"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

        install(HttpMethod.Get, "/repositories/{id}/webhooks/{webhookId}/events/{eventId}", SessionsPlugin) {
            this += "repo:webhooks:events:list"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

        install(HttpMethod.Get, "/repositories/{id}/webhooks/{webhookId}/events", SessionsPlugin) {
            this += "repo:webhooks:events:list"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

        install(HttpMethod.Delete, "/repositories/{id}/webhooks/{webhookId}", SessionsPlugin) {
            this += "repo:webhooks:delete"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

        install(HttpMethod.Patch, "/repositories/{id}/webhooks/{webhookId}", SessionsPlugin) {
            this += "repo:webhooks:update"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

        install(HttpMethod.Put, "/repositories/{id}/webhooks", SessionsPlugin) {
            this += "repo:webhooks:create"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

        install(HttpMethod.Get, "/repositories/{id}/webhooks", SessionsPlugin) {
            this += "repo:webhooks:list"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

        // +==============================+
        // Repository Releases Endpoints
        // +==============================+
        install(HttpMethod.Post, "/repositories/{id}/releases/{version}.tar.gz", SessionsPlugin) {
            this += "repo:release:create"

            condition(this@RepositoriesEndpoint::checkRepositoryPermissionOnCurrentUser)
        }

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
        val repository = call.getRepository() ?: return
        call.respond(HttpStatusCode.OK, ApiResponse.ok(Repository.fromEntity(repository)))
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

    /**
     * Patches a [Repository]'s metadata and returns if the patch was a success or not. This will only check
     * if the repository name exists on a user's account since it doesn't know if this is a organization or
     * user repository request.
     *
     * @statusCode 202 If the metadata was patched
     * @statusCode 400 If the ID was not a valid snowflake.
     * @statusCode 403 If the repository is private and the user doesn't have permission to view.
     * @statusCode 404 If the repository was not found in the database, or the owner had deleted their account or was not found.
     * @statusCode 500 If any database errors had occurred.
     */
    @Patch("/{id}")
    suspend fun updateRepo(call: ApplicationCall) {
        val id = call.parameters["id"]?.toLongOrNull()
            ?: return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "INVALID_REPOSITORY_ID",
                    "You will need to specify a Snowflake."
                )
            )

        val patched: UpdateRepositoryBody by call.body()
        val whereClause: SqlExpressionBuilder.() -> Op<Boolean> = { RepositoryTable.id eq id }

        // Do some post checks before patching
        if (patched.name != null) {
            val anyOtherRepo = asyncTransaction(ChartedScope) {
                RepositoryEntity.find { (RepositoryTable.name eq patched.name!!) and (RepositoryTable.owner eq call.currentUser!!.id) }.firstOrNull()
            }

            if (anyOtherRepo != null) {
                throw ValidationException("body.name", "Can't rename repository ${patched.name} since repository already exists on your account")
            }
        }

        asyncTransaction(ChartedScope) {
            RepositoryTable.update(whereClause) {
                it[updatedAt] = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())

                if (patched.description != null) {
                    it[description] = patched.description
                }

                if (patched.deprecated != null) {
                    it[deprecated] = patched.deprecated!!
                }

                if (patched.private != null) {
                    it[flags] = if (patched.private!!) 1 else 0
                }

                if (patched.name != null) {
                    it[name] = patched.name!!
                }

                if (patched.type != null) {
                    it[type] = patched.type!!
                }
            }
        }

        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    /**
     * Deletes a repository from the database and deleting all the releases, webhooks, webhook events, and such.
     */
    @Delete("/{id}")
    suspend fun deleteRepo(call: ApplicationCall) {
        val repository = call.getRepository() ?: return
        asyncTransaction(ChartedScope) {
            RepositoryTable.deleteWhere { RepositoryTable.id eq repository.id }
        }

        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    // +==============================+
    // Repository Icon Endpoints
    // +==============================+
    @Get("/{id}/icons/current.png")
    suspend fun getCurrentIcon(call: ApplicationCall) {
        val repository = call.getRepository() ?: return
        val content = AvatarFetchUtil.retrieveRepositoryIcon(Repository.fromEntity(repository), null)
            ?: return call.respond(HttpStatusCode.NotFound)

        call.respond(createKtorContentWithByteArray(content.second, content.first))
    }

    @Get("/{id}/icons/{icon}")
    suspend fun getRepositoryIcon(call: ApplicationCall) {
        val repository = call.getRepository() ?: return
        val content = AvatarFetchUtil.retrieveRepositoryIcon(Repository.fromEntity(repository), call.parameters["icon"]!!)
            ?: return call.respond(HttpStatusCode.NotFound)

        call.respond(createKtorContentWithByteArray(content.second, content.first))
    }

    @Post("/{id}/icons")
    suspend fun uploadRepositoryIcon(call: ApplicationCall) {
        val repository = call.getRepository() ?: return
        val multipart = call.receiveMultipart()
        val part = multipart.readPart() ?: return call.respond(
            HttpStatusCode.BadRequest,
            ApiResponse.err("EXCESSIVE_MULTIPART_AMOUNT", "There can be only one multipart in this request.")
        )

        if (part !is PartData.FileItem) {
            return call.respond(
                HttpStatusCode.NotAcceptable,
                ApiResponse.err("NOT_FILE_PART", "The multipart object must be a File object.")
            )
        }

        AvatarFetchUtil.updateRepositoryIcon(Repository.fromEntity(repository), part)
        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
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
        val body: InviteRepositoryOrOrganizationMemberBody by call.body()

        // If the email service is not in the Koin module, automatically
        // make them as a member no matter what I guess?
        //
        // This could be changed differently if wished.
        if (emails == null) {
            val repository = call.getRepository() ?: return

            // Check if the given repository member (us) have permission to invite
            // members or not.
            val member = RepositoryMember.fromEntity(repository.members.first { it.account.id.value == call.currentUser!!.id })
            if (!member.bitfield.has("member:invite")) {
                return call.respond(
                    HttpStatusCode.Unauthorized,
                    ApiResponse.err(
                        "INVALID_PERMISSIONS",
                        "You do not have permission to invite member [${body.memberId}]"
                    )
                )
            }

            // Let's get the member's information before we claim that they actually exist
            // or not
            val memberUserObject = asyncTransaction(ChartedScope) {
                UserEntity.find { UserTable.id eq body.memberId }.firstOrNull()
            } ?: return call.respond(
                HttpStatusCode.NotFound,
                ApiResponse.err(
                    "UNKNOWN_USER",
                    "User with ID [${body.memberId}] doesn't exist"
                )
            )

            val repoMember = asyncTransaction(ChartedScope) {
                RepositoryMemberEntity.new(Snowflake.generate()) {
                    this.repository = repository
                    this.account = memberUserObject

                    publicVisibility = false
                    displayName = null
                    permissions = 0
                    updatedAt = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                    joinedAt = Clock.System.now().toLocalDateTime(TimeZone.currentSystemDefault())
                }
            }

            call.respond(HttpStatusCode.Created, ApiResponse.ok(RepositoryMember.fromEntity(repoMember)))
        }

        call.respond(HttpStatusCode.NotImplemented)
    }

    /**
     * Returns all the pending member invites in the given repository.
     *
     * @statusCode 200 All the pending member invites, if any
     * @statusCode 400 If the repository ID was not a valid snowflake
     * @statusCode 401 If the user getting all the invites doesn't have permission to view the invites,
     *                 or view the repository.
     * @statusCode 500 If any database errors had occurred
     * @statusCode 501 If the email service is not available
     */
    @Get("/{id}/members/invites")
    suspend fun getAllMemberInvites(call: ApplicationCall) {
        if (emails == null) return call.respond(HttpStatusCode.NotImplemented)
    }

    /**
     * Approves a member invite from a repository. This is different from the `DELETE /repositories/{id}/members/invites/{inviteId}`
     * since the DELETE method is for denying the request and `POST /repositories/{id}/members/invites/{inviteId}/approve` is for approving
     * the invitation altogether.
     *
     * @statusCode 201 The [RepositoryMember] object that was created.
     * @statusCode 400 If the repository ID was not a valid snowflake
     * @statusCode 404 If the invite had expired
     * @statusCode 500 If any database errors had occurred
     * @statusCode 501 If the email service is not in the Koin module.
     */
    @Post("/{id}/members/invites/{inviteId}/approve")
    suspend fun approveMemberInvite(call: ApplicationCall) {
        if (emails == null) return call.respond(HttpStatusCode.NotImplemented)
    }

    /**
     * Determines if the member invite is still pending, or has expired.
     * @statusCode 200 If the member invite is still pending, it will give you how much time
     *                 and the invite URL to invite them.
     * @statusCode 400 If the repository ID was not a valid snowflake
     * @statusCode 404 If the invite had expired
     * @statusCode 500 If any database errors had occurred
     * @statusCode 501 If the email service is not in the Koin module.
     */
    @Head("/{id}/members/invites/{inviteId}")
    suspend fun checkIfMemberInviteIsAvailable(call: ApplicationCall) {
        if (emails == null) return call.respond(HttpStatusCode.NotImplemented)
    }

    @Delete("/{id}/members/invites/{inviteId}")
    suspend fun deleteMemberInvite(call: ApplicationCall) {
        if (emails == null) return call.respond(HttpStatusCode.NotImplemented)
    }

    @Get("/{id}/members/{memberId}")
    suspend fun getRepoMember(call: ApplicationCall) {
        val memberId = call.parameters["memberId"]?.toLongOrNull() ?: return call.respond(
            HttpStatusCode.BadRequest,
            ApiResponse.err(
                "INVALID_REPOSITORY_MEMBER_ID",
                "Provided repository member ID was not a valid snowflake"
            )
        )

        val repository = call.getRepository() ?: return
        val member = asyncTransaction(ChartedScope) {
            RepositoryMemberEntity.find { (RepositoryMemberTable.repository eq repository.id) and (RepositoryMemberTable.id eq memberId) }.firstOrNull()?.let { entity ->
                RepositoryMember.fromEntity(entity)
            }
        } ?: return call.respond(
            HttpStatusCode.NotFound,
            ApiResponse.err(
                "REPO_MEMBER_NOT_FOUND",
                "Repository member with ID [$memberId] was not found"
            )
        )

        call.respond(HttpStatusCode.OK, ApiResponse.ok(member))
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
    @Get("/{id}/releases/{version}.tar.gz")
    suspend fun getRelease(call: ApplicationCall) {
        if (config.features.contains(ServerFeature.DOCKER_REGISTRY)) {
            return call.respond(HttpStatusCode.NotFound)
        }

        val version = call.parameters["version"]!!
        try {
            version.toVersion(true)
        } catch (e: VersionFormatException) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "INVALID_SEMVER",
                    "Version [$version] was not a valid SemVer v2 version"
                )
            )
        }

        val repository = call.getRepository() ?: return
        val stream = charts!!.getReleaseTarball(repository.owner, repository.id.value, version) ?: return call.respond(HttpStatusCode.NotFound)
        call.respond(createKtorContentWithInputStream(stream, ContentType.parse("application/tar+gzip")))
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

    @Post("/{id}/releases/{version}.tar.gz")
    suspend fun uploadReleaseTarball(call: ApplicationCall) {
        if (config.features.contains(ServerFeature.DOCKER_REGISTRY)) {
            return call.respond(HttpStatusCode.NotFound)
        }

        val repository = call.getRepository() ?: return
        if (call.repositoryMember != null) {
            if (call.repositoryMember!!.id != repository.owner || !call.repositoryMember!!.bitfield.has("repo:update")) {
                return call.respond(
                    HttpStatusCode.Forbidden,
                    ApiResponse.err(
                        "MISSING_PERMISSIONS",
                        "You are missing the permission [repo:update] to execute this request"
                    )
                )
            }
        }

        val multipart = call.receiveMultipart()
        val part = multipart.readPart() ?: return call.respond(
            HttpStatusCode.BadRequest,
            ApiResponse.err("EXCESSIVE_MULTIPART_AMOUNT", "There can be only one multipart in this request.")
        )

        if (part !is PartData.FileItem) {
            return call.respond(
                HttpStatusCode.NotAcceptable,
                ApiResponse.err("NOT_FILE_PART", "The multipart object must be a File object.")
            )
        }

        val version = call.parameters["version"]!!
        try {
            version.toVersion(true)
        } catch (e: VersionFormatException) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "INVALID_SEMVER",
                    "Version [$version] was not a valid SemVer v2 version"
                )
            )
        }

        charts!!.uploadReleaseTarball(repository.owner, Repository.fromEntity(repository), version, part)
        call.respond(HttpStatusCode.Accepted, ApiResponse.ok())
    }

    /**
     * Returns the given `Chart.yaml` of this repository's release.
     * @statusCode 200 The release by the version if it was found
     * @statusCode 400 If the ID was not a valid snowflake.
     * @statusCode 403 If the repository is private and the user doesn't have permission to view the repository
     * @statusCode 404 If the repository OR release was not found in the database.
     */
    @Get("/{id}/releases/{version}/Chart.yaml")
    suspend fun getChartYamlFromRelease(call: ApplicationCall) {
        if (config.features.contains(ServerFeature.DOCKER_REGISTRY)) {
            return call.respond(HttpStatusCode.NotFound)
        }

        val version = call.parameters["version"]!!
        try {
            version.toVersion(true)
        } catch (e: VersionFormatException) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "INVALID_SEMVER",
                    "Version [$version] was not a valid SemVer v2 version"
                )
            )
        }

        val repository = call.getRepository() ?: return
        val stream = charts!!.getChartYaml(repository.owner, repository.id.value, version) ?: return call.respond(HttpStatusCode.NotFound)

        call.respond(createKtorContentWithInputStream(stream, ContentType.parse("text/yaml; charset=utf-8")))
    }

    @Get("/{id}/releases/{version}/values.yaml")
    suspend fun getValuesYamlFromRelease(call: ApplicationCall) {
        if (config.features.contains(ServerFeature.DOCKER_REGISTRY)) {
            return call.respond(HttpStatusCode.NotFound)
        }

        val version = call.parameters["version"]!!
        try {
            version.toVersion(true)
        } catch (e: VersionFormatException) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "INVALID_SEMVER",
                    "Version [$version] was not a valid SemVer v2 version"
                )
            )
        }

        val repository = call.getRepository() ?: return
        val stream = charts!!.getValuesYaml(repository.owner, repository.id.value, version) ?: return call.respond(HttpStatusCode.NotFound)

        call.respond(createKtorContentWithInputStream(stream, ContentType.parse("text/yaml; charset=utf-8")))
    }

    @Get("/{id}/releases/{version}/templates/{template}")
    suspend fun getTemplateFromRelease(call: ApplicationCall) {
        if (config.features.contains(ServerFeature.DOCKER_REGISTRY)) {
            return call.respond(HttpStatusCode.NotFound)
        }

        val version = call.parameters["version"]!!
        try {
            version.toVersion(true)
        } catch (e: VersionFormatException) {
            return call.respond(
                HttpStatusCode.BadRequest,
                ApiResponse.err(
                    "INVALID_SEMVER",
                    "Version [$version] was not a valid SemVer v2 version"
                )
            )
        }

        val repository = call.getRepository() ?: return
        val stream = charts!!.getTemplate(repository.owner, repository.id.value, version, call.parameters["template"]!!) ?: return call.respond(HttpStatusCode.NotFound)

        call.respond(createKtorContentWithInputStream(stream, ContentType.parse("text/plain; charset=utf-8")))
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

    @Get("/{id}/webhooks/{webhookId}/events")
    suspend fun getRepoWebhookEvents(call: ApplicationCall) {
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
    // Repository Transfer Endpoints
    // +==============================+

    @Post("/{id}/transfer/{newOwnerId}")
    suspend fun transferTo(call: ApplicationCall) {
        call.respond(HttpStatusCode.NotImplemented)
    }

    // +==============================+
    // Private helper functions
    // +==============================+

    private suspend fun checkRepositoryPermissionOnCurrentUser(call: ApplicationCall): PreconditionResult {
        val repository = call.getRepository() ?: return PreconditionResult.Failed()
        if (call.currentUser != null && repository.owner == call.currentUser!!.id) return PreconditionResult.Success

        // Check if the repository is private and the user is a member
        // of that repository.
        val bitfield = RepositoryFlags(repository.flags)
        if (bitfield.has("PRIVATE")) {
            // Check if the user is a member of that organization
            val member = repository.members.firstOrNull { it.account.id.value == call.currentUser!!.id }
            if (member == null) {
                call.respond(
                    HttpStatusCode.Forbidden,
                    ApiResponse.err(
                        "INVALID_REPOSITORY_ACCESS",
                        "You don't have access to view this repository"
                    )
                )

                return PreconditionResult.Failed()
            }

            call.attributes.put(repositoryMemberAttribute, RepositoryMember.fromEntity(member))
        }

        return PreconditionResult.Success
    }

    private suspend fun ApplicationCall.getRepository(): RepositoryEntity? {
        val id = parameters["id"]?.toLongOrNull()
            ?: return run {
                respond(
                    HttpStatusCode.BadRequest,
                    ApiResponse.err(
                        "INVALID_REPOSITORY_ID",
                        "Provided repository ID was not a valid snowflake"
                    )
                )

                null
            }

        return asyncTransaction(ChartedScope) {
            RepositoryEntity.find { RepositoryTable.id eq id }.firstOrNull()
        } ?: return run {
            respond(
                HttpStatusCode.NotFound,
                ApiResponse.err(
                    "UNKNOWN_REPOSITORY",
                    "Repository with ID [$id] was not found."
                )
            )

            null
        }
    }
}
