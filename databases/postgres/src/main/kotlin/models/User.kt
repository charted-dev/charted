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

package org.noelware.charted.databases.postgres.models

import kotlinx.datetime.LocalDateTime
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import org.noelware.charted.databases.postgres.entities.UserEntity

@Serializable
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
    val name: String? = null,
    val id: Long
) {
    companion object {
        fun fromEntity(entity: UserEntity): User = User(
            entity.gravatarEmail,
            entity.description,
            entity.avatarHash,
            entity.createdAt,
            entity.updatedAt,
            entity.username,
            entity.name,
            entity.id.value
        )
    }
}
