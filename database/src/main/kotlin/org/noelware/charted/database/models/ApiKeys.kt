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
import kotlinx.serialization.json.*
import org.noelware.charted.database.entities.ApiKeyEntity
import org.noelware.charted.database.flags.ApiKeyScopeFlags

@kotlinx.serialization.Serializable
data class ApiKeys(
    @SerialName("expires_in")
    val expiresIn: LocalDateTime? = null,
    val scopes: String = "0",
    val token: String? = null,
    val user: User,
    val name: String,
    val id: String
) {
    companion object {
        fun fromEntity(entity: ApiKeyEntity, raw: String? = null): ApiKeys = ApiKeys(
            entity.expiresIn,
            entity.scopes.toString(16),
            raw,
            User.fromEntity(entity.owner),
            entity.name,
            entity.id.value.toString()
        )
    }

    fun toJsonObject(): JsonObject = buildJsonObject {
        put("expires_in", expiresIn?.let { JsonPrimitive(it.toInstant(TimeZone.currentSystemDefault()).toString()) } ?: JsonNull)
        put("scopes", scopes)
        if (token != null) {
            put("token", token)
        }

        put("user", user.toJsonObject())
        put("name", name)
        put("id", id)
    }
}

val ApiKeys.bitfield: ApiKeyScopeFlags
    get() = ApiKeyScopeFlags(scopes.toLong())
