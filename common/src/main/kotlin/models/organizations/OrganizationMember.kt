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
import kotlinx.datetime.LocalDateTime
import kotlinx.serialization.SerialName
import org.noelware.charted.models.flags.MemberPermissions
import org.noelware.charted.models.users.User

public val OrganizationMember.permissions: MemberPermissions get() = MemberPermissions(permissionBits)

@kotlinx.serialization.Serializable
public data class OrganizationMember(
    @JsonProperty("display_name")
    @SerialName("display_name")
    val displayName: String? = null,

    @JsonProperty("permissions")
    @SerialName("permissions")
    val permissionBits: Long = 0,

    @JsonProperty("updated_at")
    @SerialName("updated_at")
    val updatedAt: LocalDateTime,

    @JsonProperty("joined_at")
    @SerialName("joined_at")
    val joinedAt: LocalDateTime,
    val user: User,
    val id: Long
)
