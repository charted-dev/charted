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
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toInstant
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import org.noelware.charted.database.entities.UserEntity
import org.noelware.charted.database.flags.UserFlags

@kotlinx.serialization.Serializable
data class User(
    @SerialName("gravatar_email")
    val gravatarEmail: String? = null,
    val description: String? = null,

    @SerialName("avatar_hash")
    val avatarHash: String? = null,

    @SerialName("created_at")
    val createdAt: LocalDateTime,

    @SerialName("updated_at")
    val updatedAt: LocalDateTime,
    val username: String,
    val flags: Long = 0L,
    val name: String? = null,
    val id: String
) {
    companion object {
        fun fromEntity(entity: UserEntity): User = User(
            entity.gravatarEmail,
            entity.description,
            entity.avatarHash,
            entity.createdAt,
            entity.updatedAt,
            entity.username,
            entity.flags,
            entity.name,
            entity.id.value.toString()
        )
    }

    fun toJsonObject(): JsonObject = buildJsonObject {
        put("gravatar_email", gravatarEmail)
        put("description", description)
        put("avatar_hash", avatarHash)
        put("created_at", createdAt.toInstant(TimeZone.currentSystemDefault()).toString())
        put("updated_at", updatedAt.toInstant(TimeZone.currentSystemDefault()).toString())
        put("username", username)
        put("flags", flags)
        put("name", name)
        put("id", id)
    }
}

val User.bitfield: UserFlags
    get() = UserFlags(flags)
