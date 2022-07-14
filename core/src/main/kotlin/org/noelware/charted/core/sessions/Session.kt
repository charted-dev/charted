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

package org.noelware.charted.core.sessions

import kotlinx.serialization.SerialName
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.buildJsonObject
import kotlinx.serialization.json.put
import org.noelware.charted.core.serializers.UUIDSerializer
import org.noelware.charted.database.models.User
import java.util.UUID

@kotlinx.serialization.Serializable
data class Session(
    @SerialName("user_id")
    val userID: Long,

    @kotlinx.serialization.Serializable(with = UUIDSerializer::class)
    @SerialName("session_id")
    val sessionID: UUID,

    @SerialName("refresh_token")
    val refreshToken: String,

    @SerialName("access_token")
    val accessToken: String
) {
    fun toJsonObject(user: User? = null): JsonObject = buildJsonObject {
        put("refresh_token", refreshToken)
        put("access_token", accessToken)
        put("session_id", sessionID.toString())

        if (user != null) {
            put("user", user.toJsonObject())
        }
    }
}
