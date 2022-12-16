/*
 * 📦 charted-server: Free, open source, and reliable Helm Chart registry made in Kotlin.
 * Copyright 2022-2023 Noelware <team@noelware.org>
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
import org.noelware.charted.databases.postgres.entities.ApiKeyEntity
import org.noelware.charted.databases.postgres.flags.ApiKeyScopes

@Serializable
data class ApiKeys(
    val description: String? = null,

    @SerialName("expires_in")
    val expiresIn: LocalDateTime? = null,
    val scopes: Long = 0,
    val token: String? = null,
    val owner: User,
    val name: String,
    val id: Long
) {
    companion object {
        fun fromEntity(entity: ApiKeyEntity, showToken: Boolean = false, token: String? = null): ApiKeys = ApiKeys(
            entity.description,
            entity.expiresIn,
            entity.scopes,
            if (showToken) token else null,
            User.fromEntity(entity.owner),
            entity.name,
            entity.id.value
        )
    }
}

val ApiKeys.bitfield: ApiKeyScopes
    get() = ApiKeyScopes(scopes)
