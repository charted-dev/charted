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

package org.noelware.charted.database.models

import kotlinx.datetime.LocalDateTime
import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import org.noelware.charted.common.data.helm.RepoType
import org.noelware.charted.common.data.helm.key
import org.noelware.charted.database.entities.RepositoryEntity
import org.noelware.charted.database.flags.RepositoryFlags

@kotlinx.serialization.Serializable
data class Repository(
    val description: String? = null,
    val deprecated: Boolean = false,

    @SerialName("created_at")
    val createdAt: LocalDateTime,

    @SerialName("updated_at")
    val updatedAt: LocalDateTime,

    @SerialName("icon_hash")
    val iconHash: String? = null,

    @SerialName("owner_id")
    val ownerID: Long,
    val flags: Long = 0L,
    val name: String,
    val type: RepoType,
    val id: Long
) {
    companion object {
        fun fromEntity(entity: RepositoryEntity): Repository = Repository(
            entity.description,
            entity.deprecated,
            entity.createdAt,
            entity.updatedAt,
            entity.iconHash,
            entity.owner,
            entity.flags,
            entity.name,
            entity.type,
            entity.id.value
        )
    }

    fun toJsonObject(): JsonObject = buildJsonObject {
        put("description", description)
        put("deprecated", deprecated)
        put("created_at", "$createdAt")
        put("updated_at", "$updatedAt")
        put("icon_hash", iconHash)
        put("owner_id", ownerID)
        put("flags", 0)
        put("name", name)
        put("type", type.key)
        put("id", id)
    }
}

val Repository.bitfield: RepositoryFlags
    get() = RepositoryFlags(flags)
