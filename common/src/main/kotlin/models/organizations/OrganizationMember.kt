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

package org.noelware.charted.models.organizations

import com.fasterxml.jackson.annotation.JsonProperty
import io.swagger.v3.oas.annotations.media.Schema
import kotlinx.datetime.LocalDateTime
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.common.Bitfield
import org.noelware.charted.models.flags.MemberPermissions
import org.noelware.charted.models.users.User
import org.noelware.charted.snowflake.Snowflake

/**
 * Returns a [MemberPermissions] flag bitfield for this organization member.
 */
public val OrganizationMember.permissions: MemberPermissions get() = MemberPermissions(permissionBits)

/**
 * Represents an Organization Member resource. A member that is apart an organization
 * can control the given organization resource if given permission towards, this can
 * be anything by editing organization metadata or editing any organization repositories.
 *
 * In a future release, members can be selected on which repositories can be edited,
 * and they will be internally added as a [repository member][org.noelware.charted.models.repositories.RepositoryMember].
 */
@Schema(
    description = "Represents an Organization Member resource. A member that is apart an organization" +
        " can control the given organization resource if given permission towards, this can" +
        " be anything by editing organization metadata or editing any organization repositories." +
        "\n" +
        "In a future release, members can be selected on which repositories can be edited," +
        " and they will be internally added as a repository member",
)
@Serializable
public data class OrganizationMember(
    /**
     * Display name for this member. It should be formatted as '{[displayName]} (@{[user.username][User.username]})' if this is set, or
     * just @[user.username][User.username]} if none was set.
     */
    @get:Schema(description = "Display name for this member. It should be formatted as '{display_name} (@{user.username})' if this is set, or just @{user.username} if none was set.")
    @JsonProperty("display_name")
    @SerialName("display_name")
    val displayName: String? = null,

    /**
     * Bitfield value of the organization member's permissions.
     */
    @get:Schema(description = "Bitfield value of the organization member's permissions.", implementation = Bitfield::class)
    @JsonProperty("permissions")
    @SerialName("permissions")
    val permissionBits: Long = 0,

    /**
     * Date-time of when this organization member resource was last updated
     * by the API server.
     */
    @get:Schema(description = "Date-time of when this organization member resource was last updated by the API server.")
    @JsonProperty("updated_at")
    @SerialName("updated_at")
    val updatedAt: LocalDateTime,

    /**
     * Date-time of when this member has joined the organization.
     */
    @get:Schema(description = "Date-time of when this member has joined the organization.")
    @JsonProperty("joined_at")
    @SerialName("joined_at")
    val joinedAt: LocalDateTime,
    val user: User,

    /**
     * Unique identifier to locate this organization member with the API
     */
    @get:Schema(implementation = Snowflake::class, description = "Unique identifier to locate this organization member with the API")
    val id: Long
)
