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

import dev.floofy.utils.exposed.asyncTransaction
import dev.floofy.utils.kotlin.ifNotNull
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.response.*
import kotlinx.datetime.LocalDateTime
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toInstant
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.*
import org.jetbrains.exposed.sql.ResultRow
import org.jetbrains.exposed.sql.select
import org.noelware.charted.core.ChartedScope
import org.noelware.charted.core.StorageWrapper
import org.noelware.charted.core.config.Config
import org.noelware.charted.core.sessions.SessionKey
import org.noelware.charted.database.tables.Organizations
import org.noelware.charted.database.tables.Users
import org.noelware.ktor.body
import org.noelware.ktor.endpoints.AbstractEndpoint
import org.noelware.ktor.endpoints.Get
import org.noelware.ktor.endpoints.Post

@kotlinx.serialization.Serializable
data class Organization(
    @SerialName("verified_publisher")
    val verifiedPublisher: Boolean = false,

    @SerialName("twitter_handle")
    val twitterHandle: String? = null,
    val description: String? = null,

    @SerialName("display_name")
    val displayName: String? = null,

    @SerialName("created_at")
    val createdAt: LocalDateTime,

    @SerialName("updated_at")
    val updatedAt: LocalDateTime,
    val handle: String,
    val owner: User? = null,
    val icon: String?,
) {
    companion object {
        suspend fun fromResultRow(row: ResultRow, loadOwner: Boolean = false): Organization {
            val owner = if (loadOwner) {
                asyncTransaction(ChartedScope) {
                    val obj = Users.select {
                        Users.id eq row[Organizations.owner].value
                    }.firstOrNull()

                    obj.ifNotNull { User.fromResultRow(it) }
                }
            } else {
                null
            }

            return Organization(
                row[Organizations.verifiedPublisher],
                row[Organizations.twitterHandle],
                row[Organizations.description],
                row[Organizations.displayName],
                row[Organizations.createdAt],
                row[Organizations.updatedAt],
                row[Organizations.handle],
                owner,
                row[Organizations.avatar]
            )
        }
    }

    fun toJsonObject(): JsonObject = buildJsonObject {
        put("verified_publisher", verifiedPublisher)
        put("twitter_handle", twitterHandle)
        put("description", description)
        put("display_name", displayName)
        put("created_at", createdAt.toInstant(TimeZone.currentSystemDefault()).toString())
        put("updated_at", updatedAt.toInstant(TimeZone.currentSystemDefault()).toString())
        put("handle", handle)
        put("owner", owner?.toJsonObject() ?: JsonNull)
        put("icon", icon)
    }
}

@kotlinx.serialization.Serializable
data class CreateOrgPayload(
    val handle: String,
    val description: String? = null
)

class OrganizationsApiEndpoints(
    private val storage: StorageWrapper,
    private val config: Config
): AbstractEndpoint("/organizations") {
    @Get
    suspend fun main(call: ApplicationCall) {
        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                put(
                    "data",
                    buildJsonObject {
                        put("message", "Welcome to the Organizations API!")
                        put("docs", "https://charts.noelware.org/docs/api/organizations")
                    }
                )
            }
        )
    }

    @Get("/{id}")
    suspend fun get(call: ApplicationCall) {
        val id = call.parameters["id"]!!
        val referenceOwner = call.request.queryParameters["with_owner"]

        if (id.toLongOrNull() == null) {
            call.respond(
                HttpStatusCode.BadRequest,
                buildJsonObject {
                    put("success", false)
                    put(
                        "errors",
                        buildJsonArray {
                            add(
                                buildJsonObject {
                                    put("code", "CANNOT_CONVERT_TO_LONG")
                                    put("message", "Cannot convert ID to long.")
                                }
                            )
                        }
                    )
                }
            )

            return
        }

        val organization = asyncTransaction(ChartedScope) {
            Organizations.select { Organizations.id eq id.toLong() }.firstOrNull()
        }

        if (organization == null) {
            call.respond(
                HttpStatusCode.NotFound,
                buildJsonObject {
                    put("success", false)
                    put(
                        "errors",
                        buildJsonArray {
                            add(
                                buildJsonObject {
                                    put("code", "UNKNOWN_ORGANIZATION")
                                    put("message", "Organization with ID $id doesn't exist.")
                                }
                            )
                        }
                    )
                }
            )

            return
        }

        call.respond(
            HttpStatusCode.OK,
            buildJsonObject {
                put("success", true)
                put("data", Organization.fromResultRow(organization, referenceOwner != null).toJsonObject())
            }
        )
    }

    @Post
    suspend fun createOrg(call: ApplicationCall) {
        val body by call.body<CreateOrgPayload>()
        val session = call.attributes[SessionKey]
    }
}
