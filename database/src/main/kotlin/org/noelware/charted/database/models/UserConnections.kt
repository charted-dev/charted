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
import org.noelware.charted.database.entities.UserConnectionEntity

@kotlinx.serialization.Serializable
data class UserConnections(
    @SerialName("noelware_account_id")
    val noelwareAccountID: Long? = null,

    @SerialName("google_account_id")
    val googleAccountID: String? = null,

    @SerialName("github_account_id")
    val githubAccountID: String? = null,

    @SerialName("apple_account_id")
    val appleAccountID: String? = null,

    @SerialName("created_at")
    val createdAt: LocalDateTime,

    @SerialName("updated_at")
    val updatedAt: LocalDateTime,
    val id: String
) {
    companion object {
        fun fromEntity(entity: UserConnectionEntity): UserConnections = UserConnections(
            entity.noelwareAccountID,
            entity.googleAccountID,
            entity.githubAccountID,
            entity.appleAccountID,
            entity.createdAt,
            entity.updatedAt,
            entity.id.value.toString()
        )
    }

    fun toJsonObject(): JsonObject = buildJsonObject {
        put("noelware_account_id", noelwareAccountID)
        put("google_account_id", googleAccountID)
        put("github_account_id", githubAccountID)
        put("apple_account_id", appleAccountID)
        put("created_at", createdAt.toInstant(TimeZone.currentSystemDefault()).toString())
        put("updated_at", updatedAt.toInstant(TimeZone.currentSystemDefault()).toString())
        put("id", id)
    }
}
