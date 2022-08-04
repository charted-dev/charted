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

package org.noelware.charted.database.models

import kotlinx.datetime.LocalDateTime
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import org.noelware.charted.database.entities.OrganizationMemberEntity

@kotlinx.serialization.Serializable
data class OrganizationMember(
    @SerialName("display_name")
    val displayName: String? = null,

    @SerialName("updated_at")
    val updatedAt: LocalDateTime,

    @SerialName("joined_at")
    val joinedAt: LocalDateTime,
    val user: User,
    val id: String
) {
    companion object {
        fun fromEntity(entity: OrganizationMemberEntity): OrganizationMember = OrganizationMember(
            entity.displayName,
            entity.updatedAt,
            entity.joinedAt,
            User.fromEntity(entity.account),
            entity.id.value.toString()
        )
    }

    fun toJsonObject(): JsonObject = buildJsonObject {
        put("display_name", displayName)
        put("joined_at", joinedAt.toString())
        put("updated_at", updatedAt.toString())
        put("user", user.toJsonObject())
        put("id", id)
    }
}
